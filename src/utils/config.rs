// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use std::collections::HashMap;

use crate::transactions::balance_manager::BalanceManagerContract;
use crate::types::{BalanceManager, Coin, Pool};
use sui_sdk::types::base_types::SuiAddress;

use crate::utils::constants::{
    MAINNET_PACKAGE_IDS, TESTNET_PACKAGE_IDS, get_mainnet_coins, get_mainnet_pools,
    get_testnet_coins, get_testnet_pools,
};

use super::constants::{DEVNET_PACKAGE_IDS, get_devnet_coins, get_devnet_pools};

pub const FLOAT_SCALAR: f64 = 1_000_000_000.0;
pub const MAX_TIMESTAMP: u64 = 1_844_674_407_370_955_161;
pub const GAS_BUDGET: f64 = 0.5 * 500_000_000.0; // Adjust based on benchmarking
pub const DEEP_SCALAR: f64 = 1_000_000.0;

#[derive(Debug, Clone)]
pub struct DeepBookConfig {
    coins: HashMap<String, Coin>,
    pools: HashMap<String, Pool>,
    balance_managers: HashMap<String, BalanceManager>,
    pub sender_address: SuiAddress,

    pub deepbook_package_id: String,
    pub registry_id: String,
    pub deep_treasury_id: String,
    pub admin_cap: Option<String>,
    // pub balance_manager: BalanceManagerContract,
}

impl DeepBookConfig {
    pub fn new(
        env: &str,
        sender_address: SuiAddress,
        admin_cap: Option<String>,
        balance_managers: Option<HashMap<String, BalanceManager>>,
        coins: Option<HashMap<String, Coin>>,
        pools: Option<HashMap<String, Pool>>,
    ) -> Self {
        let balance_managers = balance_managers.unwrap_or_else(HashMap::new);

        let (coins, pools, package_ids) = if env == "mainnet" {
            (
                coins.unwrap_or_else(|| {
                    get_mainnet_coins()
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect()
                }),
                pools.unwrap_or_else(|| {
                    get_mainnet_pools()
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect()
                }),
                &MAINNET_PACKAGE_IDS,
            )
        } else if env == "testnet" {
            (
                coins.unwrap_or_else(|| {
                    get_testnet_coins()
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect()
                }),
                pools.unwrap_or_else(|| {
                    get_testnet_pools()
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect()
                }),
                &TESTNET_PACKAGE_IDS,
            )
        } else {
            (
                coins.unwrap_or_else(|| {
                    get_devnet_coins()
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect()
                }),
                pools.unwrap_or_else(|| {
                    get_devnet_pools()
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect()
                }),
                &DEVNET_PACKAGE_IDS,
            )
        };

        Self {
            coins,
            pools,
            balance_managers,
            sender_address: sender_address,
            deepbook_package_id: package_ids.deepbook_package_id.clone().to_owned(),
            registry_id: package_ids.registry_id.clone().to_owned(),
            deep_treasury_id: package_ids.deep_treasury_id.clone().to_owned(),
            admin_cap,
            // balance_manager: BalanceManagerContract::new(),
        }
    }

    pub fn get_coin(&self, key: &str) -> &Coin {
        self.coins
            .get(key)
            .expect(&format!("Coin not found for key: {}", key))
    }

    pub fn get_pool(&self, key: &str) -> &Pool {
        self.pools
            .get(key)
            .expect(&format!("Pool not found for key: {}", key))
    }

    pub fn get_balance_manager(&self, key: &str) -> &BalanceManager {
        self.balance_managers
            .get(key)
            .expect(&format!("Balance manager with key {} not found.", key))
    }
}

fn normalize_sui_address(address: String) -> String {
    // Placeholder function for address normalization
    address.to_lowercase()
}
