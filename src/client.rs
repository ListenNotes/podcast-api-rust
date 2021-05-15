use super::{Api, Result};
use serde_json::{json, Value};

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

    pub async fn episode_by_id(&self, id: &str, parameters: &Value) -> Result<Value> {
        self.get(&format!("episodes/{}", id), parameters).await
    }

    pub async fn episodes(&self, ids: &[&str], parameters: &Value) -> Result<Value> {
        self.post("episodes", &parameters.with("ids", &ids.join(",").as_str()))
            .await
    }

    pub async fn genres(&self, parameters: &Value) -> Result<Value> {
        self.get("genres", parameters).await
    }

    async fn get(&self, endpoint: &str, parameters: &Value) -> Result<Value> {
        Ok(self
            .client
            .get(format!("{}/{}", self.api.url(), endpoint))
            .query(parameters)
            .send()
            .await?
            .json()
            .await?)
    }

    async fn post(&self, endpoint: &str, parameters: &Value) -> Result<Value> {
        Ok(self
            .client
            .post(format!("{}/{}", self.api.url(), endpoint))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_json::to_string(&parameters)?) // TODO: switch to URL encoding
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
