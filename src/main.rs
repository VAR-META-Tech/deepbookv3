use std::collections::HashMap;
use std::{any, str::FromStr};

use anyhow::anyhow;
use anyhow::{Error, Result};
use deepbookv3::client::DeepBookClient;
use deepbookv3::transactions::balance_manager;
use deepbookv3::types::BalanceManager;
use shared_crypto::intent::Intent;
use sui_config::{SUI_KEYSTORE_FILENAME, sui_config_dir};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    SuiClient, SuiClientBuilder,
    rpc_types::{
        DevInspectResults, SuiObjectData, SuiObjectDataOptions, SuiObjectResponse,
        SuiTransactionBlockResponseOptions,
    },
    types::{
        Identifier, TypeTag,
        base_types::{ObjectID, ObjectRef, SuiAddress},
        crypto::SuiKeyPair,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{
            Argument, CallArg, Command, ObjectArg, ProgrammableMoveCall, Transaction,
            TransactionData, TransactionKind,
        },
        type_input::TypeInput,
    },
};

pub async fn setup_client() -> Result<(SuiClient, SuiAddress, DeepBookClient)> {
    let client = SuiClientBuilder::default().build_testnet().await?;
    let sender =
        SuiAddress::from_str("0x38a27d258039c629219b3dbaaeb502381d26f9b93f985e2fec7d248db00d3cf1")?;

    let balance_managers = HashMap::from([(
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

    Ok((client, sender, deep_book_client))
}

/// Retrieve a gas coin from the sender's account.
pub async fn get_gas_coin(client: &SuiClient, sender: SuiAddress) -> Result<ObjectRef> {
    let coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let gas_coin = coins.data.into_iter().next().unwrap();
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

    let transaction_response = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    assert!(
        transaction_response
            .confirmed_local_execution
            .unwrap_or(false),
        "Transaction execution failed"
    );

    println!("Transaction Successful: {:?}", transaction_response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up transaction for withdrawal
    let withdraw_amount = 0.1;
    let recipient = sender; // Self-withdrawal test
    let pt = deep_book_client
        .balance_manager
        .withdraw_all_from_manager(&client, "MANAGER_2", "SUI", recipient)
        .await?;

    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute the transaction
    sign_and_execute(&client, sender, tx_data).await?;

    println!("Withdrawal successful.");
    Ok(())
}
