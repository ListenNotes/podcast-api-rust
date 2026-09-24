use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.typeahead(&json!({"q": "startup", "show_podcasts": 1})).await?;
    println!("Status: {}", response.response.status());
    println!("{}", response.json().await?);
    Ok(())
}
