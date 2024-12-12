# capitalik
retrieves working capital as token balances, gas and txs history for set of contract addresses

## Usage
cargo run -- balances        # To process balances 
cargo run -- transactions    # To process transactions

## Notes
- balances.csv contains balances for all addresses in addresses.csv
- transactions.csv contains transactions for all addresses in addresses.csv
- addresses.csv is a csv with two columns: type, address and it is input file

