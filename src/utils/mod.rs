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
use sui_types::SUI_CLOCK_OBJECT_ID;

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

pub async fn get_clock_object_arg(client: &SuiClient) -> Result<CallArg, anyhow::Error> {
    let object_response: SuiObjectResponse = client
        .read_api()
        .get_object_with_options(SUI_CLOCK_OBJECT_ID, SuiObjectDataOptions::full_content())
        .await?;

    let object_data = object_response.data.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "Missing data in object response for '{}'",
            SUI_CLOCK_OBJECT_ID
        )
    })?;

    let res = match &object_data.owner {
        Some(sui_sdk::types::object::Owner::Shared {
            initial_shared_version,
        }) => {
            // Return as Shared Object
            CallArg::Object(ObjectArg::SharedObject {
                id: SUI_CLOCK_OBJECT_ID,
                initial_shared_version: *initial_shared_version,
                mutable: false,
            })
        }
        _ => {
            return Err(anyhow::anyhow!("Clock object is not a shared object"));
        }
    };

    Ok(res)
}

pub async fn merge_and_split_coins(
    client: &SuiClient,
    ptb: &mut ProgrammableTransactionBuilder,
    owner: SuiAddress,
    coin_type: &str,
    amounts: Vec<u64>, // Accept a list of amounts
) -> Result<Vec<Argument>> {
    if coin_type != "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI" {
        let coins = client
            .coin_read_api()
            .get_coins(owner, Some(coin_type.to_string()), None, None)
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch coins for type {} from {}",
                    coin_type, owner
                )
            })?
            .data;

        if coins.is_empty() {
            return Err(anyhow::anyhow!(
                "❌ No coins found for type {} under address {}",
                coin_type,
                owner
            ));
        }

        // Create input arguments
        let mut coin_arguments: Vec<Argument> = coins
            .iter()
            .map(|coin| {
                ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject((
                    coin.coin_object_id,
                    coin.version,
                    coin.digest,
                ))))
                .expect("❌ Failed to create input from coin object")
            })
            .collect();

        let merge_target = coin_arguments.remove(0);

        if !coin_arguments.is_empty() {
            println!("🔀 Merging {} coins into one...", coin_arguments.len() + 1);
            ptb.command(Command::MergeCoins(merge_target, coin_arguments));
        } else {
            println!("ℹ️ Only one coin available, skipping merge.");
        }

        // Convert amounts to pure arguments
        let amount_args: Result<Vec<Argument>> = amounts.iter().map(|amt| ptb.pure(*amt)).collect();
        let amount_args = amount_args?;

        // Split coins
        let split_result = ptb.command(Command::SplitCoins(merge_target, amount_args));

        match split_result {
            Argument::Result(idx) => {
                let outputs = (0..amounts.len())
                    .map(|i| Argument::NestedResult(idx, i as u16))
                    .collect::<Vec<_>>();
                Ok(outputs)
            }
            _ => Err(anyhow::anyhow!("Expected Result from SplitCoins")),
        }
    } else {
        // Use GasCoin for SUI
        println!("⚙️ Using GasCoin for SUI transfer.");
        let amount_args: Result<Vec<Argument>> = amounts.iter().map(|amt| ptb.pure(*amt)).collect();
        let split_result = ptb.command(Command::SplitCoins(Argument::GasCoin, amount_args?));
        match split_result {
            Argument::Result(idx) => {
                let outputs = (0..amounts.len())
                    .map(|i| Argument::NestedResult(idx, i as u16))
                    .collect::<Vec<_>>();
                Ok(outputs)
            }
            _ => Err(anyhow::anyhow!("Expected Result from SplitCoins (GasCoin)")),
        }
    }
}
