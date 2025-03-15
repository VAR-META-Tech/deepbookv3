use core::borrow;

use sui_sdk::{
    SuiClient,
    types::{
        base_types::ObjectID,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{Argument, Command, ProgrammableMoveCall},
    },
};

use crate::utils::{config::DeepBookConfig, get_object_arg, parse_type_input};
use anyhow::{Context, Ok, Result, anyhow};

#[derive(Debug, Clone)]
pub struct FlashLoanContract {
    config: DeepBookConfig,
}

impl FlashLoanContract {
    pub fn new(config: DeepBookConfig) -> Self {
        Self { config }
    }
    pub async fn borrow_base_asset(
        &self,
        client: &SuiClient,
        pool_key: &str,
        amount: f64,
        ptb: &mut ProgrammableTransactionBuilder,
    ) -> Result<(Argument, Argument), anyhow::Error> {
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pools = self.config.get_pool(pool_key);

        let base_coin = self.config.get_coin(pools.base_coin);

        let quote_coin = self.config.get_coin(pools.quote_coin);
        let pool_object = get_object_arg(client, pools.address)
            .await
            .context("Failed to get object argument for pool")?;
        let type_argument_base_coin = parse_type_input(&base_coin.coin_type)?;

        let type_argument_quote_coin = parse_type_input(&quote_coin.coin_type)?;

        let pool_arg = ptb.input(pool_object)?;

        let amount_input = (amount * base_coin.scalar as f64) as u64;

        let amount_arg = ptb.pure(amount_input)?;

        let coin_and_flash_loan_return =
            ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
                package: package_id,
                module: "pool".to_string(),
                function: "borrow_flashloan_base".to_string(),
                type_arguments: vec![type_argument_base_coin, type_argument_quote_coin],
                arguments: vec![pool_arg, amount_arg],
            })));
        let mut command_index = 0u16;
        match coin_and_flash_loan_return {
            Argument::Result(value) => {
                command_index = value;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected Result from borrow_flashloan_base",
                ));
            }
        }
        let coin = Argument::NestedResult(command_index, 0);
        let flash_loan = Argument::NestedResult(command_index, 1);
        Ok((coin, flash_loan))
    }

    pub async fn return_flashloan_base(
        &self,
        client: &SuiClient,
        pool_key: &str,
        borrow_amount: f64,
        coin: Argument,
        flash_loan: Argument,
        ptb: &mut ProgrammableTransactionBuilder,
    ) -> Result<Argument, anyhow::Error> {
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pools = self.config.get_pool(pool_key);

        let base_coin = self.config.get_coin(pools.base_coin);

        let quote_coin = self.config.get_coin(pools.quote_coin);
        let pool_object = get_object_arg(client, pools.address)
            .await
            .context("Failed to get object argument for pool")?;
        let type_argument_base_coin = parse_type_input(&base_coin.coin_type)?;

        let type_argument_quote_coin = parse_type_input(&quote_coin.coin_type)?;

        let pool_arg = ptb.input(pool_object)?;

        let borrow_amount_input = (borrow_amount * base_coin.scalar as f64) as u64;

        let split_borrow_amount = ptb.pure(borrow_amount_input)?;

        let base_coin_return = ptb.command(Command::SplitCoins(coin, vec![split_borrow_amount]));
        let mut command_index = 0u16;
        match base_coin_return {
            Argument::Result(value) => {
                command_index = value;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Failed to split coins for flash loan return"
                ))
                .context(format!("Unexpected argument type: {:?}", base_coin_return))?;
            }
        }
        let split_borrow_amount_argument = Argument::NestedResult(command_index, 0);
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "return_flashloan_base".to_string(),
            type_arguments: vec![type_argument_base_coin, type_argument_quote_coin],
            arguments: vec![pool_arg, split_borrow_amount_argument, flash_loan],
        })));

        Ok(coin)
    }

    pub async fn borrow_quote_asset(
        &self,
        client: &SuiClient,
        pool_key: &str,
        amount: f64,
        ptb: &mut ProgrammableTransactionBuilder,
    ) -> Result<(Argument, Argument), anyhow::Error> {
        let package_id: ObjectID = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pools = self.config.get_pool(pool_key);

        let base_coin = self.config.get_coin(pools.base_coin);

        let quote_coin = self.config.get_coin(pools.quote_coin);
        let pool_object = get_object_arg(client, pools.address)
            .await
            .context("Failed to get object argument for pool")?;
        let type_argument_base_coin = parse_type_input(&base_coin.coin_type)?;

        let type_argument_quote_coin = parse_type_input(&quote_coin.coin_type)?;

        let pool_arg = ptb.input(pool_object)?;

        let amount_input = (amount * base_coin.scalar as f64) as u64;

        let amount_arg = ptb.pure(amount_input)?;

        let coin_and_flash_loan_return =
            ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
                package: package_id,
                module: "pool".to_string(),
                function: "borrow_flashloan_quote".to_string(),
                type_arguments: vec![type_argument_base_coin, type_argument_quote_coin],
                arguments: vec![pool_arg, amount_arg],
            })));
        let mut command_index = 0u16;
        match coin_and_flash_loan_return {
            Argument::Result(value) => {
                command_index = value;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected Result from borrow_flashloan_base",
                ));
            }
        }
        let coin = Argument::NestedResult(command_index, 0);
        let flash_loan = Argument::NestedResult(command_index, 1);
        Ok((coin, flash_loan))
    }

    pub async fn return_flashloan_quote(
        &self,
        client: &SuiClient,
        pool_key: &str,
        borrow_amount: f64,
        coin: Argument,
        flash_loan: Argument,
        ptb: &mut ProgrammableTransactionBuilder,
    ) -> Result<Argument, anyhow::Error> {
        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pools = self.config.get_pool(pool_key);

        let base_coin = self.config.get_coin(pools.base_coin);

        let quote_coin = self.config.get_coin(pools.quote_coin);
        let pool_object = get_object_arg(client, pools.address)
            .await
            .context("Failed to get object argument for pool")?;
        let type_argument_base_coin = parse_type_input(&base_coin.coin_type)?;

        let type_argument_quote_coin = parse_type_input(&quote_coin.coin_type)?;

        let pool_arg = ptb.input(pool_object)?;

        let borrow_amount_input = (borrow_amount * base_coin.scalar as f64) as u64;

        let split_borrow_amount = ptb.pure(borrow_amount_input)?;

        let base_coin_return = ptb.command(Command::SplitCoins(coin, vec![split_borrow_amount]));
        let mut command_index = 0u16;
        match base_coin_return {
            Argument::Result(value) => {
                command_index = value;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Failed to split coins for flash loan return"
                ))
                .context(format!("Unexpected argument type: {:?}", base_coin_return))?;
            }
        }
        let split_borrow_amount_argument = Argument::NestedResult(command_index, 0);
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "return_flashloan_quote".to_string(),
            type_arguments: vec![type_argument_base_coin, type_argument_quote_coin],
            arguments: vec![pool_arg, split_borrow_amount_argument, flash_loan],
        })));

        Ok(coin)
    }
}
