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
use crate::utils::{get_exact_coin, parse_type_input};

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

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let manager = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "new".to_string(),
            type_arguments: vec![],
            arguments: vec![],
        })));

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

        Ok(ptb.finish())
    }

    pub async fn withdraw_from_manager(
        &self,
        client: &SuiClient,
        manager_key: &str,
        coin_key: &str,
        amount_to_withdraw: f64,
        recipient: SuiAddress,
    ) -> Result<ProgrammableTransaction> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        let manager_id = self.config.get_balance_manager(manager_key).address;

        let coin = self.config.get_coin(coin_key);
        let withdraw_input = (amount_to_withdraw * coin.scalar as f64) as u64;
        let manager_object = get_object_arg(client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        let manager_arg = ptb.input(manager_object)?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let type_argument = parse_type_input(&coin.coin_type)?;

        let withdraw_arg = ptb.pure(withdraw_input)?;

        let coin_object = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "withdraw".to_string(),
            type_arguments: vec![type_argument],
            arguments: vec![manager_arg, withdraw_arg],
        })));

        let recipient_arg = ptb.pure(recipient)?;
        ptb.command(Command::TransferObjects(vec![coin_object], recipient_arg));

        Ok(ptb.finish())
    }

    pub async fn withdraw_all_from_manager(
        &self,
        client: &SuiClient,
        manager_key: &str,
        coin_key: &str,
        recipient: SuiAddress,
    ) -> Result<ProgrammableTransaction> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // ✅ Fetch Manager ID
        let manager_id = self.config.get_balance_manager(manager_key).address;

        // ✅ Fetch Coin Type
        let coin = self.config.get_coin(coin_key);

        // ✅ Convert Manager ID to ObjectRef
        let manager_object = get_object_arg(client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        // ✅ Insert Manager Object into Transaction
        let manager_arg = ptb.input(manager_object)?;

        // ✅ Convert DeepBook package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // ✅ Parse Coin Type
        let type_argument = parse_type_input(&coin.coin_type)?;

        // ✅ Call Move Function `balance_manager::withdraw_all`
        let withdrawal_coin = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "withdraw_all".to_string(),
            type_arguments: vec![type_argument],
            arguments: vec![manager_arg],
        })));

        // ✅ Transfer Withdrawn Coin to Recipient
        let recipient_arg = ptb.pure(recipient)?;
        ptb.command(Command::TransferObjects(
            vec![withdrawal_coin],
            recipient_arg,
        ));

        // ✅ Finalize Transaction
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

        // Fetch manager ID and coin details
        let manager_id = self.config.get_balance_manager(manager_key).address;
        let coin = self.config.get_coin(coin_key);

        // Convert deposit amount to correct precision
        let deposit_input = (amount_to_deposit * coin.scalar as f64) as u64;

        // Get an exact coin object for deposit
        let coin_arg = get_exact_coin(
            client,
            self.config.sender_address,
            &coin.coin_type,
            deposit_input,
            &mut ptb,
        )
        .await?;

        // Get manager object
        let manager_object = get_object_arg(client, &manager_id)
            .await
            .context("Failed to get object argument for manager")?;

        // Convert deepbook package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)
            .context("Invalid package ID format")?;

        // Parse coin type
        let type_argument = parse_type_input(&coin.coin_type)
            .context("Failed to parse type input for coin type")?;

        // Insert inputs into transaction
        let manager_arg = ptb.input(manager_object)?;

        // Create Move Call
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "deposit".to_string(),
            type_arguments: vec![type_argument],
            arguments: vec![manager_arg, coin_arg],
        })));

        // Finalize transaction
        Ok(ptb.finish())
    }

    pub async fn check_manager_balance(
        &self,
        client: &SuiClient,
        manager_key: &str,
        coin_key: &str,
    ) -> Result<ProgrammableTransaction, anyhow::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        let manager_id = self.config.get_balance_manager(manager_key).address;
        let coin_type = self.config.get_coin(coin_key).coin_type;

        let type_argument =
            parse_type_input(coin_type).context("Failed to parse type input for coin_type")?;

        let manager_object = get_object_arg(client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        let manager_arg = ptb.input(manager_object)?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)
            .context("Invalid package ID format")?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "balance".to_string(),
            type_arguments: vec![type_argument],
            arguments: vec![manager_arg],
        })));

        let builder = ptb.finish();

        Ok(builder)
    }

    pub async fn get_manager_owner(
        &self,
        client: &SuiClient,
        manager_key: &str,
    ) -> Result<ProgrammableTransaction> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // ✅ Fetch Manager ID
        let manager_id = self.config.get_balance_manager(manager_key).address;

        // ✅ Convert Manager ID to ObjectRef
        let manager_object = get_object_arg(client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        // ✅ Insert Manager Object into Transaction
        let manager_arg = ptb.input(manager_object)?;

        // ✅ Convert DeepBook package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // ✅ Call Move Function `balance_manager::owner`
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "owner".to_string(),
            type_arguments: vec![],
            arguments: vec![manager_arg],
        })));

        // ✅ Finalize Transaction
        Ok(ptb.finish())
    }

    pub async fn get_manager_id(
        &self,
        client: &SuiClient,
        manager_key: &str,
    ) -> Result<ProgrammableTransaction> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // ✅ Fetch Manager ID
        let manager_id = self.config.get_balance_manager(manager_key).address;

        // ✅ Convert Manager ID to ObjectRef
        let manager_object = get_object_arg(client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        // ✅ Insert Manager Object into Transaction
        let manager_arg = ptb.input(manager_object)?;

        // ✅ Convert DeepBook package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // ✅ Call Move Function `balance_manager::id`
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "balance_manager".to_string(),
            function: "id".to_string(),
            type_arguments: vec![],
            arguments: vec![manager_arg],
        })));

        // ✅ Finalize Transaction
        Ok(ptb.finish())
    }
}
