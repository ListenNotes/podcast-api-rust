use super::{Api, Result};
use reqwest::RequestBuilder;
use serde_json::Value;
use std::time::Duration;

static DEFAULT_USER_AGENT: &str = "api-podcast-rust";

/// Client for accessing Listen Notes API.
pub struct Client<'a> {
    /// HTTP client.
    client: reqwest::Client,
    /// API context.
    api: Api<'a>,
    /// User Agent Header for API calls.
    user_agent: &'a str,
}

#[derive(Debug)]
/// Response and request context for API call.
pub struct Response {
    /// HTTP response.
    pub response: reqwest::Response,
    /// HTTP request that resulted in this response.
    pub request: reqwest::Request,
}

impl Response {
    /// Get JSON data object from [`reqwest::Response`].
    pub async fn json(self) -> Result<Value> {
        Ok(self.response.json().await?)
    }
}

impl Client<'_> {
    /// Creates new Listen API Client.
    ///
    /// Uses default HTTP client with 30 second timeouts.
    ///
    /// To access production API:
    /// ```
    /// let client = podcast_api::Client::new(Some("YOUR-API-KEY"));
    /// ```
    /// To access mock API:
    /// ```
    /// let client = podcast_api::Client::new(None);
    /// ```
    pub fn new(id: Option<&str>) -> Client {
        Client {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            api: if let Some(id) = id {
                Api::Production(id)
            } else {
                Api::Mock
            },
            user_agent: DEFAULT_USER_AGENT,
        }
    }

    /// Creates new Listen API Client with user provided HTTP Client.
    pub fn new_custom<'a>(
        client: reqwest::Client,
        id: Option<&'a str>,
        user_agent: Option<&'a str>,
    ) -> Client<'a> {
        Client {
            client,
            api: if let Some(id) = id {
                Api::Production(id)
            } else {
                Api::Mock
            },
            user_agent: if let Some(user_agent) = user_agent {
                user_agent
            } else {
                DEFAULT_USER_AGENT
            },
        }
    }

    /// Calls [`GET /search`](https://www.listennotes.com/api/docs/#get-api-v2-search) with supplied parameters.
    pub async fn search(&self, parameters: &Value) -> Result<Response> {
        self.get("search", parameters).await
    }

    /// Calls [`GET /typeahead`](https://www.listennotes.com/api/docs/#get-api-v2-typeahead) with supplied parameters.
    pub async fn typeahead(&self, parameters: &Value) -> Result<Response> {
        self.get("typeahead", parameters).await
    }

    /// Calls [`GET /best_podcasts`](https://www.listennotes.com/api/docs/#get-api-v2-best_podcasts) with supplied parameters.
    pub async fn fetch_best_podcasts(&self, parameters: &Value) -> Result<Response> {
        self.get("best_podcasts", parameters).await
    }

    /// Calls [`GET /podcasts/{id}`](https://www.listennotes.com/api/docs/#get-api-v2-podcasts-id) with supplied parameters.
    pub async fn fetch_podcast_by_id(&self, id: &str, parameters: &Value) -> Result<Response> {
        self.get(&format!("podcasts/{}", id), parameters).await
    }

    /// Calls [`POST /podcasts`](https://www.listennotes.com/api/docs/#post-api-v2-podcasts) with supplied parameters.
    pub async fn batch_fetch_podcasts(&self, parameters: &Value) -> Result<Response> {
        self.post("podcasts", parameters).await
    }

    /// Calls [`GET /episodes/{id}`](https://www.listennotes.com/api/docs/#get-api-v2-episodes-id) with supplied parameters.
    pub async fn fetch_episode_by_id(&self, id: &str, parameters: &Value) -> Result<Response> {
        self.get(&format!("episodes/{}", id), parameters).await
    }

    /// Calls [`POST /episodes`](https://www.listennotes.com/api/docs/#post-api-v2-episodes) with supplied parameters.
    pub async fn batch_fetch_episodes(&self, parameters: &Value) -> Result<Response> {
        self.post("episodes", parameters).await
    }

    async fn get(&self, endpoint: &str, parameters: &Value) -> Result<Response> {
        let request = self
            .client
            .get(format!("{}/{}", self.api.url(), endpoint))
            .query(parameters);

        Ok(self.request(request).await?)
    }

    async fn post(&self, endpoint: &str, parameters: &Value) -> Result<Response> {
        let request = self
            .client
            .post(format!("{}/{}", self.api.url(), endpoint))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Self::urlencoded_from_json(parameters));

        Ok(self.request(request).await?)
    }

    async fn request(&self, request: RequestBuilder) -> Result<Response> {
        let request = if let Api::Production(key) = self.api {
            request.header("X-ListenAPI-Key", key)
        } else {
            request
        }
        .header("User-Agent", self.user_agent)
        .build()?;

        Ok(Response {
            response: self.client.execute(request.try_clone().expect("Error can remain unhandled because we're not using streams, which are the try_clone fail condition")).await?,
            request,
        })
    }

    fn urlencoded_from_json(json: &Value) -> String {
        if let Some(v) = json.as_object() {
            v.iter()
                .map(|(key, value)| {
                    format!(
                        "{}={}",
                        key,
                        match value {
                            Value::String(s) => s.to_owned(), // serde_json String(_) formatter includes the quotations marks, this doesn't
                            _ => format!("{}", value),
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join("&")
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    #[test]
    fn urlencoded_from_json() {
        assert_eq!(
            super::Client::urlencoded_from_json(&json!({
                "a": 1,
                "b": true,
                "c": "test_string"
            })),
            "a=1&b=true&c=test_string"
        );
    }
}
