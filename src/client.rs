use super::{Api, Error, Result};
use http::StatusCode;
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
    pub fn new_custom<'a>(client: reqwest::Client, id: Option<&'a str>, user_agent: Option<&'a str>) -> Client<'a> {
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

    /// Calls [`GET /curated_podcasts/{id}`](https://www.listennotes.com/api/docs/#get-api-v2-curated_podcasts-id) with supplied parameters.
    pub async fn fetch_curated_podcasts_list_by_id(&self, id: &str, parameters: &Value) -> Result<Response> {
        self.get(&format!("curated_podcasts/{}", id), parameters).await
    }

    /// Calls [`GET /curated_podcasts`](https://www.listennotes.com/api/docs/#get-api-v2-curated_podcasts) with supplied parameters.
    pub async fn fetch_curated_podcasts_lists(&self, parameters: &Value) -> Result<Response> {
        self.get("curated_podcasts", parameters).await
    }

    /// Calls [`GET /genres`](https://www.listennotes.com/api/docs/#get-api-v2-genres) with supplied parameters.
    pub async fn fetch_podcast_genres(&self, parameters: &Value) -> Result<Response> {
        self.get("genres", parameters).await
    }

    /// Calls [`GET /regions`](https://www.listennotes.com/api/docs/#get-api-v2-regions) with supplied parameters.
    pub async fn fetch_podcast_regions(&self, parameters: &Value) -> Result<Response> {
        self.get("regions", parameters).await
    }

    /// Calls [`GET /languages`](https://www.listennotes.com/api/docs/#get-api-v2-languages) with supplied parameters.
    pub async fn fetch_podcast_languages(&self, parameters: &Value) -> Result<Response> {
        self.get("languages", parameters).await
    }

    /// Calls [`GET /just_listen`](https://www.listennotes.com/api/docs/#get-api-v2-just_listen) with supplied parameters.
    pub async fn just_listen(&self, parameters: &Value) -> Result<Response> {
        self.get("just_listen", parameters).await
    }

    /// Calls [`GET /podcasts/{id}/recommendations`](https://www.listennotes.com/api/docs/#get-api-v2-podcasts-id-recommendations) with supplied parameters.
    pub async fn fetch_recommendations_for_podcast(&self, id: &str, parameters: &Value) -> Result<Response> {
        self.get(&format!("podcasts/{}/recommendations", id), parameters).await
    }

    /// Calls [`GET /episodes/{id}/recommendations`](https://www.listennotes.com/api/docs/#get-api-v2-episodes-id-recommendations) with supplied parameters.
    pub async fn fetch_recommendations_for_episode(&self, id: &str, parameters: &Value) -> Result<Response> {
        self.get(&format!("episodes/{}/recommendations", id), parameters).await
    }

    /// Calls [`GET /playlists/{id}`](https://www.listennotes.com/api/docs/#get-api-v2-playlists-id) with supplied parameters.
    pub async fn fetch_playlist_by_id(&self, id: &str, parameters: &Value) -> Result<Response> {
        self.get(&format!("playlists/{}", id), parameters).await
    }

    /// Calls [`GET /playlists`](https://www.listennotes.com/api/docs/#get-api-v2-playlists) with supplied parameters.
    pub async fn fetch_my_playlists(&self, parameters: &Value) -> Result<Response> {
        self.get("playlists", parameters).await
    }

    /// Calls [`POST /podcasts/submit`](https://www.listennotes.com/api/docs/#post-api-v2-podcasts-submit) with supplied parameters.
    pub async fn submit_podcast(&self, parameters: &Value) -> Result<Response> {
        self.post("podcasts/submit", parameters).await
    }

    /// Calls [`DELETE /podcasts/{id}`](https://www.listennotes.com/api/docs/#delete-api-v2-podcasts-id) with supplied parameters.
    pub async fn delete_podcast(&self, id: &str, parameters: &Value) -> Result<Response> {
        self.delete(&format!("podcasts/{}", id), parameters).await
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

    async fn delete(&self, endpoint: &str, parameters: &Value) -> Result<Response> {
        let request = self
            .client
            .delete(format!("{}/{}", self.api.url(), endpoint))
            .query(parameters);

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

        let response = self
            .client
            .execute(request.try_clone().expect(
                "Error can remain unhandled because we're not using streams, which are the try_clone fail condition",
            ))
            .await;

        match &response {
            Ok(response) => match response.status() {
                StatusCode::NOT_FOUND => return Err(Error::NotFoundError),
                StatusCode::UNAUTHORIZED => return Err(Error::AuthenticationError),
                StatusCode::TOO_MANY_REQUESTS => return Err(Error::RateLimitError),
                StatusCode::BAD_REQUEST => return Err(Error::InvalidRequestError),
                StatusCode::INTERNAL_SERVER_ERROR => return Err(Error::ListenApiError),
                _ => {}
            },
            Err(err) => {
                if err.is_connect() || err.is_timeout() {
                    return Err(Error::ApiConnectionError);
                }
            }
        };

        Ok(Response {
            response: response?,
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
