//! Official library for accessing the [Listen API](https://www.listennotes.com/api) by [Listen Notes](https://www.listennotes.com).
//!
//! # Quick Start Example
//!
//! ```
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Api Key  (None => Test API, Some(key) => Production API)
//!     let api_key = None;
//!
//!     // Create client
//!     let client = podcast_api::Client::new(api_key);
//!
//!     // Call API
//!     match client
//!         .typeahead(&json!({
//!             "q": "startup",
//!             "show_podcasts": 1
//!         }))
//!         .await
//!     {
//!         Ok(response) => {
//!             println!("Successfully called \"typeahead\" endpoint.");
//!             println!("Response Body:");
//!             println!("{:?}", response);
//!         }
//!         Err(err) => {
//!             println!("Error calling \"typeahead\" endpoint:");
//!             println!("{:?},", err);
//!         }
//!     };
//! }
//! ```
#![deny(missing_docs)]

mod api;
mod client;
mod error;

use api::Api;

pub use client::Client;
pub use error::Error;
/// Result for API calls from [`Client`]
pub type Result<T> = std::result::Result<T, error::Error>;
