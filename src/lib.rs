mod api;
mod client;
mod error;

use api::Api;

pub use client::Client;
pub use error::Error;
pub type Result<T> = std::result::Result<T, error::Error>;
