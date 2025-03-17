use crate::transactions::balance_manager::{self, BalanceManagerContract};
use crate::transactions::deep_book::DeepBookContract;
use crate::transactions::deep_book_admin::DeepBookAdminContract;
use crate::transactions::flash_loans::FlashLoanContract;
use crate::transactions::governance::GovernanceContract;
use crate::types::{Account, BalanceManager, Coin, OrderDeepPrice, Pool};
use crate::utils::config::DeepBookConfig;
use anyhow::{Context, Result, anyhow};
use std::any;
use std::collections::HashMap;
use sui_sdk::SuiClient;
use sui_sdk::rpc_types::DevInspectResults;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::collection_types::VecSet;
use sui_sdk::types::id::ID;
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::storage::AccountOwnedObjectInfo;
use sui_sdk::types::transaction::TransactionKind;

#[derive(Clone)]
pub struct DeepBookClient {
    client: SuiClient,
    config: DeepBookConfig,
    sender_address: SuiAddress,
    pub balance_manager: BalanceManagerContract,
    pub deep_book: DeepBookContract,
    pub deep_book_admin: DeepBookAdminContract,
    pub flash_loans: FlashLoanContract,
    pub governance: GovernanceContract,
}

impl DeepBookClient {
    pub fn new(
        client: SuiClient,
        sender_address: SuiAddress,
        env: &str,
        balance_managers: Option<HashMap<String, BalanceManager>>,
        coins: Option<HashMap<String, Coin>>,
        pools: Option<HashMap<String, Pool>>,
        admin_cap: Option<String>,
    ) -> Self {
        let config = DeepBookConfig::new(
            env,
            sender_address.clone(),
            admin_cap.clone(),
            balance_managers,
            coins,
            pools,
        );
        Self {
            client: client.clone(),
            config: config.clone(),
            sender_address,
            balance_manager: BalanceManagerContract::new(client.clone(), config.clone()),
            deep_book: DeepBookContract::new(
                client.clone(),
                config.clone(),
                BalanceManagerContract::new(client.clone(), config.clone()),
            ),
            deep_book_admin: DeepBookAdminContract::new(client.clone(), config.clone()),
            flash_loans: FlashLoanContract::new(client.clone(), config.clone()),
            governance: GovernanceContract::new(
                client.clone(),
                config.clone(),
                BalanceManagerContract::new(client.clone(), config.clone()),
            ),
        }
    }

    pub async fn check_manager_balance(
        &self,
        manager_key: &str,
        coin_key: &str,
    ) -> Result<(String, f64)> {
        // Fetch coin type and manager ID
        let coin = self.config.get_coin(coin_key);
        let coin_type = coin.coin_type.clone(); // Clone to return as String

        // Create transaction
        let pt = self
            .balance_manager
            .check_manager_balance(manager_key, &coin_key)
            .await
            .context("Failed to create balance check transaction")?;

        // Execute transaction block
        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(pt.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        // Extract transaction results
        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for balance check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for balance check"))?;

        let (value_bytes, _type_tag) = return_values;

        // Decode balance value
        let balance: u64 = bcs::from_bytes(value_bytes)
            .context("Failed to decode balance from transaction response")?;

        // Convert balance to `f64` using the correct scaling factor
        let adjusted_balance = balance as f64 / coin.scalar as f64;

        Ok((coin_type.to_string(), adjusted_balance))
    }

    pub async fn get_manager_owner(&self, manager_key: &str) -> Result<SuiAddress> {
        let pt = self
            .balance_manager
            .get_manager_owner(manager_key)
            .await
            .context("Failed to create owner retrieval transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(pt.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for owner check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for manager owner check"))?;

        let (value_bytes, _type_tag) = return_values;
        let owner: SuiAddress = bcs::from_bytes(value_bytes)
            .context("Failed to decode owner address from transaction response")?;
        Ok(owner)
    }

    /// ✅ **Get Manager ID**
    pub async fn get_manager_id(&self, manager_key: &str) -> Result<ID> {
        let pt = self
            .balance_manager
            .get_manager_id(manager_key)
            .await
            .context("Failed to create ID retrieval transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(pt.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results
            .ok_or_else(|| anyhow!("No results returned for ID check, effects: {:?}", effects))?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for manager ID check"))?;

        let (value_bytes, _type_tag) = return_values;
        let manager_id: ID = bcs::from_bytes(value_bytes)
            .context("Failed to decode manager ID from transaction response")?;

        Ok(manager_id)
    }

    pub async fn get_account(&self, pool_key: &str, manager_key: &str) -> Result<Account> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .account(&mut ptb, pool_key, manager_key)
            .await
            .context("Failed to create owner retrieval transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for owner check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for manager owner check"))?;

        let (value_bytes, _type_tag) = return_values;
        let data: Account = bcs::from_bytes(value_bytes)
            .context("Failed to decode account address from transaction response")?;
        Ok(data)
    }

    pub async fn get_locked_balance(&self, pool_key: &str, manager_key: &str) -> Result<u64> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .locked_balance(&mut ptb, pool_key, manager_key)
            .await
            .context("Failed to create locked balance retrieval transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for locked balance check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for locked balance retrieval"))?;

        let (value_bytes, _type_tag) = return_values;
        let data: u64 = bcs::from_bytes(value_bytes)
            .context("Failed to decode locked balance from transaction response")?;

        Ok(data)
    }

    pub async fn get_pool_deep_price(&self, pool_key: &str) -> Result<OrderDeepPrice> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .get_pool_deep_price(&mut ptb, pool_key)
            .await
            .context("Failed to create locked balance retrieval transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for locked balance check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for locked balance retrieval"))?;

        let (value_bytes, _type_tag) = return_values;

        let data: OrderDeepPrice = bcs::from_bytes(value_bytes)
            .context("Failed to decode locked balance from transaction response")?;

        Ok(data)
    }

    pub async fn get_pool_book_params(&self, pool_key: &str) -> Result<(u64, u64, u64)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // Generate the transaction to fetch pool book parameters
        self.deep_book
            .pool_book_params(&mut ptb, pool_key)
            .await
            .context("Failed to create pool book params retrieval transaction")?;

        // Execute the transaction in dev inspect mode
        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        // Check if results are returned
        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for pool book params, effects: {:?}",
                effects
            )
        })?;

        // Extract return values
        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .as_slice(); // Get the entire array

        // Ensure the array contains exactly 3 elements
        if return_values.len() != 3 {
            return Err(anyhow!(
                "Unexpected number of return values for pool book params: expected 3, got {}",
                return_values.len()
            ));
        }

        // Decode each element separately
        let tick_size: u64 = bcs::from_bytes(&return_values[0].0)
            .context("Failed to decode tick size from transaction response")?;
        let lot_size: u64 = bcs::from_bytes(&return_values[1].0)
            .context("Failed to decode lot size from transaction response")?;
        let min_size: u64 = bcs::from_bytes(&return_values[2].0)
            .context("Failed to decode min size from transaction response")?;

        Ok((tick_size, lot_size, min_size))
    }

    pub async fn get_pool_trade_params(&self, pool_key: &str) -> Result<(u64, u64, u64)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // Generate the transaction to fetch pool trade parameters
        self.deep_book
            .pool_trade_params(&mut ptb, pool_key)
            .await
            .context("Failed to create pool trade params retrieval transaction")?;

        // Execute the transaction in dev inspect mode
        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        // Check if results are returned
        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for pool trade params, effects: {:?}",
                effects
            )
        })?;

        // Extract return values
        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .as_slice(); // Get the entire array

        // Ensure the array contains exactly 3 elements
        if return_values.len() != 3 {
            return Err(anyhow!(
                "Unexpected number of return values for pool trade params: expected 3, got {}",
                return_values.len()
            ));
        }

        // Decode each element separately
        let taker_fee: u64 = bcs::from_bytes(&return_values[0].0)
            .context("Failed to decode taker fee from transaction response")?;
        let maker_fee: u64 = bcs::from_bytes(&return_values[1].0)
            .context("Failed to decode maker fee from transaction response")?;
        let stake_required: u64 = bcs::from_bytes(&return_values[2].0)
            .context("Failed to decode stake required from transaction response")?;

        Ok((taker_fee, maker_fee, stake_required))
    }

    pub async fn get_pool_id_by_assets(&self, base_type: &str, quote_type: &str) -> Result<ID> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .get_pool_id_by_assets(&mut ptb, base_type, quote_type)
            .await
            .context("Failed to create pool ID retrieval transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        println!("Results: {:?}", results);
        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for pool ID check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for pool ID retrieval"))?;

        let (value_bytes, _type_tag) = return_values;
        let pool_id: ID = bcs::from_bytes(value_bytes)
            .context("Failed to decode pool ID from transaction response")?;

        Ok(pool_id)
    }

    pub async fn get_vault_balances(&self, pool_key: &str) -> Result<(u64, u64, u64)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .vault_balances(&mut ptb, pool_key)
            .await
            .context("Failed to create vault balances retrieval transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for vault balances check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .as_slice(); // Get the entire array

        if return_values.len() != 3 {
            return Err(anyhow!(
                "Unexpected number of return values for vault balances: expected 3, got {}",
                return_values.len()
            ));
        }

        let base_balance: u64 = bcs::from_bytes(&return_values[0].0)
            .context("Failed to decode base balance from transaction response")?;
        let quote_balance: u64 = bcs::from_bytes(&return_values[1].0)
            .context("Failed to decode quote balance from transaction response")?;
        let deep_balance: u64 = bcs::from_bytes(&return_values[2].0)
            .context("Failed to decode deep balance from transaction response")?;

        Ok((base_balance, quote_balance, deep_balance))
    }
    pub async fn get_level2_ticks_from_mid(
        &self,
        pool_key: &str,
        tick_from_mid: u64,
    ) -> Result<(Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .get_level2_ticks_from_mid(&mut ptb, pool_key, tick_from_mid)
            .await
            .context("Failed to create level2 ticks retrieval transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for level2 ticks check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .as_slice(); // Get the entire array

        // 🚨 Debugging Output: Check return values 🚨
        println!("Return values count: {}", return_values.len());

        // Ensure we have exactly 4 vectors returned
        if return_values.len() != 4 {
            return Err(anyhow!(
                "Unexpected number of return values for level2 ticks: expected 4, got {}",
                return_values.len()
            ));
        }

        // ✅ Decode each vector<u64> separately
        let bid_prices: Vec<u64> = bcs::from_bytes(&return_values[0].0)
            .context("Failed to decode bid prices from transaction response")?;
        let bid_sizes: Vec<u64> = bcs::from_bytes(&return_values[1].0)
            .context("Failed to decode bid sizes from transaction response")?;
        let ask_prices: Vec<u64> = bcs::from_bytes(&return_values[2].0)
            .context("Failed to decode ask prices from transaction response")?;
        let ask_sizes: Vec<u64> = bcs::from_bytes(&return_values[3].0)
            .context("Failed to decode ask sizes from transaction response")?;

        Ok((bid_prices, bid_sizes, ask_prices, ask_sizes))
    }

    pub async fn get_level2_range(
        &self,
        pool_key: &str,
        price_low: f64,
        price_high: f64,
        is_bid: bool,
    ) -> Result<(Vec<u64>, Vec<u64>)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // Generate the transaction to fetch Level 2 order book range
        self.deep_book
            .get_level2_range(&mut ptb, pool_key, price_low, price_high, is_bid)
            .await
            .context("Failed to create Level 2 range retrieval transaction")?;

        // Execute the transaction in dev inspect mode
        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        // Ensure the response has results
        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for Level 2 order book range, effects: {:?}",
                effects
            )
        })?;

        // Extract return values
        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .as_slice(); // Get the entire array

        // Ensure the array contains exactly 2 elements (prices and volumes)
        if return_values.len() != 2 {
            return Err(anyhow!(
                "Unexpected number of return values for Level 2 order book range: expected 2, got {}",
                return_values.len()
            ));
        }

        // Decode each element separately
        let price_levels: Vec<u64> = bcs::from_bytes(&return_values[0].0)
            .context("Failed to decode price levels from transaction response")?;
        let volume_levels: Vec<u64> = bcs::from_bytes(&return_values[1].0)
            .context("Failed to decode volume levels from transaction response")?;

        Ok((price_levels, volume_levels))
    }

    pub async fn get_account_open_orders(
        &self,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<VecSet<u128>> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        // Build transaction for retrieving open orders
        self.deep_book
            .account_open_orders(&mut ptb, pool_key, manager_key)
            .await
            .context("Failed to create account open orders transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for account open orders, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for account open orders"))?;

        let (value_bytes, _type_tag) = return_values;

        // Deserialize into `VecSet<u128>`
        let open_orders: VecSet<u128> = bcs::from_bytes(value_bytes)
            .context("Failed to decode open orders from transaction response")?;

        Ok(open_orders)
    }

    pub async fn get_quantity_out(
        &self,
        pool_key: &str,
        base_quantity: f64,
        quote_quantity: f64,
    ) -> Result<(u64, u64, u64)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .get_quantity_out(&mut ptb, pool_key, base_quantity, quote_quantity)
            .await
            .context("Failed to create get_quantity_out transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for get_quantity_out, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .as_slice(); // Get the entire array

        // Ensure the array contains exactly 3 elements
        if return_values.len() != 3 {
            return Err(anyhow!(
                "Unexpected number of return values for get_quantity_out: expected 3, got {}",
                return_values.len()
            ));
        }

        // Decode each element separately
        let base_quantity_out: u64 = bcs::from_bytes(&return_values[0].0)
            .context("Failed to decode output base quantity from transaction response")?;
        let quote_quantity_out: u64 = bcs::from_bytes(&return_values[1].0)
            .context("Failed to decode output quote quantity from transaction response")?;
        let deep_quantity_required: u64 = bcs::from_bytes(&return_values[2].0)
            .context("Failed to decode execution price from transaction response")?;

        Ok((
            base_quantity_out,
            quote_quantity_out,
            deep_quantity_required,
        ))
    }

    pub async fn get_base_quantity_out(
        &self,
        pool_key: &str,
        quote_quantity: f64,
    ) -> Result<(u64, u64, u64)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .get_base_quantity_out(&mut ptb, pool_key, quote_quantity)
            .await
            .context("Failed to create get_base_quantity_out transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for get_base_quantity_out, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .as_slice();

        // Ensure we have exactly 3 return values
        if return_values.len() != 3 {
            return Err(anyhow!(
                "Unexpected number of return values for get_base_quantity_out: expected 3, got {}",
                return_values.len()
            ));
        }

        // Decode each value separately
        let base_quantity: u64 = bcs::from_bytes(&return_values[0].0)
            .context("Failed to decode base quantity from transaction response")?;
        let quote_quantity_out: u64 = bcs::from_bytes(&return_values[1].0)
            .context("Failed to decode additional data 1 from transaction response")?;
        let deep_quantity_required: u64 = bcs::from_bytes(&return_values[2].0)
            .context("Failed to decode additional data 2 from transaction response")?;

        Ok((base_quantity, quote_quantity_out, deep_quantity_required))
    }

    pub async fn get_quote_quantity_out(
        &self,
        pool_key: &str,
        base_quantity: f64,
    ) -> Result<(u64, u64, u64)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .get_quote_quantity_out(&mut ptb, pool_key, base_quantity)
            .await
            .context("Failed to create get_quote_quantity_out transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute dev inspect transaction block")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for get_quote_quantity_out, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .as_slice();

        // Ensure we have exactly 3 return values
        if return_values.len() != 3 {
            return Err(anyhow!(
                "Unexpected number of return values for get_quote_quantity_out: expected 3, got {}",
                return_values.len()
            ));
        }

        // Decode each value separately
        let quote_quantity: u64 = bcs::from_bytes(&return_values[0].0)
            .context("Failed to decode quote quantity from transaction response")?;
        let base_quantity_out: u64 = bcs::from_bytes(&return_values[1].0)
            .context("Failed to decode base quantity out from transaction response")?;
        let deep_quantity_required: u64 = bcs::from_bytes(&return_values[2].0)
            .context("Failed to decode deep quantity required from transaction response")?;

        Ok((quote_quantity, base_quantity_out, deep_quantity_required))
    }

    pub async fn get_whitelisted_status(&self, pool_key: &str) -> Result<bool> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .whitelisted(&mut ptb, pool_key)
            .await
            .context("Failed to create whitelist status transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute whitelist status transaction")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for whitelisted status check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for whitelisted status check"))?;

        let (value_bytes, _type_tag) = return_values;

        // Decode response as boolean
        let is_whitelisted: bool = bcs::from_bytes(value_bytes)
            .context("Failed to decode whitelisted status from transaction response")?;

        Ok(is_whitelisted)
    }

    pub async fn get_mid_price(&self, pool_key: &str) -> Result<u64> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        self.deep_book
            .mid_price(&mut ptb, pool_key)
            .await
            .context("Failed to create mid price transaction")?;

        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            )
            .await
            .context("Failed to execute mid price transaction")?;

        let DevInspectResults {
            results, effects, ..
        } = resp;

        let results = results.ok_or_else(|| {
            anyhow!(
                "No results returned for mid price check, effects: {:?}",
                effects
            )
        })?;

        let return_values = results
            .first()
            .ok_or_else(|| anyhow!("No return values found in transaction results"))?
            .return_values
            .first()
            .ok_or_else(|| anyhow!("No return value found for mid price check"))?;

        let (value_bytes, _type_tag) = return_values;

        // Decode response as u64 (mid price)
        let mid_price: u64 = bcs::from_bytes(value_bytes)
            .context("Failed to decode mid price from transaction response")?;

        Ok(mid_price)
    }
}
