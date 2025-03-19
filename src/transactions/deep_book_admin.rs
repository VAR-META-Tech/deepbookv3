use anyhow::{Context, Result};
use sui_sdk::SuiClient;
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::transaction::{Command, ProgrammableMoveCall};

use crate::utils::config::{DeepBookConfig, FLOAT_SCALAR};
use crate::utils::{get_object_arg, parse_type_input};

#[derive(Clone)]
pub struct DeepBookAdminContract {
    client: SuiClient,
    config: DeepBookConfig,
}

impl DeepBookAdminContract {
    pub fn new(client: SuiClient, config: DeepBookConfig) -> Self {
        Self { client, config }
    }

    /// Fetches the admin capability
    fn admin_cap(&self) -> Result<String> {
        self.config
            .admin_cap
            .as_ref()
            .map(|cap| cap.clone())
            .ok_or_else(|| anyhow::anyhow!("ADMIN_CAP environment variable not set"))
    }

    /// Create a new pool as an admin
    pub async fn create_pool_admin(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        base_coin_key: &str,
        quote_coin_key: &str,
        tick_size: f64,
        lot_size: f64,
        min_size: f64,
        whitelisted: bool,
        stable_pool: bool,
    ) -> Result<()> {
        let base_coin = self.config.get_coin(base_coin_key);
        let quote_coin = self.config.get_coin(quote_coin_key);

        let base_scalar = base_coin.scalar as f64;
        let quote_scalar = quote_coin.scalar as f64;

        let adjusted_tick_size = ((tick_size * FLOAT_SCALAR * quote_scalar) / base_scalar) as u64;
        let adjusted_lot_size = (lot_size * base_scalar) as u64;
        let adjusted_min_size = (min_size * base_scalar) as u64;

        let registry_id = get_object_arg(&self.client, &self.config.registry_id).await?;
        let admin_cap = get_object_arg(&self.client, &self.admin_cap()?).await?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;
        let registry_input = ptb.input(registry_id)?;
        let admin_cap_input = ptb.input(admin_cap)?;
        let adjusted_tick_size_arg = ptb.pure(adjusted_tick_size)?;
        let adjusted_lot_size_arg = ptb.pure(adjusted_lot_size)?;
        let adjusted_min_size_arg = ptb.pure(adjusted_min_size)?;
        let whitelisted_arg = ptb.pure(whitelisted)?;
        let stable_pool_arg = ptb.pure(stable_pool)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "create_pool_admin".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![
                registry_input,
                adjusted_tick_size_arg,
                adjusted_lot_size_arg,
                adjusted_min_size_arg,
                whitelisted_arg,
                stable_pool_arg,
                admin_cap_input,
            ],
        })));

        Ok(())
    }

    /// Unregister a pool as an admin
    pub async fn unregister_pool_admin(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(pool.base_coin);
        let quote_coin = self.config.get_coin(pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address).await?;
        let registry_id = get_object_arg(&self.client, &self.config.registry_id).await?;
        let admin_cap = get_object_arg(&self.client, &self.admin_cap()?).await?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_input = ptb.input(pool_object)?;
        let registry_id_input = ptb.input(registry_id)?;
        let admin_cap_input = ptb.input(admin_cap)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "unregister_pool_admin".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_input, registry_id_input, admin_cap_input],
        })));

        Ok(())
    }

    /// Update the allowed versions for a pool
    pub async fn update_allowed_versions(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key);
        let base_coin = self.config.get_coin(pool.base_coin);
        let quote_coin = self.config.get_coin(pool.quote_coin);

        let pool_object = get_object_arg(&self.client, &pool.address).await?;
        let registry_id = get_object_arg(&self.client, &self.config.registry_id).await?;
        let admin_cap = get_object_arg(&self.client, &self.admin_cap()?).await?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let pool_object_input = ptb.input(pool_object)?;
        let registry_id_input = ptb.input(registry_id)?;
        let admin_cap_input = ptb.input(admin_cap)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "pool".to_string(),
            function: "update_allowed_versions".to_string(),
            type_arguments: vec![
                parse_type_input(&base_coin.coin_type)?,
                parse_type_input(&quote_coin.coin_type)?,
            ],
            arguments: vec![pool_object_input, registry_id_input, admin_cap_input],
        })));

        Ok(())
    }

    /// Enable a specific version
    pub async fn enable_version(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        version: u64,
    ) -> Result<()> {
        let registry_id = get_object_arg(&self.client, &self.config.registry_id).await?;
        let admin_cap = get_object_arg(&self.client, &self.admin_cap()?).await?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let registry_id_input = ptb.input(registry_id)?;
        let version_input = ptb.pure(version)?;
        let admin_cap_input = ptb.input(admin_cap)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "registry".to_string(),
            function: "enable_version".to_string(),
            type_arguments: vec![],
            arguments: vec![registry_id_input, version_input, admin_cap_input],
        })));

        Ok(())
    }

    /// Disable a specific version
    pub async fn disable_version(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        version: u64,
    ) -> Result<()> {
        let registry_id = get_object_arg(&self.client, &self.config.registry_id).await?;
        let admin_cap = get_object_arg(&self.client, &self.admin_cap()?).await?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let registry_id_input = ptb.input(registry_id)?;
        let version_input = ptb.pure(version)?;
        let admin_cap_input = ptb.input(admin_cap)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "registry".to_string(),
            function: "disable_version".to_string(),
            type_arguments: vec![],
            arguments: vec![registry_id_input, version_input, admin_cap_input],
        })));

        Ok(())
    }

    /// Set the treasury address where pool creation fees will be sent
    pub async fn set_treasury_address(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        treasury_address: SuiAddress,
    ) -> Result<()> {
        let registry_id = get_object_arg(&self.client, &self.config.registry_id).await?;
        let admin_cap = get_object_arg(&self.client, &self.admin_cap()?).await?;

        let package_id = ObjectID::from_hex_literal(&self.config.deepbook_package_id)?;

        let registry_id_input = ptb.input(registry_id)?;
        let treasury_address_input = ptb.pure(treasury_address)?;
        let admin_cap_input = ptb.input(admin_cap)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: "registry".to_string(),
            function: "set_treasury_address".to_string(),
            type_arguments: vec![],
            arguments: vec![registry_id_input, treasury_address_input, admin_cap_input],
        })));

        Ok(())
    }
}
