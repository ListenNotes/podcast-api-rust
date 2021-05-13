pub enum Api {
    Production(String),
    Mock,
}

impl Api {
    pub fn url(&self) -> &str {
        match &self {
            Api::Production(_) => "https://listen-api.listennotes.com/api/v2",
            Api::Mock => "https://listen-api-test.listennotes.com/api/v2",
        }
    }
}