mod support;

use podcast_api::{Client, Error};
use serde_json::json;

fn client() -> Client<'static> {
    // Never read credentials or base URLs from the environment.
    Client::new_custom(Client::http_client_builder().no_proxy().build().unwrap(), None, None)
}

#[tokio::test]
#[ignore = "explicit public-mock network test; never runs in the default suite"]
async fn all_methods_against_the_public_mock() {
    // No environment API key, custom base URL, redirects, retries, or system proxy.
    let client = client();
    for op in support::contract()["operations"].as_array().unwrap() {
        let name = op["operationId"].as_str().unwrap();
        let response = support::methods::call(&client, name, &op["example_params"])
            .await
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(
            response.request.url().origin().ascii_serialization(),
            "https://listen-api-test.listennotes.com"
        );
        assert!(!response.request.headers().contains_key("x-listenapi-key"));
        assert_eq!(response.request.method().as_str(), op["method"]);
        let status = if matches!(name, "createPlaylist" | "addPlaylistItem") {
            201
        } else {
            200
        };
        assert_eq!(response.response.status().as_u16(), status, "{name}");
        for header in [
            "x-listenapi-usage",
            "x-listenapi-freequota",
            "x-listenapi-latency-seconds",
            "x-listenapi-nextbillingdate",
        ] {
            assert!(
                response.response.headers().contains_key(header),
                "{name}: missing {header}"
            );
        }
        let body = response.json().await.unwrap();
        assert!(body.is_object(), "{name} must return a JSON object");
        match name {
            "createPlaylist" | "updatePlaylist" | "getPlaylistById" => {
                assert!(body["id"].is_string());
                assert!(matches!(body["type"].as_str(), Some("episode_list" | "podcast_list")));
                assert!(matches!(
                    body["visibility"].as_str(),
                    Some("public" | "private" | "unlisted")
                ));
                assert!(
                    body["listennotes_url"]
                        .as_str()
                        .unwrap()
                        .starts_with("https://www.listennotes.com/")
                );
            }
            "addPlaylistItem" | "updatePlaylistItemNotes" => {
                assert!(body["id"].is_u64());
                assert!(body["notes"].is_string());
                assert!(body["data"].is_object());
                assert!(matches!(body["type"].as_str(), Some("episode" | "podcast")));
            }
            "deletePlaylistItem" => {
                assert_eq!(body["deleted"], true);
                assert!(body["id"].is_u64());
            }
            _ => {}
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[tokio::test]
#[ignore = "explicit public-mock network test"]
async fn encoded_search_and_empty_playlist_fields() {
    let client = client();
    let response = client.search(&json!({"q": "c++ & café / podcasts"})).await.unwrap();
    assert_eq!(
        response
            .request
            .url()
            .query_pairs()
            .find(|(key, _)| key == "q")
            .unwrap()
            .1,
        "c++ & café / podcasts"
    );
    assert!(response.json().await.unwrap()["results"].is_array());
    let response = client
        .update_playlist("m1pe7z60bsw", &json!({"description":""}))
        .await
        .unwrap();
    assert_eq!(response.request.body().unwrap().as_bytes().unwrap(), b"description=");
    assert!(response.json().await.unwrap()["id"].is_string());
    let response = client
        .add_playlist_item(
            "m1pe7z60bsw",
            &json!({"podcast_id":"4d3fe717742d4963a85562e9f84d8c79", "notes":""}),
        )
        .await
        .unwrap();
    let form = String::from_utf8(response.request.body().unwrap().as_bytes().unwrap().to_vec()).unwrap();
    assert!(form.contains("podcast_id="));
    assert!(!form.contains("episode_id="));
    assert!(form.contains("notes="));
    assert_eq!(response.response.status().as_u16(), 201);
}

#[tokio::test]
#[ignore = "explicit public-mock network test"]
async fn missing_mock_route_preserves_404() {
    let error = client()
        .with_base_url("https://listen-api-test.listennotes.com/api/v2/missing-route")
        .unwrap()
        .search(&json!({}))
        .await
        .unwrap_err();
    assert!(matches!(error, Error::NotFoundError(_)));
    assert_eq!(error.api_error().unwrap().status.as_u16(), 404);
}
