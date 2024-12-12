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
        url.push_str(&format!("?offset={}", offset_value));
    }
    
    let response = client.get(&url).send().await?;
    let response_data = response.json::<TransactionApiResponse>().await?;
    Ok(response_data)
}

pub async fn process_transactions(client: &reqwest::Client) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path("transactions.csv")?;
        
    // Write headers
    wtr.write_record(&[
        "address_type",
        "address",
        "chain",
        "block_time",
        "from",
        "to",
        "hash",
        "value",
        "transaction_type",
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
                        wtr.serialize(TransactionOutputRecord {
                            address_type: address_type.clone(),
                            address: address.clone(),
                            chain: tx.chain,
                            block_time: tx.block_time,
                            from: tx.from,
                            to: tx.to,
                            hash: tx.hash,
                            value: tx.value,
                            transaction_type: tx.transaction_type,
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