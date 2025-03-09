use anyhow::{Error, Result};
use std::str::FromStr;
use sui_sdk::{
    SuiClient,
    rpc_types::{SuiObjectData, SuiObjectDataOptions, SuiObjectResponse},
    types::{
        TypeTag,
        base_types::{ObjectID, ObjectRef},
        transaction::{CallArg, ObjectArg},
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
    let object_data: &SuiObjectData = object_response.data.as_ref().ok_or_else(|| {
        Error::msg(format!(
            "Missing data in object response for '{}'",
            object_id
        ))
    })?;

    // Create ObjectRef
    let object_ref: ObjectRef = (
        object_data.object_id,
        object_data.version,
        object_data.digest,
    );

    // Return as CallArg::Object
    Ok(CallArg::Object(ObjectArg::ImmOrOwnedObject(object_ref)))
}
