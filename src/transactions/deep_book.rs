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
}
