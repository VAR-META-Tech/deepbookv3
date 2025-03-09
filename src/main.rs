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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
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
        client,
        sender,
        "testnet",
        Some(balance_managers),
        None,
        None,
        None,
    );

    match deep_book_client
        .check_manager_balance("MANAGER_2", "SUI")
        .await
    {
        Ok((coin_type, balance)) => {
            println!("Balance: {} {}", balance, coin_type);
        }
        Err(e) => {
            eprintln!("Error checking balance: {:?}", e);
        }
    }
    Ok(())
}
