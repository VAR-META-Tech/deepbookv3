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
use crate::utils::{get_coins_to_transfer, get_object_arg};
use crate::utils::{get_exact_coin, parse_type_input};

#[derive(Clone)]
pub struct BalanceManagerContract {
    client: SuiClient,
    config: DeepBookConfig,
}

impl BalanceManagerContract {
    pub fn new(client: SuiClient, config: DeepBookConfig) -> Self {
        Self { client, config }
    }

    pub async fn create_and_share_balance_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
    ) -> Result<()> {
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

        Ok(())
    }

    pub async fn withdraw_from_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        amount_to_withdraw: f64,
        recipient: SuiAddress,
    ) -> Result<()> {
        let manager_id = self.config.get_balance_manager(manager_key).address;

        let coin = self.config.get_coin(coin_key);
        let withdraw_input = (amount_to_withdraw * coin.scalar as f64) as u64;
        let manager_object = get_object_arg(&self.client, manager_id)
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

        Ok(())
    }

    pub async fn withdraw_all_from_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        recipient: SuiAddress,
    ) -> Result<()> {
        // ✅ Fetch Manager ID
        let manager_id = self.config.get_balance_manager(manager_key).address;

        // ✅ Fetch Coin Type
        let coin = self.config.get_coin(coin_key);

        // ✅ Convert Manager ID to ObjectRef
        let manager_object = get_object_arg(&self.client, manager_id)
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
        Ok(())
    }

    pub async fn deposit_into_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        amount_to_deposit: f64,
    ) -> Result<()> {
        // Fetch manager ID and coin details
        let manager_id = self.config.get_balance_manager(manager_key).address;
        let coin = self.config.get_coin(coin_key);

        // Convert deposit amount to correct precision
        let deposit_input = (amount_to_deposit * coin.scalar as f64) as u64;

        // Get an exact coin object for deposit

        let coin_arg = get_coins_to_transfer(
            &self.client,
            ptb,
            self.config.sender_address,
            &coin.coin_type,
            deposit_input,
        )
        .await?;

        // Get manager object
        let manager_object = get_object_arg(&self.client, &manager_id)
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
        Ok(())
    }

    pub async fn check_manager_balance(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
    ) -> Result<()> {
        let manager_id = self.config.get_balance_manager(manager_key).address;
        let coin_type = self.config.get_coin(coin_key).coin_type;

        let type_argument =
            parse_type_input(coin_type).context("Failed to parse type input for coin_type")?;

        let manager_object = get_object_arg(&self.client, manager_id)
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

        Ok(())
    }

    pub async fn generate_proof(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> Result<Argument> {
        let balance_manager = self.config.get_balance_manager(manager_key);

        // ✅ Determine which proof generation function to call
        if let Some(trade_cap) = balance_manager.trade_cap {
            Ok(self
                .generate_proof_as_trader(ptb, balance_manager.address, trade_cap)
                .await?)
        } else {
            Ok(self
                .generate_proof_as_owner(ptb, balance_manager.address)
                .await?)
        }
    }

    /// Generate a trade proof as the owner
    pub async fn generate_proof_as_owner(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_id: &str,
    ) -> Result<Argument> {
        let manager_object = get_object_arg(&self.client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let manager_input = ptb.input(manager_object)?;

        Ok(
            ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
                package: package_id,
                module: "balance_manager".to_string(),
                function: "generate_proof_as_owner".to_string(),
                type_arguments: vec![],
                arguments: vec![manager_input],
            }))),
        )
    }

    /// Generate a trade proof as a trader
    async fn generate_proof_as_trader(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_id: &str,
        trade_cap_id: &str,
    ) -> Result<Argument> {
        let manager_object = get_object_arg(&self.client, manager_id)
            .await
            .context("Failed to get object argument for manager_id")?;

        let trade_cap_object = get_object_arg(&self.client, trade_cap_id)
            .await
            .context("Failed to get object argument for trade_cap_id")?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;
        let manager_input = ptb.input(manager_object)?;
        let trade_cap_input = ptb.input(trade_cap_object)?;

        Ok(
            ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
                package: package_id,
                module: "balance_manager".to_string(),
                function: "generate_proof_as_trader".to_string(),
                type_arguments: vec![],

                arguments: vec![manager_input, trade_cap_input],
            }))),
        )
    }

    pub async fn get_manager_owner(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> Result<()> {
        // ✅ Fetch Manager ID
        let manager_id = self.config.get_balance_manager(manager_key).address;

        // ✅ Convert Manager ID to ObjectRef
        let manager_object = get_object_arg(&self.client, manager_id)
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
        Ok(())
    }

    pub async fn get_manager_id(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> Result<()> {
        // ✅ Fetch Manager ID
        let manager_id = self.config.get_balance_manager(manager_key).address;

        // ✅ Convert Manager ID to ObjectRef
        let manager_object = get_object_arg(&self.client, manager_id)
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
        Ok(())
    }
}
