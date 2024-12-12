use crate::models::{BalanceApiResponse, BalanceOutputRecord};
use std::error::Error;

pub async fn fetch_balances(client: &reqwest::Client, address: &str) -> Result<BalanceApiResponse, Box<dyn Error>> {
    let url = format!(
        "https://api.dune.com/api/echo/v1/balances/evm/{}?filters=native&exclude_spam_tokens=exclude_spam_tokens",
        address
    );
    
    println!("Fetching balances for address: {}", address);
    let response = client
        .get(&url)
        .send()
        .await?;
    
    println!("Response status: {}", response.status());
    
    let response_data = response.json::<BalanceApiResponse>().await?;
    println!("Got {} balances", response_data.balances.len());

    Ok(response_data)
}

pub async fn process_balances(client: &reqwest::Client) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path("balances.csv")?;
    
    // Write headers
    wtr.write_record(&[
        "address_type",
        "address",
        "chain",
        "symbol",
        "raw_amount",
        "adjusted_amount",
        "decimals",
        "price_usd",
        "value_usd",
        "date",
        "token_address",
    ])?;

    let mut rdr = csv::Reader::from_path("addresses.csv")?;
    for result in rdr.records() {
        let record = result?;
        let address_type = record.get(0).ok_or("Missing type")?.to_string();
        let address = record.get(1).ok_or("Missing address")?.to_string();
        
        match fetch_balances(&client, &address).await {
            Ok(response) => {
                for balance in response.balances {
                    let symbol = balance.symbol.unwrap_or_else(|| "UNKNOWN".to_string());
                    let price_usd = balance.price_usd.unwrap_or(0.0);
                    let value_usd = balance.value_usd.unwrap_or(0.0);
                    let decimals = balance.decimals.unwrap_or(0);
                    
                    let date = response.response_time
                        .split('T')
                        .next()
                        .unwrap_or("unknown")
                        .to_string();
                    
                    let adjusted_amount = if let Ok(amount) = balance.amount.parse::<f64>() {
                        amount / (10_f64.powi(decimals as i32))
                    } else {
                        0.0
                    };
                    
                    wtr.serialize(BalanceOutputRecord {
                        address_type: address_type.clone(),
                        address: address.clone(),
                        chain: balance.chain.clone(),
                        symbol,
                        raw_amount: balance.amount,
                        adjusted_amount,
                        decimals,
                        price_usd,
                        value_usd,
                        date,
                        token_address: balance.address,
                    })?;
                }
            },
            Err(e) => {
                eprintln!("Error fetching balance for {}: {}", address, e);
            }
        }
    }

    wtr.flush()?;
    Ok(())
} 