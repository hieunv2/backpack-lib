use bpx_api_client::BpxClient;
use reqwest::header::HeaderMap;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let API_KEY = env::var("BACKPACK_API_KEY").expect("BACKPACK_API_KEY must be set");
    let SECRET_KEY = env::var("BACKPACK_SECRET_KEY").expect("BACKPACK_SECRET_KEY must be set");

    let base_url = "https://api.backpack.exchange".to_string();

    // Initialize the client
    let client = BpxClient::init(base_url, &API_KEY, &SECRET_KEY, Some(HeaderMap::new()))?;

    // Perform a GET request
    let response = client.get_open_orders(Some("SOL_USDC")).await?;
    println!("GET response: {:?}", response);

    Ok(())
}
