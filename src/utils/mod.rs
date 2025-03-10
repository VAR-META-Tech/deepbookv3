use anyhow::{Context, Error, Result, anyhow};
use std::str::FromStr;
use sui_sdk::{
    SuiClient,
    rpc_types::{SuiObjectData, SuiObjectDataOptions, SuiObjectResponse},
    types::{
        TypeTag,
        base_types::{ObjectID, ObjectRef, SuiAddress},
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{Argument, CallArg, Command, ObjectArg},
        type_input::TypeInput,
    },
};

pub mod config;
pub mod constants;

pub fn parse_type_input(type_str: &str) -> Result<TypeInput, anyhow::Error> {
    let type_tag = TypeTag::from_str(type_str)?;
    Ok(TypeInput::from(type_tag))
}

pub async fn get_object_arg(client: &SuiClient, object_id: &str) -> Result<CallArg> {
    // Convert to ObjectID
    let object_id = ObjectID::from_hex_literal(object_id)?;

    // Fetch object data
    let object_response: SuiObjectResponse = client
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::full_content())
        .await?;

    // Extract object data
    let object_data = object_response.data.as_ref().ok_or_else(|| {
        Error::msg(format!(
            "Missing data in object response for '{}'",
            object_id
        ))
    })?;

    // Check if the object is shared
    match object_data.owner {
        Some(sui_sdk::types::object::Owner::Shared {
            initial_shared_version,
        }) => {
            // Return as Shared Object
            Ok(CallArg::Object(ObjectArg::SharedObject {
                id: object_id,
                initial_shared_version: initial_shared_version,
                mutable: true, // Assuming the object is mutable in the contract
            }))
        }
        _ => {
            // Create ObjectRef for owned object
            let object_ref: ObjectRef = (
                object_data.object_id,
                object_data.version,
                object_data.digest,
            );

            // Return as Owned Object
            Ok(CallArg::Object(ObjectArg::ImmOrOwnedObject(object_ref)))
        }
    }
}

/// Fetches a coin object with at least the required balance
pub async fn get_exact_coin(
    client: &SuiClient,
    owner: SuiAddress,
    coin_type: &str,
    amount: u64,
    ptb: &mut ProgrammableTransactionBuilder,
) -> Result<Argument> {
    let coins = client
        .coin_read_api()
        .get_coins(owner, Some(coin_type.to_string()), None, None)
        .await
        .context(format!("Failed to fetch coins for type {}", coin_type))?
        .data;

    // Find a coin that can cover the exact amount
    let coin = coins
        .iter()
        .find(|c| c.balance >= amount)
        .ok_or_else(|| anyhow!("No suitable coin found with required amount: {}", amount))?;

    let coin_ref = (coin.coin_object_id, coin.version, coin.digest);
    let coin_arg = ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_ref)))?;

    // Split the coin if it has more balance than required
    let exact_coin = if coin.balance > amount {
        let split_amount = ptb.input(CallArg::Pure(bcs::to_bytes(&amount)?))?;
        let split_coin = ptb.command(Command::SplitCoins(coin_arg, vec![split_amount]));
        split_coin
    } else {
        coin_arg
    };

    Ok(exact_coin)
}
