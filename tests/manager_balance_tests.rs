use anyhow::Result;
use deepbookv3::client::DeepBookClient;
use deepbookv3::types::BalanceManager;
use shared_crypto::intent::Intent;
use std::collections::HashMap;
use std::str::FromStr;
use sui_config::{SUI_KEYSTORE_FILENAME, sui_config_dir};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::SuiClientBuilder;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_sdk::types::transaction::{Transaction, TransactionData};
use tokio; // Ensure `tokio` is used for async tests

#[tokio::test]
async fn test_create_and_share_balance_manager() -> Result<()> {
    let client = SuiClientBuilder::default().build_testnet().await?;

    let sender =
        SuiAddress::from_str("0x38a27d258039c629219b3dbaaeb502381d26f9b93f985e2fec7d248db00d3cf1")?;

    let balance_managers: HashMap<String, BalanceManager> = HashMap::from([(
        "MANAGER_2".to_string(),
        BalanceManager {
            address: "0x08933685e0246a2ddae2f5e5628fdeba09de831cadf5ad949db308807f18bee5",
            trade_cap: None,
        },
    )]);

    let deep_book_client = DeepBookClient::new(
        client.clone(),
        sender,
        "testnet",
        Some(balance_managers),
        None,
        None,
        None,
    );

    // Step 1: Generate transaction
    let pt = deep_book_client
        .balance_manager
        .create_and_share_balance_manager()
        .await?;

    // Step 2: Fetch coins to use as gas
    let coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let coin = coins.data.into_iter().next().unwrap();

    // Step 3: Set gas budget and get gas price
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;

    // Step 4: Create transaction data
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![coin.object_ref()],
        pt,
        gas_budget,
        gas_price,
    );

    // Step 5: Sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

    // Step 6: Execute transaction
    let transaction_response = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    // Step 7: Validate response
    assert!(
        transaction_response
            .confirmed_local_execution
            .unwrap_or(false),
        "Transaction execution failed"
    );

    println!("Transaction Successful: {:?}", transaction_response);
    Ok(())
}

#[tokio::test]
async fn test_check_manager_balance() -> Result<()> {
    // Initialize SuiClient
    let client = SuiClientBuilder::default().build_testnet().await?;

    // Set up sender address
    let sender =
        SuiAddress::from_str("0x38a27d258039c629219b3dbaaeb502381d26f9b93f985e2fec7d248db00d3cf1")?;

    // Create balance managers
    let balance_managers: HashMap<String, BalanceManager> = HashMap::from([(
        "MANAGER_2".to_string(),
        BalanceManager {
            address: "0x08933685e0246a2ddae2f5e5628fdeba09de831cadf5ad949db308807f18bee5",
            trade_cap: None,
        },
    )]);

    // Initialize DeepBookClient
    let deep_book_client = DeepBookClient::new(
        client,
        sender,
        "testnet",
        Some(balance_managers),
        None,
        None,
        None,
    );

    // Run check_manager_balance function
    let result = deep_book_client
        .check_manager_balance("MANAGER_2", "SUI")
        .await;

    // Assert expected behavior
    match result {
        Ok((coin_type, balance)) => {
            println!("Balance: {} {}", balance, coin_type);
            assert_eq!(
                coin_type,
                "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
            ); // Expected coin type
            println!("Balance is non-negative as expected");
        }
        Err(e) => {
            panic!("Error checking balance: {:?}", e);
        }
    }

    Ok(())
}
