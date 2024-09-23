use std::str::FromStr;
use web3::transports::Http;
use web3::types::U256;
use web3::Web3;

async fn web3_test() {
    // Connect to the Ethereum node
    let transport = Http::new("http://localhost:8545").unwrap();
    let web3 = Web3::new(transport);

    // Perform some web3 operations
    let block_number = web3.eth().block_number().await.unwrap();
    println!("Latest block number: {:?}", block_number);

    use web3::types::H160;

    let balance = web3
        .eth()
        .balance(
            H160::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
            None,
        )
        .await
        .unwrap();
    println!("Account balance: {:?}", balance);

    let gas_price = web3.eth().gas_price().await.unwrap();
    println!("Gas price: {:?}", gas_price);

    let transaction_count = web3
        .eth()
        .transaction_count(
            H160::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap(),
            None,
        )
        .await
        .unwrap();
    println!("Transaction count: {:?}", transaction_count);

    use web3::types::BlockId;

    use web3::types::BlockNumber;

    let block = web3
        .eth()
        .block_with_txs(BlockId::Number(BlockNumber::Number(
            block_number.as_u64().into(),
        )))
        .await
        .unwrap();
    println!("Block: {:?}", block);

    use web3::types::TransactionId;

    use web3::types::H256;

    let transaction_id = TransactionId::from(
        H256::from_str("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
            .unwrap(),
    );
    let transaction = web3.eth().transaction(transaction_id).await.unwrap();
    println!("Transaction: {:?}", transaction);

    let receipt = web3
        .eth()
        .transaction_receipt(
            H256::from_str("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
                .unwrap(),
        )
        .await
        .unwrap();
    println!("Transaction receipt: {:?}", receipt);

    let contract_address: H160 = "0x1234567890abcdef1234567890abcdef12345678"
        .parse()
        .unwrap();

    use web3::contract::Options;

    let options = Options::with(|opt| {
        opt.gas = Some(5_000.into());
        opt.gas_price = Some(1.into());
        opt.value = Some(10.into());
    });
}

fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(web3_test());
}
