use std::collections::HashMap;
use std::{any, str::FromStr};

use anyhow::{Error, Result};
use anyhow::{Ok, anyhow};
use deepbookv3::client::DeepBookClient;
use deepbookv3::transactions::balance_manager;
use deepbookv3::types::{
    BalanceManager, OrderType, PlaceLimitOrderParams, PlaceMarketOrderParams, SelfMatchingOptions,
    SwapParams,
};
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
use sui_types::collection_types::VecSet;
use sui_types::transaction::ProgrammableTransaction;

pub async fn setup_client() -> Result<(SuiClient, SuiAddress, DeepBookClient)> {
    let client = SuiClientBuilder::default().build_testnet().await?;
    let sender =
        SuiAddress::from_str("0x38a27d258039c629219b3dbaaeb502381d26f9b93f985e2fec7d248db00d3cf1")?;

    let balance_managers = HashMap::from([(
        "MANAGER_2".to_string(),
        BalanceManager {
            address: "0x08933685e0246a2ddae2f5e5628fdeba09de831cadf5ad949db308807f18bee5", // balance_manager for testnet
            // address: "0x73e7bc2f1007a4f1ffcc42af9305e4e7ce16274297e2e513b2503b9c85c287d4", // balance_manager for devnet
            trade_cap: None,
            deposit_cap: None,
            withdraw_cap: None,
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

async fn test_deposit_to_manager() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();
    // Step 1: Set up transaction for withdrawal
    let recipient = sender; // Self-withdrawal test
    // deep_book_client
    //     .balance_manager
    //     .withdraw_all_from_manager(&mut ptb, "MANAGER_2", "SUI", recipient)
    //     .await?;

    deep_book_client
        .balance_manager
        .deposit_into_manager(&mut ptb, "MANAGER_2", "DEEP", 1.1)
        .await?;
    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, Some("0x2::sui::SUI".to_string()), None, None)
        .await?
        .data;
    let gas_object_refs: Vec<ObjectRef> = gas_coins
        .iter()
        .map(|coin| (coin.coin_object_id, coin.version, coin.digest))
        .collect();

    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data: TransactionData = TransactionData::new_programmable(
        sender,
        gas_object_refs,
        ptb.finish(),
        gas_budget,
        gas_price,
    );

    println!("Signing and executing transaction...");
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("transaction_response: {:?} ", transaction_response);
    Ok(())
}
async fn test_place_limit_order() -> Result<()> {
    println!("Placing limit order...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up transaction for place_limit_order
    let params = PlaceLimitOrderParams {
        pool_key: "DBUSDT_SUI".to_string(),
        balance_manager_key: "MANAGER_2".to_string(),
        client_order_id: "123123".to_string(),
        price: 2.27,
        quantity: 10.0,
        is_bid: false,
        expiration: None,
        order_type: Some(OrderType::NoRestriction),
        self_matching_option: Some(SelfMatchingOptions::SelfMatchingAllowed),
        pay_with_deep: Some(true),
    };
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();
    deep_book_client
        .deep_book
        .place_limit_order(&mut ptb, &params)
        .await?;

    let pt = ptb.finish();
    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 50_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute the transaction
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("Transaction response: {:?}", transaction_response);

    // assert!(
    //     transaction_response.digest.is_some(),
    //     "Transaction digest should not be empty"
    // );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb = ProgrammableTransactionBuilder::new();

    let (base_coin_result, quote_coin_result, deep_coin_result) = deep_book_client
        .deep_book
        .swap_exact_quote_for_base(
            &mut ptb,
            &SwapParams {
                pool_key: "DEEP_SUI".to_string(),
                amount: 1.0,      // Quote amount (e.g., DBUSDT)
                deep_amount: 0.0, // DEEP tokens burned
                min_out: 0.0,     // Expected min base out (e.g., SUI)
            },
        )
        .await?;

    ptb.transfer_args(
        sender,
        vec![base_coin_result, quote_coin_result, deep_coin_result],
    );

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, Some("0x2::sui::SUI".to_string()), None, None)
        .await?
        .data;

    let gas_object_refs: Vec<ObjectRef> = gas_coins
        .iter()
        .map(|coin| (coin.coin_object_id, coin.version, coin.digest))
        .collect();

    let gas_budget = 50_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let pt = ptb.finish();

    println!("📜 Commands for swap_exact_quote_for_base:");
    for (i, cmd) in pt.commands.iter().enumerate() {
        println!("  [{}] {:?}", i, cmd);
    }

    let tx_data =
        TransactionData::new_programmable(sender, gas_object_refs, pt, gas_budget, gas_price);

    println!("🚀 Signing and executing quote-for-base swap transaction...");
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("✅ Transaction response: {:?}", transaction_response);

    Ok(())
}
