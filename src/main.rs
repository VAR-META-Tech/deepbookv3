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

pub async fn get_coin_with_balance(
    client: &SuiClient,
    owner: SuiAddress,
    coin_type: &str,
    amount: u64,
) -> Result<CallArg> {
    let coins = client
        .coin_read_api()
        .get_coins(owner, Some(coin_type.to_string()), None, None)
        .await
        .map_err(|e| anyhow!("Failed to fetch coins for type {}: {}", coin_type, e))?
        .data;

    // Find a coin with at least the required balance
    let coin = coins
        .into_iter()
        .find(|c| c.balance >= amount)
        .ok_or(anyhow!("No suitable coin found for deposit"))?;
    let coin_id = coin.coin_object_id;

    let coin_object = CallArg::Object(ObjectArg::ImmOrOwnedObject((
        coin_id,
        coin.version,
        coin.digest,
    )));
    print!("Coin object: {:?}", coin_object);
    Ok(coin_object)
}

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
        client.clone(),
        sender,
        "testnet",
        Some(balance_managers),
        None,
        None,
        None,
    );
    let pt = deep_book_client
        .balance_manager
        .create_and_share_balance_manager()
        .await?;
    let coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let coin = coins.data.into_iter().next().unwrap();

    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![coin.object_ref()],
        pt,
        gas_budget,
        gas_price,
    );

    // 4) sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

    // 5) execute the transaction
    print!("Executing the transaction...");
    let transaction_response = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    print!("done\n Transaction information: ");
    println!("{:?}", transaction_response);

    Ok(())
}
