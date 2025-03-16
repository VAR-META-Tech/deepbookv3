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
            flash_loans: FlashLoanContract::new(config.clone()),
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
}
