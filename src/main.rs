use std::{any, str::FromStr};

use anyhow::anyhow;
use shared_crypto::intent::Intent;
use sui_config::{SUI_KEYSTORE_FILENAME, sui_config_dir};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    SuiClientBuilder,
    rpc_types::{
        DevInspectResults, SuiObjectData, SuiObjectDataOptions, SuiObjectResponse,
        SuiTransactionBlockResponseOptions,
    },
    types::{
        Identifier, TypeTag,
        base_types::{ObjectID, ObjectRef, SuiAddress},
        crypto::SuiKeyPair,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{
            Argument, CallArg, Command, ObjectArg, ProgrammableMoveCall, Transaction,
            TransactionData, TransactionKind,
        },
        type_input::TypeInput,
    },
};

fn parse_type_input(type_str: &str) -> Result<TypeInput, anyhow::Error> {
    let type_tag = TypeTag::from_str(type_str)?;
    Ok(TypeInput::from(type_tag))
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = SuiClientBuilder::default().build_testnet().await?;

    let keypair = SuiKeyPair::decode(
        "suiprivkey1qzdlfxn2qa2lj5uprl8pyhexs02sg2wrhdy7qaq50cqgnffw4c2477kg9h3",
    )
    .map_err(|_| anyhow!("Invalid Bech32"))?;

    let recipient =
        SuiAddress::from_str("0xf9059cb79bee83c6d746aa24c100f52ff045ba52d13ce6e221580a8f200a4f7d")
            .map_err(|_| anyhow!("Invalid Bech32"))?;

    let sender =
        SuiAddress::from_str("0x38a27d258039c629219b3dbaaeb502381d26f9b93f985e2fec7d248db00d3cf1")
            .map_err(|_| anyhow!("Invalid Bech32"))?;
    println!("Sender address: {}", sender);
    println!("recipient address: {}", recipient);
    // // we need to find the coin we will use as gas
    // let coins = client
    //     .coin_read_api()
    //     .get_coins(sender, None, None, None)
    //     .await?;
    // let coin = coins.data.into_iter().next().unwrap();

    // programmable transactions allows the user to bundle a number of actions into one transaction
    let mut ptb = ProgrammableTransactionBuilder::new();

    // 2) split coin
    // the amount we want in the new coin, 1000 MIST
    let package_id = "0xcbf4748a965d469ea3a36cf0ccc5743b96c2d0ae6dee0762ed3eca65fac07f7e";
    let manager_id = "0x08933685e0246a2ddae2f5e5628fdeba09de831cadf5ad949db308807f18bee5";
    let type_argument = parse_type_input(
        "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
    )
    .map_err(|e| anyhow!(e))?;
    // Create an Argument::Input for Pure 6 value of type u64
    let input_value = manager_id;
    let input_argument = CallArg::Pure(bcs::to_bytes(&input_value).unwrap());

    let idObject = ObjectID::from_str(manager_id).unwrap();
    let sequence_number = 1; // Replace with the actual sequence number
    let object_digest = [0u8; 32]; // Replace with the actual object digest
    let input_argument2 = CallArg::Object(ObjectArg::ImmOrOwnedObject((
        idObject,
        sequence_number.into(),
        object_digest.into(),
    )));

    let pool_address = ObjectID::from_hex_literal(&manager_id)?;
    let pool_object: SuiObjectResponse = client
        .read_api()
        .get_object_with_options(pool_address, SuiObjectDataOptions::full_content())
        .await?;
    let pool_data: &SuiObjectData = pool_object.data.as_ref().ok_or(anyhow::Error::msg(
        format!("Missing data in pool object response for '{}'", "asd"),
    ))?;
    let pool_object_ref: ObjectRef = (pool_data.object_id, pool_data.version, pool_data.digest);
    let pool_input = CallArg::Object(ObjectArg::ImmOrOwnedObject(pool_object_ref));
    ptb.input(pool_input)?;
    // Add this input to the builder
    // ptb.input(input_argument2)?;

    let package = ObjectID::from_hex_literal(package_id).map_err(|e| anyhow!(e))?;
    let module = Identifier::new("balance_manager").map_err(|e| anyhow!(e))?;
    let function = Identifier::new("balance").map_err(|e| anyhow!(e))?;
    // ptb.command(Command::move_call(
    //     package,
    //     module,
    //     function,
    //     vec![type_argument],
    //     vec![Argument::Input(0)],
    // ));
    // let manager_id_arg = ptb.pure(manager_id)?;
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package: ObjectID::from_hex_literal(
            "0xcbf4748a965d469ea3a36cf0ccc5743b96c2d0ae6dee0762ed3eca65fac07f7e",
        )?,
        module: "balance_manager".to_string(),
        function: "balance".to_string(),
        type_arguments: vec![type_argument],
        arguments: vec![Argument::Input(0)],
    })));

    // finish building the transaction block by calling finish on the ptb
    let builder = ptb.finish();

    // let pt = ProgrammableTransaction {
    //     inputs: vec![
    //         CallArg::Object(bridge_object_arg),
    //         CallArg::Pure(bcs::to_bytes(&source_chain_id).unwrap()),
    //         CallArg::Pure(bcs::to_bytes(&seq_number).unwrap()),
    //     ],
    //     commands: vec![Command::move_call(
    //         ObjectID::from_hex_literal(
    //             "0xcbf4748a965d469ea3a36cf0ccc5743b96c2d0ae6dee0762ed3eca65fac07f7e",
    //         )?,
    //         Identifier::new("bridge").unwrap(),
    //         Identifier::new(function_name).unwrap(),
    //         vec![],
    //         vec![Argument::Input(0), Argument::Input(1), Argument::Input(2)],
    //     )],
    // };
    let kind = TransactionKind::programmable(builder);

    let resp = client
        .read_api()
        .dev_inspect_transaction_block(sender, kind, None, None, None)
        .await?;

    println!("resp: {:?}", resp);

    let DevInspectResults {
        results, effects, ..
    } = resp;
    println!("results: {:?}", results);
    let Some(results) = results else {
        return Err(anyhow::Error::msg(format!(
            "No results returned for '{}', effects: {:?}",
            "balance", effects
        )));
    };

    let return_values = &results
        .first()
        .ok_or(anyhow::Error::msg(format!(
            "No return values for '{}', results: {:?}",
            "balance", results
        )))?
        .return_values;

    let (value_bytes, _type_tag) = return_values.first().ok_or(anyhow::Error::msg(format!(
        "No first return value for '{}', results: {:?}",
        "balance", results
    )))?;

    println!("Return value: {:?}", value_bytes);

    let data: u64 = bcs::from_bytes(value_bytes)?;
    println!("Return value: {:?}", data / 1000000000);

    // let gas_budget = 5_000_000;
    // let gas_price = client.read_api().get_reference_gas_price().await?;
    // // create the transaction data that will be sent to the network
    // let tx_data = TransactionData::new_programmable(
    //     sender,
    //     vec![coin.object_ref()],
    //     builder,
    //     gas_budget,
    //     gas_price,
    // );

    // // 4) sign transaction
    // let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    // let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

    // 5) execute the transaction
    // print!("Executing the transaction...");
    // let transaction_response = client
    //     .quorum_driver_api()
    //     .execute_transaction_block(
    //         Transaction::from_data(tx_data, vec![signature]),
    //         SuiTransactionBlockResponseOptions::full_content(),
    //         Some(ExecuteTransactionRequestType::WaitForLocalExecution),
    //     )
    //     .await?;
    // print!("done\n Transaction information: ");
    // println!("{:?}", transaction_response);

    Ok(())
}
