use crate::models::{TransactionApiResponse, TransactionOutputRecord};
use std::error::Error;

pub async fn fetch_transactions(
    client: &reqwest::Client, 
    address: &str,
    offset: Option<&str>
) -> Result<TransactionApiResponse, Box<dyn Error>> {
    let mut url = format!(
        "https://api.dune.com/api/echo/v1/transactions/evm/{}?chain_ids=1",
        address
    );
    
    if let Some(offset_value) = offset {
        url.push_str(&format!("&offset={}", offset_value));
    }
    
    println!("Fetching URL: {}", url);
    let response = client.get(&url).send().await?;
    
    println!("Response status: {}", response.status());
    
    if !response.status().is_success() {
        return Err(format!("API error: {} - {}", response.status(), response.text().await?).into());
    }
    
    let response_data = response.json::<TransactionApiResponse>().await?;
    Ok(response_data)
}

pub async fn process_transactions(client: &reqwest::Client) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path("transactions.csv")?;
        
    // Update headers
    wtr.write_record(&[
        "address_type",
        "address",
        "chain",
        "from",
        "to",
        "value",
        "transaction_type",
        "gas_price",
        "max_fee_per_gas",
        "max_priority_fee_per_gas",
        "block_time",
    ])?;

    let mut rdr = csv::Reader::from_path("addresses.csv")?;
    for result in rdr.records() {
        let record = result?;
        let address_type = record.get(0).ok_or("Missing type")?.to_string();
        let address = record.get(1).ok_or("Missing address")?.to_string();
        
        let mut offset: Option<String> = None;
        loop {
            match fetch_transactions(&client, &address, offset.as_deref()).await {
                Ok(response) => {
                    for tx in response.transactions {
                        // Convert hex strings to decimal values
                        let gas_price = hex_to_decimal(&tx.gas_price) / 1e9;
                        let max_fee = tx.max_fee_per_gas
                            .as_ref()
                            .map_or(gas_price, |fee| hex_to_decimal(fee) / 1e9);
                        let max_priority_fee = tx.max_priority_fee_per_gas
                            .as_ref()
                            .map_or(0.0, |fee| hex_to_decimal(fee) / 1e9);
                        let value_decimal = hex_to_decimal(&tx.value) / 1e18; // Convert to ETH
                        
                        wtr.serialize(TransactionOutputRecord {
                            address_type: address_type.clone(),
                            address: address.clone(),
                            chain: tx.chain,
                            block_time: tx.block_time,
                            from: tx.from,
                            to: tx.to,
                            value: value_decimal.to_string(), // Use decimal value instead of hex
                            transaction_type: tx.transaction_type,
                            gas_price,
                            max_fee_per_gas: max_fee,
                            max_priority_fee_per_gas: max_priority_fee,
                        })?;
                    }
                    
                    // Check if there are more pages
                    if let Some(next_offset) = response.next_offset {
                        offset = Some(next_offset);
                    } else {
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("Error fetching transactions for {}: {}", address, e);
                    break;
                }
            }
        }
    }

    wtr.flush()?;
    Ok(())
}

fn hex_to_decimal(hex_str: &str) -> f64 {
    let hex_str = hex_str.trim_start_matches("0x");
    u64::from_str_radix(hex_str, 16).unwrap_or(0) as f64
} 