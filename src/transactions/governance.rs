use anyhow::{Context, Result};
use sui_sdk::SuiClient;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::transaction::{Command, ProgrammableMoveCall};

use crate::utils::config::{DEEP_SCALAR, DeepBookConfig, FLOAT_SCALAR};
use crate::utils::{get_object_arg, parse_type_input};

use super::balance_manager::BalanceManagerContract;

#[derive(Clone)]
pub struct GovernanceContract {
    client: SuiClient,
    config: DeepBookConfig,
    balance_manager: BalanceManagerContract,
}

impl GovernanceContract {
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

    /// Stake a specified amount in the pool
    pub async fn stake(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        balance_manager_key: &str,
        stake_amount: f64,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let balance_manager = self.config.get_balance_manager(balance_manager_key);
        let trade_proof_arg = self
            .balance_manager
            .generate_proof(ptb, balance_manager_key)
            .await?;

        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);
        let stake_input = (stake_amount * DEEP_SCALAR as f64) as u64;

        let pool_object = get_object_arg(&self.client, &pool.address).await?;
        let manager_object = get_object_arg(&self.client, &balance_manager.address).await?;
        let stake_input_arg = ptb.pure(stake_input)?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;
        let pool_object_arg = ptb.input(pool_object)?;
        let manager_object_arg = ptb.input(manager_object)?;
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "stake".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![
                pool_object_arg,
                manager_object_arg,
                trade_proof_arg,
                stake_input_arg,
            ],
        })));

        Ok(())
    }

    /// Unstake from the pool
    pub async fn unstake(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        balance_manager_key: &str,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let balance_manager = self.config.get_balance_manager(balance_manager_key);
        let trade_proof_arg = self
            .balance_manager
            .generate_proof(ptb, balance_manager_key)
            .await?;

        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address).await?;
        let manager_object = get_object_arg(&self.client, &balance_manager.address).await?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_arg = ptb.input(pool_object)?;
        let manager_object_arg = ptb.input(manager_object)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "unstake".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_arg, manager_object_arg, trade_proof_arg],
        })));

        Ok(())
    }

    /// Submit a governance proposal
    pub async fn submit_proposal(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        balance_manager_key: &str,
        taker_fee: f64,
        maker_fee: f64,
        stake_required: f64,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let balance_manager = self.config.get_balance_manager(balance_manager_key);
        let trade_proof_arg = self
            .balance_manager
            .generate_proof(ptb, balance_manager_key)
            .await?;

        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let taker_fee_input = (taker_fee * FLOAT_SCALAR) as u64;
        let maker_fee_input = (maker_fee * FLOAT_SCALAR) as u64;
        let stake_required_input = (stake_required * DEEP_SCALAR) as u64;

        let pool_object = get_object_arg(&self.client, &pool.address).await?;
        let manager_object = get_object_arg(&self.client, &balance_manager.address).await?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;
        let pool_object_arg = ptb.input(pool_object)?;
        let manager_object_arg = ptb.input(manager_object)?;
        let taker_fee_input_arg = ptb.pure(taker_fee_input)?;
        let maker_fee_input_arg = ptb.pure(maker_fee_input)?;
        let stake_required_input_arg = ptb.pure(stake_required_input)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "submit_proposal".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![
                pool_object_arg,
                manager_object_arg,
                trade_proof_arg,
                taker_fee_input_arg,
                maker_fee_input_arg,
                stake_required_input_arg,
            ],
        })));

        Ok(())
    }

    /// Vote on a governance proposal
    pub async fn vote(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        balance_manager_key: &str,
        proposal_id: &str,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let balance_manager = self.config.get_balance_manager(balance_manager_key);
        let trade_proof_arg = self
            .balance_manager
            .generate_proof(ptb, balance_manager_key)
            .await?;

        let base_coin = self.config.get_coin(&pool.base_coin);
        let quote_coin = self.config.get_coin(&pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address).await?;
        let manager_object = get_object_arg(&self.client, &balance_manager.address).await?;
        let proposal_id_arg = ptb.pure(proposal_id)?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;
        let pool_object_arg = ptb.input(pool_object)?;
        let manager_object_arg = ptb.input(manager_object)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "vote".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![
                pool_object_arg,
                manager_object_arg,
                trade_proof_arg,
                proposal_id_arg,
            ],
        })));

        Ok(())
    }
}
