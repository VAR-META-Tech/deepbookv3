mod test_helper;

use anyhow::Result;
use serial_test::serial;
use sui_sdk::types::{
    collection_types::VecSet, programmable_transaction_builder::ProgrammableTransactionBuilder,
    transaction::TransactionData,
};
use test_helper::{get_gas_coin, setup_client, sign_and_execute};
use tokio::time::{Duration, sleep}; // Ensure `tokio` is used for async tests

// #[tokio::test]
// #[serial]
// async fn test_get_account() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Act: Fetch account information
//     let account = deep_book_client
//         .get_account("SUI_DBUSDC", "MANAGER_2")
//         .await?;

//     // Debugging Output
//     println!("Account Info: {:?}", account);

//     // Assertions: Verify that key fields are correctly populated
//     assert!(account.epoch > 0, "Epoch should be greater than 0");
//     assert!(
//         account.open_orders.contents.len() >= 0,
//         "Open orders list should exist"
//     );
//     assert!(
//         account.taker_volume >= 0,
//         "Taker volume should be non-negative"
//     );
//     assert!(
//         account.maker_volume >= 0,
//         "Maker volume should be non-negative"
//     );
//     assert!(
//         account.active_stake >= 0,
//         "Active stake should be non-negative"
//     );
//     assert!(
//         account.inactive_stake >= 0,
//         "Inactive stake should be non-negative"
//     );

//     // Validate balances are properly structured
//     assert!(
//         account.unclaimed_rebates.base >= 0
//             && account.unclaimed_rebates.quote >= 0
//             && account.unclaimed_rebates.deep >= 0,
//         "Unclaimed rebates should be non-negative"
//     );

//     assert!(
//         account.settled_balances.base >= 0
//             && account.settled_balances.quote >= 0
//             && account.settled_balances.deep >= 0,
//         "Settled balances should be non-negative"
//     );

//     assert!(
//         account.owed_balances.base >= 0
//             && account.owed_balances.quote >= 0
//             && account.owed_balances.deep >= 0,
//         "Owed balances should be non-negative"
//     );

//     println!("✅ Test passed: get_account returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_locked_balance() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Act: Fetch locked balance information
//     let locked_balance = deep_book_client
//         .get_locked_balance("SUI_DBUSDC", "MANAGER_2")
//         .await?;

//     // Debugging Output
//     println!("Locked Balance: {}", locked_balance);

//     println!("✅ Test passed: get_locked_balance returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_pool_deep_price() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Act: Fetch deep price for the pool
//     let data = deep_book_client.get_pool_deep_price("SUI_DBUSDC").await?;

//     // Debugging Output
//     println!("Pool Deep Price Data: {:?}", data);

//     // Assertions: Ensure returned data is valid
//     assert!(
//         data.deep_per_asset > 0,
//         "Deep per asset should be greater than 0"
//     );

//     println!("✅ Test passed: get_pool_deep_price returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_pool_book_params() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Act: Fetch pool book parameters
//     let (tick_size, lot_size, min_size) =
//         deep_book_client.get_pool_book_params("SUI_DBUSDC").await?;

//     // Debugging Output
//     println!("Tick Size: {}", tick_size);
//     println!("Lot Size: {}", lot_size);
//     println!("Min Size: {}", min_size);

//     // Assertions: Verify that all returned values are valid (greater than zero)
//     assert!(tick_size > 0, "Tick size should be greater than zero");
//     assert!(lot_size > 0, "Lot size should be greater than zero");
//     assert!(min_size > 0, "Min size should be greater than zero");

//     println!("✅ Test passed: get_pool_book_params returns valid data.");
//     Ok(())
// }
// #[tokio::test]
// #[serial]
// async fn test_get_pool_trade_params() -> Result<()> {
//     let (_client, _sender, deep_book_client) = setup_client().await?;

//     // Act: Fetch trade parameters
//     let trade_params = deep_book_client.get_pool_trade_params("SUI_DBUSDC").await?;

//     // Debugging Output
//     println!("Trade Parameters: {:?}", trade_params);

//     // Assertions: Verify that key fields are correctly populated
//     let (taker_fee, maker_fee, stake_required) = trade_params;

//     assert!(taker_fee >= 0, "Taker fee should be non-negative");
//     assert!(maker_fee >= 0, "Maker fee should be non-negative");
//     assert!(stake_required >= 0, "Stake required should be non-negative");

//     println!("✅ Test passed: get_pool_trade_params returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_pool_id_by_assets() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Act: Fetch the pool ID by asset types
//     let pool_id = deep_book_client
//         .get_pool_id_by_assets(
//             "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP",
//             "0x2::sui::SUI",
//         )
//         .await?;

//     // Debugging Output
//     println!("Pool ID: {:?}", pool_id);

//     // Assert: Ensure that the returned ID is not empty or invalid
//     assert!(
//         format!("{:?}", pool_id).len() > 0,
//         "Pool ID should not be empty"
//     );

//     println!("✅ Test passed: get_pool_id_by_assets returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_vault_balances() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Act: Fetch vault balances for a given pool
//     let balances = deep_book_client.get_vault_balances("SUI_DBUSDC").await?;

//     // Debugging Output
//     println!(
//         "Vault Balances -> Base: {}, Quote: {}, Deep: {}",
//         balances.0, balances.1, balances.2
//     );

//     println!("✅ Test passed: get_vault_balances returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_level2_ticks_from_mid() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Define pool key and tick distance
//     let pool_key = "SUI_DBUSDC";
//     let tick_from_mid = 10;

//     // Act: Fetch level 2 order book ticks
//     let (bid_prices, bid_sizes, ask_prices, ask_sizes) = deep_book_client
//         .get_level2_ticks_from_mid(pool_key, tick_from_mid)
//         .await?;

//     // Debugging Output
//     println!("Bid Prices: {:?}", bid_prices);
//     println!("Bid Sizes: {:?}", bid_sizes);
//     println!("Ask Prices: {:?}", ask_prices);
//     println!("Ask Sizes: {:?}", ask_sizes);

//     // ✅ Check at least some order book data is present (not empty)
//     assert!(
//         !bid_prices.is_empty() || !ask_prices.is_empty(),
//         "Either bid or ask prices should contain data"
//     );

//     println!("✅ Test passed: get_level2_ticks_from_mid returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_level2_range() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Fetch Level 2 order book range
//     let (price_levels, volume_levels) = deep_book_client
//         .get_level2_range("SUI_DBUSDC", 0.1, 200.0, true)
//         .await?;

//     // Debugging Output
//     println!("Price Levels: {:?}", price_levels);
//     println!("Volume Levels: {:?}", volume_levels);

//     println!("✅ Test passed: get_level2_range returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_account_open_orders() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     // Act: Fetch open orders
//     let open_orders: VecSet<u128> = deep_book_client
//         .get_account_open_orders("SUI_DBUSDC", "MANAGER_2")
//         .await?;

//     // Debugging Output
//     println!("Open Orders: {:?}", open_orders);

//     // Assertions: Verify that we received some valid orders
//     assert!(
//         open_orders.contents.len() >= 0,
//         "Open orders list should be at least empty or populated"
//     );

//     println!("✅ Test passed: get_account_open_orders returns valid VecSet<u128> data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_quantity_out() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     let pool_key = "SUI_DBUSDC";
//     let base_quantity = 100.0;
//     let quote_quantity = 0.0;

//     let (output_base, output_quote, execution_price) = deep_book_client
//         .get_quantity_out(pool_key, base_quantity, quote_quantity)
//         .await?;

//     println!("Output Base: {}", output_base);
//     println!("Output Quote: {}", output_quote);
//     println!("Execution Price: {}", execution_price);

//     println!("✅ Test passed: get_quantity_out returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_base_quantity_out() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     let pool_key = "SUI_DBUSDC";
//     let quote_quantity = 100.0;

//     let (base_quantity, quote_quantity_out, deep_quantity_required) = deep_book_client
//         .get_base_quantity_out(pool_key, quote_quantity)
//         .await?;

//     println!("Base Quantity Out: {}", base_quantity);
//     println!("Quote Quantity Out: {}", quote_quantity_out);
//     println!("Deep Quantity Required: {}", deep_quantity_required);

//     println!("✅ Test passed: get_base_quantity_out returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_quote_quantity_out() -> Result<()> {
//     let (client, sender, deep_book_client) = setup_client().await?;

//     let pool_key = "SUI_DBUSDC";
//     let base_quantity = 100.0;

//     let (quote_quantity, base_quantity_out, deep_quantity_required) = deep_book_client
//         .get_quote_quantity_out(pool_key, base_quantity)
//         .await?;

//     println!("Quote Quantity Out: {}", quote_quantity);
//     println!("Base Quantity Out: {}", base_quantity_out);
//     println!("Deep Quantity Required: {}", deep_quantity_required);

//     println!("✅ Test passed: get_quote_quantity_out returns valid data.");
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_get_whitelisted_status() -> Result<()> {
//     let (_client, _sender, deep_book_client) = setup_client().await?;

//     let pool_key = "SUI_DBUSDC"; // Replace with a real pool key

//     let is_whitelisted = deep_book_client.get_whitelisted_status(pool_key).await?;

//     // Debug output
//     println!("Pool {} is whitelisted: {}", pool_key, is_whitelisted);

//     // Assert the result
//     assert!(
//         is_whitelisted == true || is_whitelisted == false,
//         "Whitelisted status should be a boolean"
//     );

//     println!("✅ Test passed: get_whitelisted_status returns a valid boolean.");
//     Ok(())
// }

#[tokio::test]
#[serial]
async fn test_get_mid_price() -> Result<()> {
    let (_client, _sender, deep_book_client) = setup_client().await?;

    let pool_key = "SUI_DBUSDC"; // Replace with a real pool key

    let mid_price = deep_book_client.get_mid_price(pool_key).await?;

    // Debug output
    println!("Mid Price for {}: {}", pool_key, mid_price);

    // Assert the result
    assert!(mid_price > 0, "Mid price should be greater than 0");

    println!("✅ Test passed: get_mid_price returns a valid value.");
    Ok(())
}
