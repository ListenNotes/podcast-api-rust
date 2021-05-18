/// Error for API calls from [`Client`](super::Client).
#[derive(Debug)]
pub enum Error {
    /// Wrong api key or your account is suspended.
    AuthenticationError,
    /// Fail to connect to API servers.
    ApiConnectionError,
    /// Something wrong on your end (client side errors), e.g., missing required parameters.
    InvalidRequestError,
    /// You are using FREE plan and you exceed the quota limit.
    RateLimitError,
    /// Endpoint not exist, or podcast / episode not exist.
    NotFoundError,
    /// Something wrong on our end (unexpected server errors).
    ListenApiError,
    /// Error from http client.
    Reqwest(reqwest::Error),
    /// Error from JSON creation/processing.
    Json(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Reqwest(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AuthenticationError =>   write!(f, "Wrong api key or your account is suspended."),
            Error::ApiConnectionError =>    write!(f, "Fail to connect to API servers."),
            Error::InvalidRequestError =>   write!(f, "Something wrong on your end (client side errors), e.g., missing required parameters."),
            Error::RateLimitError =>        write!(f, "You are using FREE plan and you exceed the quota limit."),
            Error::NotFoundError =>         write!(f, "Endpoint not exist, or podcast / episode not exist."),
            Error::ListenApiError =>        write!(f, "Something wrong on our end (unexpected server errors)."),
            Error::Reqwest(_) | Error::Json(_) => write!(f, "{}", self)
        }
    }
}
