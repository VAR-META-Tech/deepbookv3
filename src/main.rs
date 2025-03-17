use std::collections::HashMap;
use std::{any, str::FromStr};

use anyhow::anyhow;
use anyhow::{Error, Result};
use deepbookv3::client::DeepBookClient;
use deepbookv3::transactions::balance_manager;
use deepbookv3::types::{BalanceManager, OrderType, PlaceLimitOrderParams, SelfMatchingOptions};
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
use sui_types::transaction::ProgrammableTransaction;

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

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     let (client, sender, deep_book_client) = setup_client().await?;
//     // Run check_manager_balance function
//     let owner = deep_book_client.get_manager_owner("MANAGER_2").await?;
//     println!("Balance Manager Owner: {:?}", owner);

//     let manager_id = deep_book_client.get_manager_id("MANAGER_2").await?;
//     println!("Balance Manager ID: {:?}", manager_id);

//     // let pt: sui_types::transaction::ProgrammableTransaction = deep_book_client
//     //     .balance_manager
//     //     .deposit_into_manager(&client, "MANAGER_2", "SUI", 10.1)
//     //     .await?;
//     // println!("pt2222: {:}", pt);
//     // let gas_coin = get_gas_coin(&client, sender).await?;
//     let mut pt = ProgrammableTransactionBuilder::new();
//     let gas_coins = client
//         .coin_read_api()
//         .get_coins(sender, Some("0x2::sui::SUI".to_string()), None, None)
//         .await?
//         .data;

//     // Chuyển đổi sang ObjectRef
//     let gas_object_refs: Vec<ObjectRef> = gas_coins
//         .iter()
//         .map(|coin| (coin.coin_object_id, coin.version, coin.digest))
//         .collect();
//     let amount = pt.pure(10_000)?;
//     let a: Argument = pt.command(Command::SplitCoins(Argument::GasCoin, vec![amount]));
//     let receive = pt.pure(&sender)?;
//     pt.command(Command::TransferObjects(vec![a], receive));
//     let ptb = pt.finish();
//     // Step 7: Set up gas and create transaction data
//     let gas_budget = 5_000_000;
//     let gas_price = client.read_api().get_reference_gas_price().await?;
//     let tx_data =
//         TransactionData::new_programmable(sender, gas_object_refs, ptb, gas_budget, gas_price);

//     // Step 8: Sign and execute the transaction
//     println!("Signing and executing transaction...");
//     let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     let params = PlaceLimitOrderParams {
//         pool_key: "DEEP_SUI".to_string(),
//         balance_manager_key: "MANAGER_2".to_string(),
//         client_order_id: "234affs3".to_string(),
//         price: 0.01,
//         quantity: 15.0,
//         is_bid: true,
//         expiration: None,
//         order_type: Some(OrderType::NoRestriction),
//         self_matching_option: Some(SelfMatchingOptions::SelfMatchingAllowed),
//         pay_with_deep: Some(true),
//     };

//     let pt = deep_book_client
//         .deep_book
//         .place_limit_order(&client, &params)
//         .await?;
//     println!("pt2222: {:}", pt);
//     let gas_coin = get_gas_coin(&client, sender).await?;

//     // Step 7: Set up gas and create transaction data
//     let gas_budget = 50_000_000;
//     let gas_price = client.read_api().get_reference_gas_price().await?;
//     let tx_data =
//         TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

//     // Step 8: Sign and execute the transaction
//     println!("Signing and executing transaction...");
//     let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

//     println!("transaction_response: {:?} ", transaction_response);
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Define pool key and tick distance
    let pool_key = "SUI_DBUSDC";
    let tick_from_mid = 10;

    // Act: Fetch level 2 order book ticks
    let (bid_prices, bid_sizes, ask_prices, ask_sizes) = deep_book_client
        .get_level2_ticks_from_mid(pool_key, tick_from_mid)
        .await?;

    // Debugging Output
    println!("Bid Prices: {:?}", bid_prices);
    println!("Bid Sizes: {:?}", bid_sizes);
    println!("Ask Prices: {:?}", ask_prices);
    println!("Ask Sizes: {:?}", ask_sizes);

    // ✅ Check at least some order book data is present (not empty)
    assert!(
        !bid_prices.is_empty() || !ask_prices.is_empty(),
        "Either bid or ask prices should contain data"
    );

    println!("✅ Test passed: get_level2_ticks_from_mid returns valid data.");
    Ok(())
}
