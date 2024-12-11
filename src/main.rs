use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;
use csv;
use std::env;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    balances: Vec<Balance>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Balance {
    address: String,
    chain: String,
    symbol: Option<String>,
    amount: String,
    price_usd: Option<f64>,
    value_usd: Option<f64>,
}

#[derive(Debug, Serialize)]
struct OutputRecord {
    wallet_address: String,
    chain: String,
    symbol: String,
    amount: String,
    price_usd: f64,
    value_usd: f64,
}

async fn fetch_balances(client: &reqwest::Client, address: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!(
        "https://api.dune.com/api/echo/v1/balances/evm/{}",
        address
    );
    
    println!("Fetching data for address: {}", address);
    let response = client
        .get(&url)
        .send()
        .await?;
    
    println!("Response status: {}", response.status());
    
    let response_data = response.json::<ApiResponse>().await?;
    println!("Got {} balances", response_data.balances.len());

    Ok(response_data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv().ok();
    let api_key = env::var("DUNE_API_KEY").expect("DUNE_API_KEY must be set");

    // Set up HTTP client with headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Dune-Api-Key",
        HeaderValue::from_str(&api_key)?
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Create output CSV writer
    let mut wtr = csv::Writer::from_path("output.csv")?;
    wtr.write_record(&[
        "wallet_address",
        "chain",
        "symbol",
        "amount",
        "price_usd",
        "value_usd",
    ])?;

    // Process addresses sequentially instead of concurrently
    let mut rdr = csv::Reader::from_path("addresses.csv")?;
    for result in rdr.records() {
        let record = result?;
        let address = record.get(0).ok_or("Missing address")?.to_string();
        
        match fetch_balances(&client, &address).await {
            Ok(response) => {
                for balance in response.balances {
                    // Debug print each balance
                    println!("Processing balance: {:?}", balance);
                    
                    // More lenient processing - use default values if None
                    let symbol = balance.symbol.unwrap_or_else(|| "UNKNOWN".to_string());
                    let price_usd = balance.price_usd.unwrap_or(0.0);
                    let value_usd = balance.value_usd.unwrap_or(0.0);
                    
                    wtr.serialize(OutputRecord {
                        wallet_address: address.clone(),
                        chain: balance.chain,
                        symbol,
                        amount: balance.amount,
                        price_usd,
                        value_usd,
                    })?;
                }
            },
            Err(e) => {
                eprintln!("Error fetching balance for {}: {}", address, e);
            }
        }
    }

    wtr.flush()?;
    println!("Processing complete! Check output.csv for results.");
    Ok(())
}
    