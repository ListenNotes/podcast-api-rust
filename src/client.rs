use super::{Api, Result};
use reqwest::RequestBuilder;
use serde_json::Value;

/// Client for accessing Listen Notes API.
pub struct Client<'a> {
    /// HTTP client.
    client: reqwest::Client,
    /// API context.
    api: Api<'a>,
}

impl Client<'_> {
    /// Creates new Listen API Client.
    ///
    /// To access production API:
    /// ```
    /// let client = podcast_api::Client::new(reqwest::Client::new(), Some("YOUR-API-KEY"));
    /// ```
    /// To access mock API:
    /// ```
    /// let client = podcast_api::Client::new(reqwest::Client::new(), None);
    /// ```
    pub fn new(client: reqwest::Client, id: Option<&str>) -> Client {
        Client {
            client,
            api: if let Some(id) = id {
                Api::Production(id)
            } else {
                Api::Mock
            },
        }
    }

    /// Calls [`GET /search`](https://www.listennotes.com/api/docs/#get-api-v2-search) with supplied parameters.
    pub async fn search(&self, parameters: &Value) -> Result<Value> {
        self.get("search", parameters).await
    }

    /// Calls [`GET /typeahead`](https://www.listennotes.com/api/docs/#get-api-v2-typeahead) with supplied parameters.
    pub async fn typeahead(&self, parameters: &Value) -> Result<Value> {
        self.get("typeahead", parameters).await
    }

    /// Calls [`GET /best_podcasts`](https://www.listennotes.com/api/docs/#get-api-v2-best_podcasts) with supplied parameters.
    pub async fn best_podcasts(&self, parameters: &Value) -> Result<Value> {
        self.get("best_podcasts", parameters).await
    }

    /// Calls [`GET /podcasts/{id}`](https://www.listennotes.com/api/docs/#get-api-v2-podcasts-id) with supplied parameters.
    pub async fn podcast(&self, id: &str, parameters: &Value) -> Result<Value> {
        self.get(&format!("podcasts/{}", id), parameters).await
    }

    /// Calls [`POST /podcasts`](https://www.listennotes.com/api/docs/#post-api-v2-podcasts) with supplied parameters.
    pub async fn podcasts(&self, parameters: &Value) -> Result<Value> {
        self.post("podcasts", parameters).await
    }

    /// Calls [`GET /episodes/{id}`](https://www.listennotes.com/api/docs/#get-api-v2-episodes-id) with supplied parameters.
    pub async fn episode(&self, id: &str, parameters: &Value) -> Result<Value> {
        self.get(&format!("episodes/{}", id), parameters).await
    }

    /// Calls [`POST /episodes`](https://www.listennotes.com/api/docs/#post-api-v2-episodes) with supplied parameters.
    pub async fn episodes(&self, parameters: &Value) -> Result<Value> {
        self.post("episodes", parameters).await
    }

    async fn get(&self, endpoint: &str, parameters: &Value) -> Result<Value> {
        let request = self
            .client
            .get(format!("{}/{}", self.api.url(), endpoint))
            .query(parameters);

        Ok(self.request(request).await?)
    }

    async fn post(&self, endpoint: &str, parameters: &Value) -> Result<Value> {
        let request = self
            .client
            .post(format!("{}/{}", self.api.url(), endpoint))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_json::to_string(&parameters)?); // TODO: switch to URL encoding

        Ok(self.request(request).await?)
    }

    async fn request(&self, request: RequestBuilder) -> Result<Value> {
        Ok(if let Api::Production(key) = self.api {
            request.header("X-ListenAPI-Key", key)
        } else {
            request
        }
        .send()
        .await?
        .json()
        .await?)
    }
}
