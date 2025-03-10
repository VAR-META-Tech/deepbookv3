// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Ok, Result, anyhow};
use sui_sdk::SuiClient;
use sui_sdk::rpc_types::{SuiObjectDataOptions, SuiObjectResponse};
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::types::transaction::{
    Argument, CallArg, Command, ObjectArg, ProgrammableMoveCall, ProgrammableTransaction,
    TransactionKind,
};
use sui_sdk::types::{
    programmable_transaction_builder::ProgrammableTransactionBuilder, transaction::Transaction,
};

use crate::utils::config::DeepBookConfig;
use crate::utils::get_object_arg;
use crate::utils::parse_type_input;

#[derive(Debug, Clone)]
pub struct BalanceManagerContract {
    config: DeepBookConfig,
}

impl BalanceManagerContract {
    pub fn new(config: DeepBookConfig) -> Self {
        Self { config }
    }

    pub async fn create_and_share_balance_manager(
        &self,
    ) -> Result<ProgrammableTransaction, anyhow::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // Convert deepbook package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // Step 1: Create a new BalanceManager
        let manager = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "new".to_string(),
            type_arguments: vec![],
            arguments: vec![],
        })));

        // Step 2: Share the BalanceManager object publicly
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: ObjectID::from_hex_literal("0x2")?, // Sui Framework
            module: "transfer".to_string(),
            function: "public_share_object".to_string(),
            type_arguments: vec![parse_type_input(&format!(
                "{}::balance_manager::BalanceManager",
                self.config.deepbook_package_id
            ))?],
            arguments: vec![manager],
        })));

        // Finalize transaction
        Ok(ptb.finish())
    }

    pub async fn deposit_into_manager(
        &self,
        client: &SuiClient,
        manager_key: &str,
        coin_key: &str,
        amount_to_deposit: f64,
    ) -> Result<ProgrammableTransaction> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)
            .context("Invalid package ID format")?;

        let manager_id = self.config.get_balance_manager(manager_key).address;

        let manager_object = get_object_arg(client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        let coin = self.config.get_coin(coin_key);

        let deposit_input = (amount_to_deposit * coin.scalar as f64) as u64;

        let coin_object = get_coin_with_balance(
            client,
            self.config.sender_address,
            coin.coin_type,
            deposit_input as u64,
        )
        .await?;

        let type_argument = parse_type_input(&coin.coin_type)
            .context("Failed to parse type input for coin_type")?;

        let manager_arg = ptb.input(manager_object)?;
        let coin_arg = ptb.input(coin_object)?;
        // let deposit_arg = ptb.input(deposit_input)?;

        // Create Move Call
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "deposit".to_string(),
            type_arguments: vec![type_argument],
            arguments: vec![manager_arg, coin_arg],
        })));

        // Finalize transaction
        let builder = ptb.finish();

        Ok(builder)
    }

    pub async fn check_manager_balance(
        &self,
        client: &SuiClient,
        manager_id: &str,
        coin_type: &str,
    ) -> Result<ProgrammableTransaction, anyhow::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // Parse the type argument safely
        let type_argument =
            parse_type_input(coin_type).context("Failed to parse type input for coin_type")?;

        // Fetch the object argument for the manager
        let manager_object = get_object_arg(client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        // Insert manager object into the transaction builder
        let manager_arg = ptb.input(manager_object)?;

        // Convert deepbook package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)
            .context("Invalid package ID format")?;

        // Construct the Move call command
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "balance".to_string(),
            type_arguments: vec![type_argument],
            arguments: vec![manager_arg],
        })));

        // Finalize transaction
        let builder = ptb.finish();

        Ok(builder)
    }
}

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
