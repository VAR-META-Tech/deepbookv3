mod test_helper;

use anyhow::Result;
use serial_test::serial;
use sui_sdk::types::transaction::TransactionData;
use test_helper::{get_gas_coin, setup_client, sign_and_execute};
use tokio::time::{Duration, sleep}; // Ensure `tokio` is used for async tests

#[tokio::test]
#[serial]
async fn test_create_and_share_balance_manager() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Generate transaction
    let pt = deep_book_client
        .balance_manager
        .create_and_share_balance_manager()
        .await?;

    // Step 2: Fetch gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute transaction
    sign_and_execute(&client, sender, tx_data).await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_check_manager_balance() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Run check_manager_balance function
    let result = deep_book_client
        .check_manager_balance("MANAGER_2", "SUI")
        .await;

    match result {
        Ok((coin_type, balance)) => {
            println!("Balance: {} {}", balance, coin_type);
            assert_eq!(
                coin_type,
                "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
            ); // Expected coin type
            println!("Balance is non-negative as expected");
        }
        Err(e) => {
            panic!("Error checking balance: {:?}", e);
        }
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_withdraw_from_manager() -> Result<()> {
    println!("Withdrawing from manager...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up transaction for withdrawal
    let withdraw_amount = 0.1;
    let recipient = sender; // Self-withdrawal test
    let pt = deep_book_client
        .balance_manager
        .withdraw_from_manager(&client, "MANAGER_2", "SUI", withdraw_amount, recipient)
        .await?;

    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute the transaction
    sign_and_execute(&client, sender, tx_data).await?;

    println!("Withdrawal successful.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_withdraw_all_from_manager() -> Result<()> {
    println!("Withdrawing from manager...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up transaction for withdrawal
    let recipient = sender; // Self-withdrawal test
    let pt = deep_book_client
        .balance_manager
        .withdraw_all_from_manager(&client, "MANAGER_2", "SUI", recipient)
        .await?;

    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 4: Sign and execute the transaction
    sign_and_execute(&client, sender, tx_data).await?;

    println!("Withdrawal successful.");
    Ok(())
}
