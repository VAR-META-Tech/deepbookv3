use anyhow::{Context, Result};
use sui_sdk::SuiClient;
use sui_sdk::types::SUI_CLOCK_OBJECT_ID;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::transaction::{Command, ProgrammableMoveCall};

use super::balance_manager::BalanceManagerContract;
use crate::utils::config::{DEEP_SCALAR, DeepBookConfig, FLOAT_SCALAR, GAS_BUDGET, MAX_TIMESTAMP};
use crate::utils::{get_object_arg, parse_type_input};

#[derive(Clone)]
pub struct DeepBookContract {
    client: SuiClient,
    config: DeepBookConfig,
    balance_manager: BalanceManagerContract,
}

impl DeepBookContract {
    pub fn new(
        client: SuiClient,
        config: DeepBookConfig,
        balance_manager: BalanceManagerContract,
    ) -> Self {
        Self {
            client,
            config,
            balance_manager,
        }
    }

    pub async fn get_pool_id_by_assets(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        base_type: &str,
        quote_type: &str,
    ) -> Result<()> {
        // Fetch registry ID
        let registry_object = get_object_arg(&self.client, &self.config.registry_id)
            .await
            .context("Failed to get registry object argument")?;

        // Convert package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // Prepare inputs
        let registry_object_arg = ptb.input(registry_object)?;

        // Construct transaction
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "get_pool_id_by_asset".to_string(),
            type_arguments: vec![parse_type_input(base_type)?, parse_type_input(quote_type)?],
            arguments: vec![registry_object_arg],
        })));

        Ok(())
    }

    pub async fn pool_trade_params(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        // Fetch pool details
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        // Get object arguments
        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        // Convert package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // Prepare inputs
        let pool_object_arg = ptb.input(pool_object)?;

        // Construct transaction
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "pool_trade_params".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg],
        })));

        Ok(())
    }

    pub async fn pool_book_params(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        // Fetch pool details
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        // Get object arguments
        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        // Convert package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // Prepare inputs
        let pool_object_arg = ptb.input(pool_object)?;

        // Construct transaction
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "pool_book_params".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg],
        })));

        Ok(())
    }

    /// Get the account information for a given pool and balance manager
    pub async fn account(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<()> {
        // Fetch pool and balance manager details
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);
        let manager = self.config.get_balance_manager(manager_key);

        // Get object arguments
        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;
        let manager_object = get_object_arg(&self.client, &manager.address)
            .await
            .context("Failed to get manager object argument")?;

        // Convert package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // Prepare inputs
        let pool_object_arg = ptb.input(pool_object)?;
        let manager_object_arg = ptb.input(manager_object)?;

        // Construct transaction
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "account".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg, manager_object_arg],
        })));

        Ok(())
    }

    /// Get the locked balance for a given pool and balance manager
    pub async fn locked_balance(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<()> {
        // Fetch pool and balance manager details
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);
        let manager = self.config.get_balance_manager(manager_key);

        // Get object arguments
        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;
        let manager_object = get_object_arg(&self.client, &manager.address)
            .await
            .context("Failed to get manager object argument")?;

        // Convert package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // Prepare inputs
        let pool_object_arg = ptb.input(pool_object)?;
        let manager_object_arg = ptb.input(manager_object)?;

        // Construct transaction
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "locked_balance".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg, manager_object_arg],
        })));

        Ok(())
    }
    pub async fn get_pool_deep_price(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        // Fetch pool details
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        // Get object arguments
        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        // Convert package ID to ObjectID
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        // Prepare inputs
        let pool_object_arg = ptb.input(pool_object)?;

        // Construct transaction
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "get_order_deep_price".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg],
        })));

        Ok(())
    }
}
