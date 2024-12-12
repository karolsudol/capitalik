mod models;
mod balances;
mod transactions;

use std::error::Error;
use dotenv::dotenv;
use std::env;
use reqwest::header::{HeaderMap, HeaderValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <balances|transactions>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    
    // Set up client
    let api_key = env::var("DUNE_API_KEY").expect("DUNE_API_KEY must be set");
    let mut headers = HeaderMap::new();
    headers.insert("X-Dune-Api-Key", HeaderValue::from_str(&api_key)?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    match mode.as_str() {
        "balances" => {
            balances::process_balances(&client).await?;
            println!("Balance processing complete! Check balances.csv for results.");
        },
        "transactions" => {
            transactions::process_transactions(&client).await?;
            println!("Transaction processing complete! Check transactions.csv for results.");
        },
        _ => {
            eprintln!("Invalid mode. Use 'balances' or 'transactions'");
            std::process::exit(1);
        }
    }

    Ok(())
}
    