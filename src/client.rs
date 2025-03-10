use crate::transactions::balance_manager::BalanceManagerContract;
use crate::types::{BalanceManager, Coin, Pool};
use crate::utils::config::DeepBookConfig;
use anyhow::{Context, Result, anyhow};
use std::collections::HashMap;
use sui_sdk::SuiClient;
use sui_sdk::rpc_types::DevInspectResults;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::transaction::TransactionKind;

#[derive(Clone)]
pub struct DeepBookClient {
    client: SuiClient,
    config: DeepBookConfig,
    sender_address: SuiAddress,
    pub balance_manager: BalanceManagerContract,
    // deep_book: DeepBookContract,
    // deep_book_admin: DeepBookAdminContract,
    // flash_loans: FlashLoanContract,
    // governance: GovernanceContract,
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
            client,
            config: config.clone(),
            sender_address,
            balance_manager: BalanceManagerContract::new(config),
            // deep_book: DeepBookContract::new(config.clone()),
            // deep_book_admin: DeepBookAdminContract::new(config.clone()),
            // flash_loans: FlashLoanContract::new(config.clone()),
            // governance: GovernanceContract::new(config.clone()),
        }
    }

    pub async fn check_manager_balance(
        &self,
        manager_key: &str,
        coin_key: &str,
    ) -> Result<(String, u64)> {
        // Fetch coin type and manager ID
        let coin = self.config.get_coin(coin_key);
        let coin_type = coin.coin_type; // Clone to return as String

        let manager_id = self.config.get_balance_manager(manager_key).address;

        // Create transaction
        let pt = self
            .balance_manager
            .check_manager_balance(&self.client, manager_id, &coin_type)
            .await
            .context("Failed to create balance check transaction")?;

        // Execute transaction block
        let resp = self
            .client
            .read_api()
            .dev_inspect_transaction_block(
                self.sender_address,
                TransactionKind::programmable(pt),
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

        let adjusted_balance = balance / coin.scalar; // Adjust scaling factor

        Ok((coin_type.to_string(), adjusted_balance))
    }
}
