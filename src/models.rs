use serde::{Deserialize, Serialize};

// Balances models
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BalanceApiResponse {
    pub balances: Vec<Balance>,
    pub request_time: String,
    pub response_time: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Balance {
    pub address: String,
    pub chain: String,
    pub symbol: Option<String>,
    pub amount: String,
    pub decimals: Option<u8>,
    pub price_usd: Option<f64>,
    pub value_usd: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct BalanceOutputRecord {
    pub address_type: String,
    pub address: String,
    pub chain: String,
    pub symbol: String,
    pub raw_amount: String,
    pub adjusted_amount: f64,
    pub decimals: u8,
    pub price_usd: f64,
    pub value_usd: f64,
    pub date: String,
    pub token_address: String,
}

// Transactions models
#[derive(Debug, Deserialize)]
pub struct TransactionApiResponse {
    pub transactions: Vec<Transaction>,
    pub next_offset: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub address: String,
    pub block_hash: String,
    pub block_number: u64,
    pub block_time: String,
    pub chain: String,
    pub from: String,
    pub to: String,
    pub data: String,
    pub gas_price: String,
    pub hash: String,
    pub index: u64,
    pub transaction_type: String,
    pub value: String,
    pub max_fee_per_gas: Option<String>,
    pub max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionOutputRecord {
    pub address_type: String,
    pub address: String,
    pub chain: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub transaction_type: String,
    pub gas_price: f64,
    pub max_fee_per_gas: f64,
    pub max_priority_fee_per_gas: f64,
    pub block_time: String,
} 