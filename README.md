# Podcast API Rust Library

The Podcast API Rust library provides convenient access to the [Listen Notes Podcast API](https://www.listennotes.com/api/) from
applications written in the Rust language.

Simple and no-nonsense podcast search & directory API. Search the meta data of all podcasts and episodes by people, places, or topics. It's the same API that powers [the best podcast search engine Listen Notes](https://www.listennotes.com/).

If you have any questions, please contact [hello@listennotes.com](hello@listennotes.com?subject=Questions+about+the+Rust+SDK+of+Listen+API)

<a href="https://www.listennotes.com/api/"><img src="https://raw.githubusercontent.com/ListenNotes/ListenApiDemo/master/web/src/powered_by_listennotes.png" width="300" /></a>

## Installation

Add the following line to your `Cargo.toml` file's dependencies section:

```toml
[dependencies]
podcast-api = "1.0.1"
```

## Usage

The library needs to be configured with your account's API key which is
available in your [Listen API Dashboard](https://www.listennotes.com/api/dashboard/#apps). Set `api_key` to its
value:

```rust
use podcast_api::Error;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Api Key  (None => Test API, Some(key) => Production API)
    let api_key = None;
    // let api_key = Some("put your api key here");    

    // Create client
    let client = podcast_api::Client::new(api_key);

    // Call API
    match client
        .typeahead(&json!({
            "q": "startup",
            "show_podcasts": 1
        }))
        .await
    {
        Ok(response) => {
            println!("Successfully called Listen Notes API.");
            if let Ok(body) = response.json().await {
                println!("Response Body:");
                println!("{}", body);
            } else {
                println!("Response body JSON data parsing error.")
            }
        }
        Err(err) => {
            match err {
                Error::NotFoundError => { println!("Not Found: {}", err); }
                Error::AuthenticationError => { println!("Authentication Issue: {}", err); }
                Error::RateLimitError => { println!("Rate Limit: {}", err); }
                Error::InvalidRequestError => { println!("Invalid Request: {}", err); }
                Error::ListenApiError => { println!("API Error: {}", err); }
                Error::ApiConnectionError => { println!("Connection Issue: {}", err); }
                Error::Reqwest(err) => { println!("Reqwest HTTP Client Error: {}", err); }
                Error::Json(err) => { println!("JSON Parsing Error: {}", err); }
            }
        }
    };
}
```

If `apiKey` is `None`, then we'll connect to a [mock server](https://www.listennotes.com/api/tutorials/#faq0) that returns fake data for testing purposes.


### Handling errors

Unsuccessful requests return errors.

| Error  | Description |
| ------------- | ------------- |
|  AuthenticationError | wrong api key or your account is suspended  |
| InvalidRequestError  | something wrong on your end (client side errors), e.g., missing required parameters  |
| RateLimitError  | you are using FREE plan and you exceed the quota limit  |
| NotFoundError  | endpoint not exist, or podcast / episode not exist  |
| ApiConnectionError | failed to connect to Listen API servers | 
| ListenApiError  | something wrong on our end (unexpected server errors)  |

All errors can be found in [this file](https://github.com/ListenNotes/podcast-api-rust/blob/main/src/error.rs).


## Development

### Check
 - formatting: `cargo fmt -- --check`  
 - valid code: `cargo check`  
 - linting: `cargo clippy`  

### Open Docs
`cargo doc --open`

### Build
`cargo build`  

### Test
`cargo test`

### Run example app
```sh
cd examples/sample && cargo run
```
