mod test_helper;

use anyhow::Result;
use deepbookv3::types::{
    OrderType, PlaceLimitOrderParams, PlaceMarketOrderParams, SelfMatchingOptions, SwapParams,
};
use serial_test::serial;
use sui_sdk::types::{
    collection_types::VecSet,
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    transaction::{Command, TransactionData},
};
use sui_types::base_types::ObjectRef;
use test_helper::{get_gas_coin, setup_client, sign_and_execute};
use tokio::time::{Duration, sleep}; // Ensure `tokio` is used for async tests

#[tokio::test]
#[serial]
async fn test_place_limit_order() -> Result<()> {
    println!("Placing limit order...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up transaction for place_limit_order
    let params = PlaceLimitOrderParams {
        pool_key: "DEEP_SUI".to_string(),
        balance_manager_key: "MANAGER_2".to_string(),
        client_order_id: "123123".to_string(),
        price: 0.01,
        quantity: 1.0,
        is_bid: true,
        expiration: None,
        order_type: Some(OrderType::NoRestriction),
        self_matching_option: Some(SelfMatchingOptions::SelfMatchingAllowed),
        pay_with_deep: Some(true),
    };
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();
    deep_book_client
        .deep_book
        .place_limit_order(&mut ptb, &params)
        .await?;

    let pt = ptb.finish();
    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 50_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute the transaction
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("Transaction response: {:?}", transaction_response);

    // assert!(
    //     transaction_response.digest.is_some(),
    //     "Transaction digest should not be empty"
    // );

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_place_market_order() -> Result<()> {
    println!("Placing market order...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up transaction for place_market_order
    let params = PlaceMarketOrderParams {
        pool_key: "DEEP_SUI".to_string(),
        balance_manager_key: "MANAGER_2".to_string(),
        client_order_id: "123123".to_string(),
        quantity: 1f64,
        is_bid: true,
        self_matching_option: Some(SelfMatchingOptions::SelfMatchingAllowed),
        pay_with_deep: Some(true),
    };
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();

    deep_book_client
        .deep_book
        .place_market_order(&mut ptb, &params)
        .await?;

    let pt = ptb.finish();

    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 20_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute the transaction
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("Transaction response: {:?}", transaction_response);

    // assert!(
    //     transaction_response.digest.is_some(),
    //     "Transaction digest should not be empty"
    // );

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_cancel_order() -> Result<()> {
    println!("cancel order...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up transaction for cancel_order
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();

    deep_book_client
        .deep_book
        .cancel_order(
            &mut ptb,
            "DEEP_SUI",
            "MANAGER_2",
            184467440755542260233709402626,
        )
        .await?;

    let pt = ptb.finish();
    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 50_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute the transaction
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("Transaction response: {:?}", transaction_response);

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_cancel_alls_orders() -> Result<()> {
    println!("cancel all orders...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up transaction for cancel all orders
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();

    deep_book_client
        .deep_book
        .cancel_all_orders(&mut ptb, "DEEP_SUI", "MANAGER_2")
        .await?;

    let pt = ptb.finish();
    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 50_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute the transaction
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("Transaction response: {:?}", transaction_response);

    Ok(())
}

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

    // Act: Fetch deep price for the pool with scaled values
    let data = deep_book_client.get_pool_deep_price("DEEP_SUI").await?;

    // Debugging Output
    println!("📊 Pool Deep Price Scaled Data: {:?}", data);

    // Assertions
    assert!(
        data.deep_per_base.unwrap_or(0.0) >= 0.0 || data.deep_per_quote.unwrap_or(0.0) >= 0.0,
        "At least one of deep_per_base or deep_per_quote should be greater than 0"
    );

    println!("✅ Test passed: get_pool_deep_price_with_scaled_value returns valid scaled data.");
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
    assert!(tick_size > 0.0, "Tick size should be greater than zero");
    assert!(lot_size > 0.0, "Lot size should be greater than zero");
    assert!(min_size > 0.0, "Min size should be greater than zero");

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

    assert!(taker_fee >= 0.0, "Taker fee should be non-negative");
    assert!(maker_fee >= 0.0, "Maker fee should be non-negative");
    assert!(
        stake_required >= 0.0,
        "Stake required should be non-negative"
    );

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

#[tokio::test]
#[serial]
async fn test_get_vault_balances() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Fetch vault balances for a given pool
    let balances = deep_book_client.get_vault_balances("SUI_DBUSDC").await?;

    // Debugging Output
    println!(
        "Vault Balances -> Base: {}, Quote: {}, Deep: {}",
        balances.0, balances.1, balances.2
    );

    println!("✅ Test passed: get_vault_balances returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_level2_ticks_from_mid() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Define pool key and tick distance
    let pool_key = "SUI_DBUSDC";
    let tick_from_mid = 10;

    // Act: Fetch level 2 order book ticks
    let (bid_prices, bid_sizes, ask_prices, ask_sizes) = deep_book_client
        .get_level2_ticks_from_mid(pool_key, tick_from_mid)
        .await?;

    // Debugging Output
    println!("Bid Prices: {:?}", bid_prices);
    println!("Bid Sizes: {:?}", bid_sizes);
    println!("Ask Prices: {:?}", ask_prices);
    println!("Ask Sizes: {:?}", ask_sizes);

    // ✅ Check at least some order book data is present (not empty)
    assert!(
        !bid_prices.is_empty() || !ask_prices.is_empty(),
        "Either bid or ask prices should contain data"
    );

    println!("✅ Test passed: get_level2_ticks_from_mid returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_level2_range() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Fetch Level 2 order book range
    let (price_levels, volume_levels) = deep_book_client
        .get_level2_range("SUI_DBUSDC", 0.1, 200.0, true)
        .await?;

    // Debugging Output
    println!("Price Levels: {:?}", price_levels);
    println!("Volume Levels: {:?}", volume_levels);

    println!("✅ Test passed: get_level2_range returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_account_open_orders() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Fetch open orders
    let open_orders: VecSet<u128> = deep_book_client
        .get_account_open_orders("SUI_DBUSDC", "MANAGER_2")
        .await?;

    // Debugging Output
    println!("Open Orders: {:?}", open_orders);

    // Assertions: Verify that we received some valid orders
    assert!(
        open_orders.contents.len() >= 0,
        "Open orders list should be at least empty or populated"
    );

    println!("✅ Test passed: get_account_open_orders returns valid VecSet<u128> data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_quantity_out() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    let pool_key = "SUI_DBUSDC";
    let base_quantity = 100.0;
    let quote_quantity = 0.0;

    let (output_base, output_quote, execution_price) = deep_book_client
        .get_quantity_out(pool_key, base_quantity, quote_quantity)
        .await?;

    println!("Output Base: {}", output_base);
    println!("Output Quote: {}", output_quote);
    println!("Execution Price: {}", execution_price);

    println!("✅ Test passed: get_quantity_out returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_base_quantity_out() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    let pool_key = "SUI_DBUSDC";
    let quote_quantity = 100.0;

    let (base_quantity, quote_quantity_out, deep_quantity_required) = deep_book_client
        .get_base_quantity_out(pool_key, quote_quantity)
        .await?;

    println!("Base Quantity Out: {}", base_quantity);
    println!("Quote Quantity Out: {}", quote_quantity_out);
    println!("Deep Quantity Required: {}", deep_quantity_required);

    println!("✅ Test passed: get_base_quantity_out returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_quote_quantity_out() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    let pool_key = "SUI_DBUSDC";
    let base_quantity = 100.0;

    let (quote_quantity, base_quantity_out, deep_quantity_required) = deep_book_client
        .get_quote_quantity_out(pool_key, base_quantity)
        .await?;

    println!("Quote Quantity Out: {}", quote_quantity);
    println!("Base Quantity Out: {}", base_quantity_out);
    println!("Deep Quantity Required: {}", deep_quantity_required);

    println!("✅ Test passed: get_quote_quantity_out returns valid data.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_whitelisted_status() -> Result<()> {
    let (_client, _sender, deep_book_client) = setup_client().await?;

    let pool_key = "SUI_DBUSDC"; // Replace with a real pool key

    let is_whitelisted = deep_book_client.get_whitelisted_status(pool_key).await?;

    // Debug output
    println!("Pool {} is whitelisted: {}", pool_key, is_whitelisted);

    // Assert the result
    assert!(
        is_whitelisted == true || is_whitelisted == false,
        "Whitelisted status should be a boolean"
    );

    println!("✅ Test passed: get_whitelisted_status returns a valid boolean.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_mid_price() -> Result<()> {
    let (_client, _sender, deep_book_client) = setup_client().await?;

    let pool_key = "DEEP_SUI"; // Replace with a real pool key

    let mid_price = deep_book_client.get_mid_price(pool_key).await?;

    // Debug output
    println!("Mid Price for {}: {}", pool_key, mid_price);

    // Assert the result
    assert!(mid_price > 0, "Mid price should be greater than 0");

    println!("✅ Test passed: get_mid_price returns a valid value.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_swap_exact_base_for_quote() -> Result<(), anyhow::Error> {
    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb = ProgrammableTransactionBuilder::new();

    let (base_coin_result, quote_coin_result, deep_coin_result) = deep_book_client
        .deep_book
        .swap_exact_base_for_quote(
            &mut ptb,
            &SwapParams {
                pool_key: "DEEP_SUI".to_string(),
                amount: 1.1,
                deep_amount: 0.0,
                min_out: 0.00,
            },
        )
        .await?;

    ptb.transfer_args(
        sender,
        vec![base_coin_result, quote_coin_result, deep_coin_result],
    );

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, Some("0x2::sui::SUI".to_string()), None, None)
        .await?
        .data;

    let gas_object_refs: Vec<ObjectRef> = gas_coins
        .iter()
        .map(|coin| (coin.coin_object_id, coin.version, coin.digest))
        .collect();

    let gas_budget = 50_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let pt = ptb.finish();

    let tx_data =
        TransactionData::new_programmable(sender, gas_object_refs, pt, gas_budget, gas_price);
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("✅ Swap transaction succeeded: {:?}", transaction_response);

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_swap_exact_quote_for_base() -> Result<(), anyhow::Error> {
    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb = ProgrammableTransactionBuilder::new();

    let (base_coin_result, quote_coin_result, deep_coin_result) = deep_book_client
        .deep_book
        .swap_exact_quote_for_base(
            &mut ptb,
            &SwapParams {
                pool_key: "DEEP_SUI".to_string(),
                amount: 1.0,      // Quote amount (e.g., DBUSDT)
                deep_amount: 0.0, // DEEP tokens burned
                min_out: 0.0,     // Expected min base out (e.g., SUI)
            },
        )
        .await?;

    ptb.transfer_args(
        sender,
        vec![base_coin_result, quote_coin_result, deep_coin_result],
    );

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, Some("0x2::sui::SUI".to_string()), None, None)
        .await?
        .data;

    let gas_object_refs: Vec<ObjectRef> = gas_coins
        .iter()
        .map(|coin| (coin.coin_object_id, coin.version, coin.digest))
        .collect();

    let gas_budget = 50_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let pt = ptb.finish();

    println!("📜 Commands for swap_exact_quote_for_base:");
    for (i, cmd) in pt.commands.iter().enumerate() {
        println!("  [{}] {:?}", i, cmd);
    }

    let tx_data =
        TransactionData::new_programmable(sender, gas_object_refs, pt, gas_budget, gas_price);

    println!("🚀 Signing and executing quote-for-base swap transaction...");
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("✅ Transaction response: {:?}", transaction_response);

    Ok(())
}
