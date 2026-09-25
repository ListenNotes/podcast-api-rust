mod support;

use podcast_api::{Client, Error};
use reqwest::Url;
use serde_json::{Value, json};
use std::{collections::BTreeMap, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
};

#[derive(Debug)]
struct Request {
    method: String,
    target: String,
    headers: BTreeMap<String, String>,
    body: String,
}

struct Fixture {
    base: String,
    task: Option<JoinHandle<Vec<Request>>>,
}

impl Fixture {
    async fn new(replies: Vec<(u16, String, String)>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}/api/v2", listener.local_addr().unwrap());
        let task = tokio::spawn(async move {
            let mut requests = Vec::new();
            for (status, headers, body) in replies {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut bytes = Vec::new();
                let (end, length) = loop {
                    let mut buf = [0; 4096];
                    let count = stream.read(&mut buf).await.unwrap();
                    assert!(count > 0, "incomplete request headers");
                    bytes.extend_from_slice(&buf[..count]);
                    assert!(bytes.len() < 65536, "request too large");
                    if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let text = String::from_utf8_lossy(&bytes[..end]);
                        let length = text
                            .lines()
                            .find_map(|line| {
                                line.split_once(':')
                                    .filter(|(k, _)| k.eq_ignore_ascii_case("content-length"))
                                    .map(|(_, v)| v.trim().parse::<usize>().unwrap())
                            })
                            .unwrap_or(0);
                        break (end + 4, length);
                    }
                };
                while bytes.len() < end + length {
                    let mut buf = [0; 4096];
                    let count = stream.read(&mut buf).await.unwrap();
                    assert!(count > 0);
                    bytes.extend_from_slice(&buf[..count]);
                }
                let text = String::from_utf8(bytes[..end].to_vec()).unwrap();
                let mut lines = text.lines();
                let mut start = lines.next().unwrap().split_whitespace();
                requests.push(Request {
                    method: start.next().unwrap().into(),
                    target: start.next().unwrap().into(),
                    headers: lines
                        .filter_map(|line| line.split_once(':'))
                        .map(|(k, v)| (k.to_ascii_lowercase(), v.trim().into()))
                        .collect(),
                    body: String::from_utf8(bytes[end..end + length].to_vec()).unwrap(),
                });
                let response = format!(
                    "HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n{body}",
                    body.len()
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }
            requests
        });
        Self { base, task: Some(task) }
    }

    fn client<'a>(&self, key: Option<&'a str>) -> Client<'a> {
        Client::new_custom(Client::http_client_builder().no_proxy().build().unwrap(), key, None)
            .with_base_url(&self.base)
            .unwrap()
    }

    async fn finish(mut self) -> Vec<Request> {
        tokio::time::timeout(Duration::from_secs(5), self.task.as_mut().unwrap())
            .await
            .unwrap()
            .as_ref()
            .unwrap()
            .iter()
            .map(|r| Request {
                method: r.method.clone(),
                target: r.target.clone(),
                headers: r.headers.clone(),
                body: r.body.clone(),
            })
            .collect()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

fn reply(status: u16) -> (u16, String, String) {
    (status, "X-ListenAPI-Usage: 12\r\n".into(), "{\"ok\":true}".into())
}

fn pairs(value: &str) -> BTreeMap<String, String> {
    Url::parse(&format!("http://fixture/?{value}"))
        .unwrap()
        .query_pairs()
        .into_owned()
        .collect()
}

#[tokio::test]
async fn every_generated_method_obeys_the_contract() {
    let contract = support::contract();
    let operations = contract["operations"].as_array().unwrap();
    assert_eq!(operations.len(), 31);
    let fixture = Fixture::new(
        operations
            .iter()
            .map(|op| reply(if op["method"] == "POST" { 201 } else { 200 }))
            .collect(),
    )
    .await;
    let client = fixture.client(Some("test-key"));
    for op in operations {
        let response = support::methods::call(&client, op["operationId"].as_str().unwrap(), &op["example_params"])
            .await
            .unwrap();
        assert_eq!(response.response.headers()["x-listenapi-usage"], "12");
        assert!(!format!("{:?}", response.request).contains("test-key"));
        assert_eq!(response.json().await.unwrap(), json!({"ok": true}));
    }
    let requests = fixture.finish().await;
    for (op, request) in operations.iter().zip(requests) {
        let mut path = op["path"].as_str().unwrap().to_owned();
        let mut query = BTreeMap::new();
        let mut body = BTreeMap::new();
        for param in op["parameters"].as_array().unwrap() {
            let name = param["name"].as_str().unwrap();
            let value = &op["example_params"][name];
            if value.is_null() {
                continue;
            }
            let scalar = support::methods::scalar(value);
            match param["in"].as_str().unwrap() {
                "path" => path = path.replace(&format!("{{{name}}}"), &scalar),
                "query" => {
                    query.insert(name.to_owned(), scalar);
                }
                "body" => {
                    body.insert(name.to_owned(), scalar);
                }
                other => panic!("unexpected location {other}"),
            }
        }
        let url = Url::parse(&format!("http://fixture{}", request.target)).unwrap();
        assert_eq!(request.method, op["method"]);
        assert_eq!(url.path(), format!("/api/v2{path}"));
        assert_eq!(url.query_pairs().into_owned().collect::<BTreeMap<_, _>>(), query);
        assert_eq!(pairs(&request.body), body);
        assert_eq!(request.headers["x-listenapi-key"], "test-key");
        assert_eq!(
            request.headers["user-agent"],
            concat!("podcast-api-rust ", env!("CARGO_PKG_VERSION"))
        );
    }
}

#[tokio::test]
async fn encodes_nested_paths_and_preserves_empty_and_scalar_values() {
    let fixture = Fixture::new(vec![reply(200), reply(201), reply(200)]).await;
    let client = fixture.client(None);
    client
        .update_playlist_item_notes(
            "a/b?#%é",
            "2/3",
            &json!({"notes":"", "id":"must-not-leak", "item_id":99}),
        )
        .await
        .unwrap();
    client
        .create_playlist(&json!({"name":"a & b=c+é", "description":"", "zero":0, "boolean":false, "skip":null}))
        .await
        .unwrap();
    client
        .search(&json!({"q":"c++ & café", "offset":0, "safe_mode":false, "skip":null}))
        .await
        .unwrap();
    let requests = fixture.finish().await;
    assert_eq!(requests[0].target, "/api/v2/playlists/a%2Fb%3F%23%25%C3%A9/items/2%2F3");
    assert_eq!(requests[0].body, "notes=");
    assert_eq!(requests[0].headers["content-type"], "application/x-www-form-urlencoded");
    assert_eq!(
        pairs(&requests[1].body),
        BTreeMap::from([
            ("name".into(), "a & b=c+é".into()),
            ("description".into(), "".into()),
            ("zero".into(), "0".into()),
            ("boolean".into(), "false".into()),
        ])
    );
    assert!(requests.iter().all(|r| !r.headers.contains_key("x-listenapi-key")));
    let query = requests[2].target.split_once('?').unwrap().1;
    assert_eq!(pairs(query)["q"], "c++ & café");
    assert_eq!(pairs(query)["safe_mode"], "false");
    assert!(!pairs(query).contains_key("skip"));
}

#[tokio::test]
async fn delete_playlist_encodes_id_without_query_or_body() {
    let id = "a/b?#%é";
    let payload = json!({"id": id, "deleted": true});
    let fixture = Fixture::new(vec![(200, "X-ListenAPI-Usage: 12\r\n".into(), payload.to_string())]).await;
    let response = fixture
        .client(None)
        .delete_playlist(id, &json!({"id": "must-not-leak", "skip": null}))
        .await
        .unwrap();
    assert_eq!(response.response.status().as_u16(), 200);
    assert_eq!(response.response.headers()["x-listenapi-usage"], "12");
    assert_eq!(response.request.method(), reqwest::Method::DELETE);
    assert_eq!(response.request.url().query(), None);
    assert!(response.request.body().is_none());
    assert_eq!(response.json().await.unwrap(), payload);
    let requests = fixture.finish().await;
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].target, "/api/v2/playlists/a%2Fb%3F%23%25%C3%A9");
    assert!(requests[0].body.is_empty());
    assert!(!requests[0].headers.contains_key("content-type"));
}

#[tokio::test]
async fn client_keys_user_agents_and_request_methods_are_isolated() {
    let fixture = Fixture::new(vec![reply(200), reply(200), reply(200), reply(200), reply(200)]).await;
    let first = fixture.client(Some("first"));
    let second = Client::new_custom(
        Client::http_client_builder().no_proxy().build().unwrap(),
        Some("second"),
        Some("custom-agent"),
    )
    .with_base_url(&fixture.base)
    .unwrap();
    first.update_playlist("list", &json!({"description":""})).await.unwrap();
    second.fetch_my_playlists(&json!({})).await.unwrap();
    first.delete_playlist_item("list", "7", &json!({})).await.unwrap();
    first.delete_playlist("list", &json!({})).await.unwrap();
    first.search(&json!({"q":"hello"})).await.unwrap();
    let requests = fixture.finish().await;
    assert_eq!(
        requests.iter().map(|r| r.method.as_str()).collect::<Vec<_>>(),
        ["PUT", "GET", "DELETE", "DELETE", "GET"]
    );
    assert_eq!(
        requests
            .iter()
            .map(|r| r.headers["x-listenapi-key"].as_str())
            .collect::<Vec<_>>(),
        ["first", "second", "first", "first", "first"]
    );
    assert_eq!(requests[1].headers["user-agent"], "custom-agent");
    assert!(requests[1..].iter().all(|r| r.body.is_empty()));
}

#[tokio::test]
async fn errors_preserve_response_details_and_are_not_retried_or_redirected() {
    let statuses = [301, 302, 307, 400, 401, 403, 404, 422, 429, 500, 503];
    let fixture = Fixture::new(
        statuses
            .iter()
            .flat_map(|status| [*status; 2])
            .map(|status| {
                (
                    status,
                    "Location: http://127.0.0.1:1/never\r\nX-ListenAPI-Usage: 13\r\n".into(),
                    "{\"error\":\"precise reason\"}".into(),
                )
            })
            .collect(),
    )
    .await;
    let client = fixture.client(None);
    for status in statuses {
        for method in ["create_playlist", "delete_playlist"] {
            let error = match method {
                "create_playlist" => client.create_playlist(&json!({"name":"test"})).await.unwrap_err(),
                _ => client.delete_playlist("list", &json!({})).await.unwrap_err(),
            };
            let context = error.api_error().unwrap();
            assert_eq!(context.status.as_u16(), status);
            assert_eq!(context.headers["x-listenapi-usage"], "13");
            assert!(context.body.contains("precise reason"));
            assert!(error.to_string().contains("precise reason"));
            match status {
                400 => assert!(matches!(error, Error::InvalidRequestError(_))),
                401 => assert!(matches!(error, Error::AuthenticationError(_))),
                403 => assert!(matches!(error, Error::PermissionDeniedError(_))),
                404 => assert!(matches!(error, Error::NotFoundError(_))),
                429 => assert!(matches!(error, Error::RateLimitError(_))),
                _ => assert!(matches!(error, Error::ListenApiError(_))),
            }
        }
    }
    assert_eq!(fixture.finish().await.len(), statuses.len() * 2);
}

#[tokio::test]
async fn local_validation_never_sends_invalid_requests() {
    let client = Client::new(None).with_base_url("http://127.0.0.1:1/api/v2").unwrap();
    for params in [Value::Null, json!([]), json!("not an object")] {
        assert!(matches!(client.search(&params).await, Err(Error::InvalidParameter(_))));
        assert!(matches!(
            client.delete_playlist("list", &params).await,
            Err(Error::InvalidParameter(_))
        ));
    }
    for id in ["", ".", ".."] {
        assert!(matches!(
            client.fetch_podcast_by_id(id, &json!({})).await,
            Err(Error::InvalidParameter(_))
        ));
        assert!(matches!(
            client.delete_playlist(id, &json!({})).await,
            Err(Error::InvalidParameter(_))
        ));
    }
    for base in [
        "ftp://host/",
        "http://user:password@host/",
        "http://host/?q=1",
        "http://host/#fragment",
        "invalid",
    ] {
        assert!(Client::new(None).with_base_url(base).is_err());
    }
}

#[tokio::test]
async fn formatting_json_connection_and_http_client_errors_does_not_recurse() {
    let json_error = Error::from(serde_json::from_str::<Value>("{").unwrap_err());
    assert!(json_error.to_string().contains("EOF"));
    assert!(std::error::Error::source(&json_error).is_some());
    let fixture = Fixture::new(vec![(200, String::new(), "not json".into())]).await;
    let error = fixture
        .client(None)
        .search(&json!({}))
        .await
        .unwrap()
        .json()
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Reqwest(_)));
    assert!(!error.to_string().is_empty());
    fixture.finish().await;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}/api/v2", listener.local_addr().unwrap());
    drop(listener);
    let error = Client::new_custom(Client::http_client_builder().no_proxy().build().unwrap(), None, None)
        .with_base_url(&base)
        .unwrap()
        .search(&json!({}))
        .await
        .unwrap_err();
    assert!(matches!(error, Error::ApiConnectionError(_)));
    assert!(!error.to_string().is_empty());
    assert!(std::error::Error::source(&error).is_some());
}

#[tokio::test]
async fn custom_timeouts_remain_effective() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}/api/v2", listener.local_addr().unwrap());
    let client = Client::new_custom(
        Client::http_client_builder()
            .no_proxy()
            .timeout(Duration::from_millis(30))
            .build()
            .unwrap(),
        None,
        None,
    )
    .with_base_url(&base)
    .unwrap();
    let error = client.search(&json!({})).await.unwrap_err();
    assert!(matches!(error, Error::ApiConnectionError(_)));
    let error = client.delete_playlist("list", &json!({})).await.unwrap_err();
    assert!(matches!(error, Error::ApiConnectionError(_)));
}
