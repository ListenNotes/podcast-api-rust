use serde_json::json;

#[tokio::main]
async fn main() {
    // Api Key  (None => Test API, Some(key) => Production API)
    let api_key = None;

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
            println!("Successfully called \"typeahead\" endpoint.");
            if let Ok(body) = response.json().await {
                println!("Response Body:");
                println!("{:?}", body);
            } else {
                println!("Response body JSON data parsing error.")
            }
        }
        Err(err) => {
            println!("Error calling \"typeahead\" endpoint:");
            println!("{:?},", err);
        }
    };
}
