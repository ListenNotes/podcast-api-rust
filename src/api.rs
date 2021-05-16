/// API url and key context.
pub enum Api<'a> {
    /// API context for Listen Notes production API.
    Production(&'a str),
    /// API context for Listen Notes mock API for testing.
    Mock,
}

impl Api<'_> {
    pub fn url(&self) -> &str {
        match &self {
            Api::Production(_) => "https://listen-api.listennotes.com/api/v2",
            Api::Mock => "https://listen-api-test.listennotes.com/api/v2",
        }
    }
}
