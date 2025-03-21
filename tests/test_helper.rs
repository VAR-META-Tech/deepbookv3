use anyhow::{Result, anyhow};
use deepbookv3::client::DeepBookClient;
use deepbookv3::types::BalanceManager;
use shared_crypto::intent::Intent;
use std::collections::HashMap;
use std::str::FromStr;
use sui_config::{SUI_KEYSTORE_FILENAME, sui_config_dir};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::types::base_types::{ObjectRef, SuiAddress};
use sui_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_sdk::types::transaction::{Transaction, TransactionData};
use sui_sdk::{SuiClient, SuiClientBuilder};
use tokio::time::{Duration, sleep};

/// Set up a SuiClient, sender address, and DeepBookClient instance for testing.
pub async fn setup_client() -> Result<(SuiClient, SuiAddress, DeepBookClient)> {
    let client = SuiClientBuilder::default().build_testnet().await?;
    let sender =
        SuiAddress::from_str("0x38a27d258039c629219b3dbaaeb502381d26f9b93f985e2fec7d248db00d3cf1")?;

    let balance_managers = HashMap::from([(
        "MANAGER_2".to_string(),
        BalanceManager {
            // address: "0x08933685e0246a2ddae2f5e5628fdeba09de831cadf5ad949db308807f18bee5", // balance_manager for testnet
            address: "0x73e7bc2f1007a4f1ffcc42af9305e4e7ce16274297e2e513b2503b9c85c287d4", // balance_manager for devnet
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
        Some("0x1277f3450132015f868b7b7adc6f4f3cee8ecc2d7d03607243a53709e58ea726".to_string()),
    );

    Ok((client, sender, deep_book_client))
}

/// Retrieve a gas coin from the sender's account.
/// Retrieve a fresh gas coin from the sender's account.
pub async fn get_gas_coin(client: &SuiClient, sender: SuiAddress) -> Result<ObjectRef> {
    let coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;

    // Ensure the gas coin is large enough for at least one transaction
    let gas_coin = coins
        .data
        .into_iter()
        .find(|c| c.balance > 5_000_000) // Minimum required gas
        .ok_or_else(|| anyhow!("No suitable gas coin found"))?;

    Ok(gas_coin.object_ref())
}

/// Sign and execute a transaction.
pub async fn sign_and_execute(
    client: &SuiClient,
    sender: SuiAddress,
    tx_data: TransactionData,
) -> Result<()> {
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

    let mut attempts = 0;
    let max_attempts = 5;

    while attempts < max_attempts {
        let transaction_response = client
            .quorum_driver_api()
            .execute_transaction_block(
                Transaction::from_data(tx_data.clone(), vec![signature.clone()]),
                SuiTransactionBlockResponseOptions::full_content(),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await;

        match transaction_response {
            Ok(response) => {
                assert!(
                    response.confirmed_local_execution.unwrap_or(false),
                    "Transaction execution failed"
                );
                println!("Transaction Successful: {:?}", response);

                return Ok(());
            }
            Err(e) => {
                if e.to_string().contains("reserved for another transaction") {
                    println!("Transaction is locked. Retrying in 3 seconds...");
                    sleep(Duration::from_secs(3)).await;
                    attempts += 1;
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    Err(anyhow::anyhow!(
        "Failed after multiple retries due to object contention"
    ))
}
