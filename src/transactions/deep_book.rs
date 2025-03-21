use anyhow::{Context, Ok, Result, anyhow};
use sui_sdk::SuiClient;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::transaction::{CallArg, Command, ProgrammableMoveCall};
use sui_types::transaction::{Argument, ProgrammableTransaction};

use super::balance_manager::BalanceManagerContract;
use crate::types::{
    OrderType, PlaceLimitOrderParams, PlaceMarketOrderParams, SelfMatchingOptions, SwapParams,
};
use crate::utils::config::{DEEP_SCALAR, DeepBookConfig, FLOAT_SCALAR, GAS_BUDGET, MAX_TIMESTAMP};
use crate::utils::{get_coins_to_transfer, get_object_arg, parse_type_input};

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

    pub async fn place_limit_order(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        params: &PlaceLimitOrderParams,
    ) -> Result<()> {
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let PlaceLimitOrderParams {
            pool_key,
            balance_manager_key,
            client_order_id,
            price,
            quantity,
            is_bid,
            expiration,
            order_type,
            self_matching_option,
            pay_with_deep,
        } = params;

        let balance_manager = self
            .config
            .get_balance_manager(balance_manager_key.as_str());
        let balance_manager_object = get_object_arg(&self.client, balance_manager.address)
            .await
            .context("Failed to get object argument for balance_manager")?;

        let balance_manager_arg = ptb.input(balance_manager_object)?;

        let trade_proof = self
            .balance_manager
            .generate_proof(ptb, balance_manager_key)
            .await?;

        let pools = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(pools.base_coin);

        let quote_coin = self.config.get_coin(pools.quote_coin);
        let pool_object = get_object_arg(&self.client, pools.address)
            .await
            .context("Failed to get object argument for pool")?;
        let type_argument_base_coin = parse_type_input(base_coin.coin_type)?;

        let type_argument_quote_coin = parse_type_input(quote_coin.coin_type)?;

        let pool_arg = ptb.input(pool_object)?;

        let client_order_id_arg = ptb.pure(client_order_id.parse().unwrap_or(0u64))?;

        let input_order_number = match order_type {
            Some(value) => *value as u8,
            None => OrderType::NoRestriction as u8,
        };
        let order_type_arg = ptb.pure(input_order_number)?;

        let self_matching_option_number = match self_matching_option {
            Some(value) => *value as u8,
            None => SelfMatchingOptions::SelfMatchingAllowed as u8,
        };

        let self_matching_option_number = ptb.pure(self_matching_option_number)?;

        let input_price = ((price * FLOAT_SCALAR as f64 * quote_coin.scalar as f64)
            / base_coin.scalar as f64)
            .round() as u64;

        let input_price_arg = ptb.pure(input_price)?;

        let input_quantity = (quantity * base_coin.scalar as f64).round() as u64;

        let input_quantity_arg = ptb.pure(input_quantity)?;

        let is_bid_arg = ptb.pure(is_bid)?;

        let pay_with_deep_bool = match pay_with_deep {
            Some(value) => *value,
            None => true,
        };

        let pay_with_deep_arg = ptb.pure(pay_with_deep_bool)?;

        let expiration_number = match expiration {
            Some(value) => *value,
            None => MAX_TIMESTAMP,
        };

        let expiration_number_valid = expiration_number;

        let expiration_arg = ptb.pure(expiration_number_valid)?;

        let clock_arg = ptb.input(CallArg::CLOCK_IMM)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "place_limit_order".to_string(),
            arguments: vec![
                pool_arg,
                balance_manager_arg,
                trade_proof,
                client_order_id_arg,
                order_type_arg,
                self_matching_option_number,
                input_price_arg,
                input_quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
                expiration_arg,
                clock_arg,
            ],
            type_arguments: vec![type_argument_base_coin, type_argument_quote_coin],
        })));

        Ok(())
    }

    pub async fn place_market_order(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        params: &PlaceMarketOrderParams,
    ) -> Result<()> {
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let PlaceMarketOrderParams {
            pool_key,
            balance_manager_key,
            client_order_id,
            quantity,
            is_bid,
            self_matching_option,
            pay_with_deep,
        } = params;

        let balance_manager = self
            .config
            .get_balance_manager(balance_manager_key.as_str());

        let balance_manager_object = get_object_arg(&self.client, balance_manager.address)
            .await
            .context("Failed to get object argument for balance_manager")?;

        let balance_manager_arg = ptb.input(balance_manager_object)?;

        let trade_proof = self
            .balance_manager
            .generate_proof(ptb, balance_manager_key)
            .await?;

        let pools = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(pools.base_coin);

        let quote_coin = self.config.get_coin(pools.quote_coin);
        let pool_object = get_object_arg(&self.client, pools.address)
            .await
            .context("Failed to get object argument for pool")?;
        let type_argument_base_coin = parse_type_input(base_coin.coin_type)?;

        let type_argument_quote_coin = parse_type_input(quote_coin.coin_type)?;

        let pool_arg = ptb.input(pool_object)?;

        let client_order_id_arg = ptb.pure(client_order_id.parse().unwrap_or(0u64))?;

        let self_matching_option_number = match self_matching_option {
            Some(value) => *value as u8,
            None => SelfMatchingOptions::SelfMatchingAllowed as u8,
        };

        let self_matching_option_number = ptb.pure(self_matching_option_number)?;

        let input_quantity = (quantity * base_coin.scalar as f64).round() as u64;

        let input_quantity_arg = ptb.pure(input_quantity)?;

        let is_bid_arg = ptb.pure(is_bid)?;

        let pay_with_deep_bool = match pay_with_deep {
            Some(value) => *value,
            None => true,
        };

        let pay_with_deep_arg = ptb.pure(pay_with_deep_bool)?;

        let clock_arg = ptb.input(CallArg::CLOCK_IMM)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "place_market_order".to_string(),
            arguments: vec![
                pool_arg,
                balance_manager_arg,
                trade_proof,
                client_order_id_arg,
                self_matching_option_number,
                input_quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
                clock_arg,
            ],
            type_arguments: vec![type_argument_base_coin, type_argument_quote_coin],
        })));

        Ok(())
    }

    pub async fn cancel_order(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        balance_manager_key: &str,
        order_id: u128,
    ) -> Result<()> {
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let balance_manager = self.config.get_balance_manager(balance_manager_key);

        let balance_manager_object = get_object_arg(&self.client, balance_manager.address)
            .await
            .context("Failed to get object argument for balance_manager")?;

        let balance_manager_arg = ptb.input(balance_manager_object)?;

        let trade_proof = self
            .balance_manager
            .generate_proof(ptb, balance_manager_key)
            .await?;

        let pools = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(pools.base_coin);

        let quote_coin = self.config.get_coin(pools.quote_coin);
        let pool_object = get_object_arg(&self.client, pools.address)
            .await
            .context("Failed to get object argument for pool")?;
        let type_argument_base_coin = parse_type_input(base_coin.coin_type)?;

        let type_argument_quote_coin = parse_type_input(quote_coin.coin_type)?;

        let pool_arg = ptb.input(pool_object)?;

        let order_id_arg = ptb.pure(order_id)?;

        let clock_arg = ptb.input(CallArg::CLOCK_IMM)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "cancel_order".to_string(),
            arguments: vec![
                pool_arg,
                balance_manager_arg,
                trade_proof,
                order_id_arg,
                clock_arg,
            ],
            type_arguments: vec![type_argument_base_coin, type_argument_quote_coin],
        })));

        Ok(())
    }

    pub async fn cancel_all_orders(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        balance_manager_key: &str,
    ) -> Result<()> {
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let balance_manager = self.config.get_balance_manager(balance_manager_key);

        let balance_manager_object = get_object_arg(&self.client, balance_manager.address)
            .await
            .context("Failed to get object argument for balance_manager")?;

        let balance_manager_arg = ptb.input(balance_manager_object)?;

        let trade_proof = self
            .balance_manager
            .generate_proof(ptb, balance_manager_key)
            .await?;

        let pools = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(pools.base_coin);

        let quote_coin = self.config.get_coin(pools.quote_coin);
        let pool_object = get_object_arg(&self.client, pools.address)
            .await
            .context("Failed to get object argument for pool")?;
        let type_argument_base_coin = parse_type_input(base_coin.coin_type)?;

        let type_argument_quote_coin = parse_type_input(quote_coin.coin_type)?;

        let pool_arg = ptb.input(pool_object)?;

        let clock_arg = ptb.input(CallArg::CLOCK_IMM)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "cancel_all_orders".to_string(),
            arguments: vec![pool_arg, balance_manager_arg, trade_proof, clock_arg],
            type_arguments: vec![type_argument_base_coin, type_argument_quote_coin],
        })));

        Ok(())
    }

    pub async fn mid_price(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_arg = ptb.input(pool_object)?;
        let clock_arg = ptb.input(CallArg::CLOCK_MUT)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "mid_price".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg, clock_arg],
        })));

        Ok(())
    }

    /// Check if a pool is whitelisted
    pub async fn whitelisted(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        // Fetch pool details
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        // Get pool object argument
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
            function: "whitelisted".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg],
        })));

        Ok(())
    }

    pub async fn get_quote_quantity_out(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        base_quantity: f64,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_arg = ptb.input(pool_object)?;
        let base_quantity_arg = ptb.pure((base_quantity * base_coin.scalar as f64) as u64)?;
        let clock_arg = ptb.input(CallArg::CLOCK_MUT)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "get_quote_quantity_out".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg, base_quantity_arg, clock_arg],
        })));

        Ok(())
    }

    pub async fn get_base_quantity_out(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        quote_quantity: f64,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_arg = ptb.input(pool_object)?;
        let quote_quantity_arg = ptb.pure((quote_quantity * quote_coin.scalar as f64) as u64)?;
        let clock_arg = ptb.input(CallArg::CLOCK_MUT)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "get_base_quantity_out".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg, quote_quantity_arg, clock_arg],
        })));

        Ok(())
    }

    pub async fn get_quantity_out(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        base_quantity: f64,
        quote_quantity: f64,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_arg = ptb.input(pool_object)?;
        let base_quantity_arg = ptb.pure((base_quantity * base_coin.scalar as f64) as u64)?;
        let quote_quantity_arg = ptb.pure((quote_quantity * quote_coin.scalar as f64) as u64)?;
        let clock_arg = ptb.input(CallArg::CLOCK_MUT)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "get_quantity_out".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![
                pool_object_arg,
                base_quantity_arg,
                quote_quantity_arg,
                clock_arg,
            ],
        })));

        Ok(())
    }

    pub async fn account_open_orders(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<()> {
        // Fetch pool and balance manager details
        let pool = self.config.get_pool(pool_key);
        let manager = self.config.get_balance_manager(manager_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

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
            function: "account_open_orders".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg, manager_object_arg],
        })));

        Ok(())
    }

    pub async fn get_level2_range(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        price_low: f64,
        price_high: f64,
        is_bid: bool,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_arg = ptb.input(pool_object)?;
        let price_low_arg = ptb.pure(
            (price_low * FLOAT_SCALAR * quote_coin.scalar as f64 / base_coin.scalar as f64) as u64,
        )?;
        let price_high_arg = ptb.pure(
            (price_high * FLOAT_SCALAR * quote_coin.scalar as f64 / base_coin.scalar as f64) as u64,
        )?;
        let is_bid_arg = ptb.pure(is_bid)?;
        let clock_arg = ptb.input(CallArg::CLOCK_MUT)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "get_level2_range".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![
                pool_object_arg,
                price_low_arg,
                price_high_arg,
                is_bid_arg,
                clock_arg,
            ],
        })));

        Ok(())
    }

    pub async fn get_level2_ticks_from_mid(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        tick_from_mid: u64,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_arg = ptb.input(pool_object)?;
        let tick_from_mid_arg = ptb.pure(tick_from_mid)?;
        let clock_arg = ptb.input(CallArg::CLOCK_MUT)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "get_level2_ticks_from_mid".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg, tick_from_mid_arg, clock_arg],
        })));

        Ok(())
    }

    pub async fn vault_balances(
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
            function: "vault_balances".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg],
        })));

        Ok(())
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

    pub async fn swap_exact_base_for_quote(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        params: &SwapParams,
    ) -> Result<(Argument, Argument, Argument)> {
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let SwapParams {
            pool_key,
            amount,
            deep_amount,
            min_out,
        } = params;

        let pool = self.config.get_pool(pool_key);

        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);
        let deep_coin = self.config.get_coin("DEEP");
        let pool_object = get_object_arg(&self.client, &pool.address)
            .await
            .context("Failed to get pool object argument")?;
        let pool_object_arg = ptb.input(pool_object)?;

        let amount_input = (amount * base_coin.scalar as f64).round() as u64;
        let base_coin_input = get_coins_to_transfer(
            &self.client,
            ptb,
            self.config.sender_address,
            &base_coin.coin_type,
            amount_input,
        )
        .await?;
        let deep_amount_input = (deep_amount * deep_coin.scalar as f64).round() as u64;

        let deep_coin_input = get_coins_to_transfer(
            &self.client,
            ptb,
            self.config.sender_address,
            &deep_coin.coin_type,
            deep_amount_input,
        )
        .await?;
        let min_out_input = ptb.pure((min_out * base_coin.scalar as f64).round() as u64)?;
        let clock_arg = ptb.input(CallArg::CLOCK_IMM)?;
        let swap_exact_base_for_quote =
            ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
                package: package_id,
                module: "pool".to_string(),
                function: "swap_exact_base_for_quote".to_string(),
                type_arguments: vec![
                    parse_type_input(&base_coin.coin_type)?,
                    parse_type_input(&quote_coin.coin_type)?,
                ],
                arguments: vec![
                    pool_object_arg,
                    base_coin_input,
                    deep_coin_input,
                    min_out_input,
                    clock_arg,
                ],
            })));
        let mut command_index: u16;
        match swap_exact_base_for_quote {
            Argument::Result(value) => {
                command_index = value;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected Result from borrow_flashloan_base",
                ));
            }
        }
        let base_coin_result = Argument::NestedResult(command_index, 0);
        let quote_coin_result = Argument::NestedResult(command_index, 1);
        let deep_coin_result = Argument::NestedResult(command_index, 2);
        Ok((base_coin_result, quote_coin_result, deep_coin_result))
    }
}
