use super::{Api, Result};
use reqwest::RequestBuilder;
use serde_json::Value;

pub struct Client<'a> {
    client: reqwest::Client,
    api: Api<'a>,
}

impl Client<'_> {
    pub fn new<'a>(client: reqwest::Client, id: Option<&'a str>) -> Client<'a> {
        Client {
            client,
            api: if let Some(id) = id {
                Api::Production(id)
            } else {
                Api::Mock
            },
        }
    }

    pub async fn search(&self, parameters: &Value) -> Result<Value> {
        self.get("search", parameters).await
    }

    pub async fn typeahead(&self, parameters: &Value) -> Result<Value> {
        self.get("typeahead", parameters).await
    }

    pub async fn best_podcasts(&self, parameters: &Value) -> Result<Value> {
        self.get("best_podcasts", parameters).await
    }

    pub async fn podcast(&self, id: &str, parameters: &Value) -> Result<Value> {
        self.get(&format!("podcasts/{}", id), parameters).await
    }

    pub async fn podcasts(&self, parameters: &Value) -> Result<Value> {
        self.post("podcasts", parameters).await
    }

    pub async fn episode(&self, id: &str, parameters: &Value) -> Result<Value> {
        self.get(&format!("episodes/{}", id), parameters).await
    pub async fn episodes(&self, parameters: &Value) -> Result<Value> {
        self.post("episodes", parameters).await
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

trait AddField {
    fn with(&self, key: &str, value: &str) -> Self;
}

impl AddField for Value {
    fn with(&self, key: &str, value: &str) -> Self {
        let mut p = self.clone();
        p[key] = json!(value);
        p
    }
}
