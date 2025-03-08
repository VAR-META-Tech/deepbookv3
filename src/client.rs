use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::SuiClient;

use crate::transactions::balance_manager::BalanceManagerContract;
use crate::types::{BalanceManager, Coin, Pool};
use crate::utils::config::DeepBookConfig;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DeepBookClient {
    client: SuiClient,
    config: DeepBookConfig,
    address: String,
    balance_manager: BalanceManagerContract,
    // deep_book: DeepBookContract,
    // deep_book_admin: DeepBookAdminContract,
    // flash_loans: FlashLoanContract,
    // governance: GovernanceContract,
}

impl DeepBookClient {
    pub fn new(
        client: SuiClient,
        address: String,
        env: &str,
        balance_managers: Option<HashMap<String, BalanceManager>>,
        coins: Option<HashMap<String, Coin>>,
        pools: Option<HashMap<String, Pool>>,
        admin_cap: Option<String>,
    ) -> Self {
        let config = DeepBookConfig::new(
            env,
            address.clone(),
            admin_cap.clone(),
            balance_managers,
            coins,
            pools,
        );

        Self {
            client,
            config: config.clone(),
            address,
            balance_manager: BalanceManagerContract::new(config),
            // deep_book: DeepBookContract::new(config.clone()),
            // deep_book_admin: DeepBookAdminContract::new(config.clone()),
            // flash_loans: FlashLoanContract::new(config.clone()),
            // governance: GovernanceContract::new(config.clone()),
        }
    }

    // pub async fn check_manager_balance(
    //     &self,
    //     manager_key: &str,
    //     coin_key: &str,
    // ) -> Result<(String, f64), Box<dyn std::error::Error>> {
    //     Ok(("Type".to_string(), 0.0))
    // }

    // pub fn check_manager_balance(
    //     &self,
    //     manager_key: &str,
    //     coin_key: &str,
    // ) -> ProgrammableTransactionBuilder {
    //     let mut builder = ProgrammableTransactionBuilder::new();
    //     let manager_id = self.config.get_balance_manager(manager_key).address.clone();
    //     let coin = self.config.get_coin(coin_key);

    //     builder.move_call(
    //         self.config.deepbook_package_id.clone(),
    //         "balance_manager".to_string(),
    //         "balance".to_string(),
    //         vec![coin.coin_type.clone()],
    //         vec![manager_id.into()],
    //     );

    //     builder
    // }
}
