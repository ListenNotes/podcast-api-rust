use super::{Api, ApiError, Error, Result};
use reqwest::{Method, Url, header::HeaderValue};
use serde_json::Value;
use std::time::Duration;

const DEFAULT_USER_AGENT: &str = concat!("podcast-api-rust ", env!("CARGO_PKG_VERSION"));

/// Client for accessing Listen Notes API. Each instance owns its HTTP configuration.
pub struct Client<'a> {
    client: reqwest::Client,
    api: Api<'a>,
    user_agent: &'a str,
    base_url: Url,
}

/// Response and request context for an API call.
#[derive(Debug)]
pub struct Response {
    /// HTTP response, including status and response headers.
    pub response: reqwest::Response,
    /// HTTP request that resulted in this response. API key headers are sensitive.
    pub request: reqwest::Request,
}

impl Response {
    /// Consume the response and deserialize its JSON body.
    pub async fn json(self) -> Result<Value> {
        Ok(self.response.json().await?)
    }
}

impl<'a> Client<'a> {
    /// Create a production client with an API key, or a public mock client with `None`.
    /// Uses a 30-second total timeout and a 10-second connection timeout.
    pub fn new(api_key: Option<&'a str>) -> Self {
        Self::new_custom(
            Self::http_client_builder()
                .build()
                .expect("build Listen API HTTP client"),
            api_key,
            None,
        )
    }

    /// Start with the SDK's timeout, no-redirect, and no-retry defaults.
    /// Use this builder with [`Self::new_custom`] to customize proxies or timeouts.
    pub fn http_client_builder() -> reqwest::ClientBuilder {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
    }

    /// Create a client with a supplied HTTP client and optional User-Agent.
    /// The caller controls that HTTP client's timeout, proxy, redirect, and retry policies.
    pub fn new_custom(client: reqwest::Client, api_key: Option<&'a str>, user_agent: Option<&'a str>) -> Self {
        let api = api_key.map_or(Api::Mock, Api::Production);
        let base_url = Url::parse(api.url()).expect("valid Listen API base URL");
        Self {
            client,
            api,
            user_agent: user_agent.unwrap_or(DEFAULT_USER_AGENT),
            base_url,
        }
    }

    /// Override the API base URL, for example for a local test server.
    /// Requests send this client's credentials to the supplied server.
    pub fn with_base_url(mut self, base_url: &str) -> Result<Self> {
        let url = Url::parse(base_url).map_err(|_| Error::InvalidParameter("invalid base URL".into()))?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(Error::InvalidParameter(
                "base URL must be HTTP(S) without credentials, query, or fragment".into(),
            ));
        }
        self.base_url = url;
        Ok(self)
    }

    pub(crate) async fn request_api(
        &self,
        method: Method,
        path: &str,
        path_params: &[(&str, &str)],
        query_names: &[&str],
        parameters: &Value,
    ) -> Result<Response> {
        let parameters = parameters
            .as_object()
            .ok_or_else(|| Error::InvalidParameter("parameters must be a JSON object".into()))?;
        let mut url = self.base_url.clone();
        {
            let mut segments = url
                .path_segments_mut()
                .map_err(|_| Error::InvalidParameter("invalid base URL".into()))?;
            segments.pop_if_empty();
            for segment in path.trim_start_matches('/').split('/') {
                if let Some(name) = segment.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                    let value = path_params
                        .iter()
                        .find(|(key, _)| *key == name)
                        .map(|(_, value)| *value)
                        .ok_or_else(|| Error::InvalidParameter(format!("missing path parameter: {name}")))?;
                    // URL parsers normalize dot segments; reject them instead of changing endpoints.
                    if value.is_empty() || value == "." || value == ".." {
                        return Err(Error::InvalidParameter(format!("invalid path parameter: {name}")));
                    }
                    segments.push(value);
                } else {
                    segments.push(segment);
                }
            }
        }
        let has_body = method == Method::POST || method == Method::PUT;
        let mut query = Vec::new();
        let mut body = Vec::new();
        for (name, value) in parameters {
            if value.is_null() || path_params.iter().any(|(key, _)| key == name) {
                continue;
            }
            let encoded = value.as_str().map(str::to_owned).unwrap_or_else(|| value.to_string());
            if !has_body || query_names.contains(&name.as_str()) {
                query.push((name, encoded));
            } else {
                body.push((name, encoded));
            }
        }
        let mut builder = self
            .client
            .request(method, url)
            .query(&query)
            .header("User-Agent", self.user_agent);
        if let Api::Production(key) = self.api {
            let mut header =
                HeaderValue::from_str(key).map_err(|_| Error::InvalidParameter("invalid API key header".into()))?;
            header.set_sensitive(true);
            builder = builder.header("X-ListenAPI-Key", header);
        }
        if has_body {
            builder = builder.form(&body);
        }
        let request = builder.build()?;
        let outgoing = request
            .try_clone()
            .ok_or_else(|| Error::InvalidParameter("request body cannot be cloned".into()))?;
        let response = self.client.execute(outgoing).await?;
        let status = response.status();
        if !status.is_success() {
            let headers = response.headers().clone();
            let body = response.text().await?;
            return Err(Error::from_api(Box::new(ApiError { status, headers, body })));
        }
        Ok(Response { response, request })
    }
}
