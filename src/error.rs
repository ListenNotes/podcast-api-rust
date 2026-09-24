use reqwest::{StatusCode, header::HeaderMap};

/// The status, headers, and body returned by an unsuccessful API request.
#[derive(Debug)]
pub struct ApiError {
    /// HTTP status code.
    pub status: StatusCode,
    /// API response headers, including usage and quota information.
    pub headers: HeaderMap,
    /// Unmodified response body.
    pub body: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP {}: {}", self.status, self.body)
    }
}

/// Errors returned by [`crate::Client`]. HTTP errors retain their response context.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Wrong API key or a suspended account (401).
    AuthenticationError(Box<ApiError>),
    /// The account is not allowed to perform this operation (403).
    PermissionDeniedError(Box<ApiError>),
    /// Unable to connect or a request timed out.
    ApiConnectionError(reqwest::Error),
    /// Invalid API request (400).
    InvalidRequestError(Box<ApiError>),
    /// Request quota or rate limit exceeded (429).
    RateLimitError(Box<ApiError>),
    /// Endpoint or requested content was not found (404).
    NotFoundError(Box<ApiError>),
    /// Other unsuccessful HTTP responses, including redirects and server errors.
    ListenApiError(Box<ApiError>),
    /// Invalid local parameter structure or path identifier.
    InvalidParameter(String),
    /// Other errors from the HTTP client.
    Reqwest(reqwest::Error),
    /// JSON creation or processing error.
    Json(serde_json::Error),
}

impl Error {
    pub(crate) fn from_api(error: Box<ApiError>) -> Self {
        match error.status {
            StatusCode::BAD_REQUEST => Self::InvalidRequestError(error),
            StatusCode::UNAUTHORIZED => Self::AuthenticationError(error),
            StatusCode::FORBIDDEN => Self::PermissionDeniedError(error),
            StatusCode::NOT_FOUND => Self::NotFoundError(error),
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimitError(error),
            _ => Self::ListenApiError(error),
        }
    }

    /// Access the response status, headers, and body for an HTTP error.
    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            Self::AuthenticationError(e)
            | Self::PermissionDeniedError(e)
            | Self::InvalidRequestError(e)
            | Self::RateLimitError(e)
            | Self::NotFoundError(e)
            | Self::ListenApiError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_connect() || error.is_timeout() {
            Self::ApiConnectionError(error)
        } else {
            Self::Reqwest(error)
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reqwest(e) | Self::ApiConnectionError(e) => Some(e),
            Self::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(error) = self.api_error() {
            return std::fmt::Display::fmt(error, f);
        }
        match self {
            Self::Reqwest(e) | Self::ApiConnectionError(e) => std::fmt::Display::fmt(e, f),
            Self::Json(e) => std::fmt::Display::fmt(e, f),
            Self::InvalidParameter(message) => f.write_str(message),
            _ => unreachable!("HTTP error handled above"),
        }
    }
}
