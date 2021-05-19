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
        Err(err) => match err {
            Error::NotFoundError => {
                println!("Not Found: {}", err);
            }
            Error::AuthenticationError => {
                println!("Authentication Issue: {}", err);
            }
            Error::RateLimitError => {
                println!("Rate Limit: {}", err);
            }
            Error::InvalidRequestError => {
                println!("Invalid Request: {}", err);
            }
            Error::ListenApiError => {
                println!("API Error: {}", err);
            }
            Error::ApiConnectionError => {
                println!("Connection Issue: {}", err);
            }
            Error::Reqwest(err) => {
                println!("Reqwest HTTP Client Error: {}", err);
            }
            Error::Json(err) => {
                println!("JSON Parsing Error: {}", err);
            }
        },
    };
}
