// mod test_helper;

// use anyhow::Result;
// use serial_test::serial;
// use sui_sdk::types::base_types::SuiAddress;
// use sui_sdk::types::transaction::TransactionData;
// use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
// use test_helper::{get_gas_coin, setup_client, sign_and_execute};
// use tokio::time::{Duration, sleep};

// //Only work with admin wallet

// #[tokio::test]
// #[serial]
// async fn test_create_pool_admin() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;
//     let mut ptb = ProgrammableTransactionBuilder::new();
//     deep_book_client
//         .deep_book_admin
//         .create_pool_admin(&mut ptb, "PI", "SUI", 0.000001, 0.1, 1.0, true, false)
//         .await?;

//     let gas_coin = get_gas_coin(&client, sender).await?;

//     let gas_budget = 50_000_000;
//     let gas_price = client.read_api().get_reference_gas_price().await?;
//     let tx_data = TransactionData::new_programmable(
//         sender,
//         vec![gas_coin],
//         ptb.finish(),
//         gas_budget,
//         gas_price,
//     );

//     sign_and_execute(&client, sender, tx_data).await?;

//     println!("✅ Pool creation test passed!");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_unregister_pool_admin() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;
//     let mut ptb = ProgrammableTransactionBuilder::new();
//     deep_book_client
//         .deep_book_admin
//         .unregister_pool_admin(&mut ptb, "PI_SUI")
//         .await?;

//     let gas_coin = get_gas_coin(&client, sender).await?;

//     let gas_budget = 5_000_000;
//     let gas_price = client.read_api().get_reference_gas_price().await?;
//     let tx_data = TransactionData::new_programmable(
//         sender,
//         vec![gas_coin],
//         ptb.finish(),
//         gas_budget,
//         gas_price,
//     );

//     sign_and_execute(&client, sender, tx_data).await?;

//     println!("✅ Unregister pool test passed!");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_update_allowed_versions() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;
//     let mut ptb = ProgrammableTransactionBuilder::new();
//     deep_book_client
//         .deep_book_admin
//         .update_allowed_versions(&mut ptb, "PI_SUI")
//         .await?;

//     let gas_coin = get_gas_coin(&client, sender).await?;

//     let gas_budget = 5_000_000;
//     let gas_price = client.read_api().get_reference_gas_price().await?;
//     let tx_data = TransactionData::new_programmable(
//         sender,
//         vec![gas_coin],
//         ptb.finish(),
//         gas_budget,
//         gas_price,
//     );

//     sign_and_execute(&client, sender, tx_data).await?;

//     println!("✅ Update allowed versions test passed!");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_enable_version() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;
//     let mut ptb = ProgrammableTransactionBuilder::new();
//     deep_book_client
//         .deep_book_admin
//         .enable_version(&mut ptb, 3)
//         .await?;

//     let gas_coin = get_gas_coin(&client, sender).await?;

//     let gas_budget = 5_000_000;
//     let gas_price = client.read_api().get_reference_gas_price().await?;
//     let tx_data = TransactionData::new_programmable(
//         sender,
//         vec![gas_coin],
//         ptb.finish(),
//         gas_budget,
//         gas_price,
//     );

//     sign_and_execute(&client, sender, tx_data).await?;

//     println!("✅ Enable version test passed!");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_disable_version() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;
//     let mut ptb = ProgrammableTransactionBuilder::new();
//     deep_book_client
//         .deep_book_admin
//         .disable_version(&mut ptb, 3)
//         .await?;

//     let gas_coin = get_gas_coin(&client, sender).await?;

//     let gas_budget = 5_000_000;
//     let gas_price = client.read_api().get_reference_gas_price().await?;
//     let tx_data = TransactionData::new_programmable(
//         sender,
//         vec![gas_coin],
//         ptb.finish(),
//         gas_budget,
//         gas_price,
//     );

//     sign_and_execute(&client, sender, tx_data).await?;

//     println!("✅ Disable version test passed!");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_set_treasury_address() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;
//     let treasury_address = SuiAddress::random_for_testing_only();
//     let mut ptb = ProgrammableTransactionBuilder::new();
//     deep_book_client
//         .deep_book_admin
//         .set_treasury_address(&mut ptb, treasury_address)
//         .await?;

//     let gas_coin = get_gas_coin(&client, sender).await?;

//     let gas_budget = 5_000_000;
//     let gas_price = client.read_api().get_reference_gas_price().await?;
//     let tx_data = TransactionData::new_programmable(
//         sender,
//         vec![gas_coin],
//         ptb.finish(),
//         gas_budget,
//         gas_price,
//     );

//     sign_and_execute(&client, sender, tx_data).await?;

//     println!("✅ Set treasury address test passed!");
//     Ok(())
// }
