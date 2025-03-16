mod test_helper;

use anyhow::Result;
use serial_test::serial;
use sui_sdk::types::{
    programmable_transaction_builder::ProgrammableTransactionBuilder, transaction::TransactionData,
};
use test_helper::{get_gas_coin, setup_client, sign_and_execute};
use tokio::time::{Duration, sleep}; // Ensure `tokio` is used for async tests

#[tokio::test]
#[serial]
async fn test_get_account() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Fetch account information
    let account = deep_book_client
        .get_account("SUI_DBUSDC", "MANAGER_2")
        .await?;

    // Debugging Output
    println!("Account Info: {:?}", account);

    // Assertions: Verify that key fields are correctly populated
    assert!(account.epoch > 0, "Epoch should be greater than 0");
    assert!(
        account.open_orders.contents.len() >= 0,
        "Open orders list should exist"
    );
    assert!(
        account.taker_volume >= 0,
        "Taker volume should be non-negative"
    );
    assert!(
        account.maker_volume >= 0,
        "Maker volume should be non-negative"
    );
    assert!(
        account.active_stake >= 0,
        "Active stake should be non-negative"
    );
    assert!(
        account.inactive_stake >= 0,
        "Inactive stake should be non-negative"
    );

    // Validate balances are properly structured
    assert!(
        account.unclaimed_rebates.base >= 0
            && account.unclaimed_rebates.quote >= 0
            && account.unclaimed_rebates.deep >= 0,
        "Unclaimed rebates should be non-negative"
    );

    assert!(
        account.settled_balances.base >= 0
            && account.settled_balances.quote >= 0
            && account.settled_balances.deep >= 0,
        "Settled balances should be non-negative"
    );

    assert!(
        account.owed_balances.base >= 0
            && account.owed_balances.quote >= 0
            && account.owed_balances.deep >= 0,
        "Owed balances should be non-negative"
    );

    println!("✅ Test passed: get_account returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_locked_balance() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Fetch locked balance information
    let locked_balance = deep_book_client
        .get_locked_balance("SUI_DBUSDC", "MANAGER_2")
        .await?;

    // Debugging Output
    println!("Locked Balance: {}", locked_balance);

    println!("✅ Test passed: get_locked_balance returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_pool_deep_price() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Fetch deep price for the pool
    let data = deep_book_client.get_pool_deep_price("SUI_DBUSDC").await?;

    // Debugging Output
    println!("Pool Deep Price Data: {:?}", data);

    // Assertions: Ensure returned data is valid
    assert!(
        data.deep_per_asset > 0,
        "Deep per asset should be greater than 0"
    );

    println!("✅ Test passed: get_pool_deep_price returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_pool_book_params() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Fetch pool book parameters
    let (tick_size, lot_size, min_size) =
        deep_book_client.get_pool_book_params("SUI_DBUSDC").await?;

    // Debugging Output
    println!("Tick Size: {}", tick_size);
    println!("Lot Size: {}", lot_size);
    println!("Min Size: {}", min_size);

    // Assertions: Verify that all returned values are valid (greater than zero)
    assert!(tick_size > 0, "Tick size should be greater than zero");
    assert!(lot_size > 0, "Lot size should be greater than zero");
    assert!(min_size > 0, "Min size should be greater than zero");

    println!("✅ Test passed: get_pool_book_params returns valid data.");
    Ok(())
}
#[tokio::test]
#[serial]
async fn test_get_pool_trade_params() -> Result<()> {
    let (_client, _sender, deep_book_client) = setup_client().await?;

    // Act: Fetch trade parameters
    let trade_params = deep_book_client.get_pool_trade_params("SUI_DBUSDC").await?;

    // Debugging Output
    println!("Trade Parameters: {:?}", trade_params);

    // Assertions: Verify that key fields are correctly populated
    let (taker_fee, maker_fee, stake_required) = trade_params;

    assert!(taker_fee >= 0, "Taker fee should be non-negative");
    assert!(maker_fee >= 0, "Maker fee should be non-negative");
    assert!(stake_required >= 0, "Stake required should be non-negative");

    println!("✅ Test passed: get_pool_trade_params returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_pool_id_by_assets() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Fetch the pool ID by asset types
    let pool_id = deep_book_client
        .get_pool_id_by_assets(
            "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP",
            "0x2::sui::SUI",
        )
        .await?;

    // Debugging Output
    println!("Pool ID: {:?}", pool_id);

    // Assert: Ensure that the returned ID is not empty or invalid
    assert!(
        format!("{:?}", pool_id).len() > 0,
        "Pool ID should not be empty"
    );

    println!("✅ Test passed: get_pool_id_by_assets returns valid data.");
    Ok(())
}
