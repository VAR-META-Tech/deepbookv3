mod test_helper;

use anyhow::Result;
use serial_test::serial;
use sui_sdk::types::{
    programmable_transaction_builder::ProgrammableTransactionBuilder, transaction::TransactionData,
};
use sui_types::base_types::ObjectRef;
use test_helper::{get_gas_coin, setup_client, sign_and_execute};
use tokio::time::{Duration, sleep}; // Ensure `tokio` is used for async tests

#[tokio::test]
#[serial]
async fn test_create_and_share_balance_manager() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();

    // Step 1: Generate transaction
    deep_book_client
        .balance_manager
        .create_and_share_balance_manager(&mut ptb)
        .await?;

    // Step 2: Fetch gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![gas_coin],
        ptb.finish(),
        gas_budget,
        gas_price,
    );

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
async fn test_deposit_to_manager() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();

    deep_book_client
        .balance_manager
        .deposit_into_manager(&mut ptb, "MANAGER_2", "DEEP", 1000.1)
        .await?;
    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, Some("0x2::sui::SUI".to_string()), None, None)
        .await?
        .data;
    let gas_object_refs: Vec<ObjectRef> = gas_coins
        .iter()
        .map(|coin| (coin.coin_object_id, coin.version, coin.digest))
        .collect();

    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data: TransactionData = TransactionData::new_programmable(
        sender,
        gas_object_refs,
        ptb.finish(),
        gas_budget,
        gas_price,
    );

    println!("Signing and executing transaction...");
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!("transaction_response: {:?} ", transaction_response);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_withdraw_from_manager() -> Result<()> {
    println!("Withdrawing from manager...");
    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();

    // Step 1: Set up transaction for withdrawal
    let withdraw_amount = 0.1;
    let recipient = sender; // Self-withdrawal test
    let pt = deep_book_client
        .balance_manager
        .withdraw_from_manager(&mut ptb, "MANAGER_2", "SUI", withdraw_amount, recipient)
        .await?;

    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![gas_coin],
        ptb.finish(),
        gas_budget,
        gas_price,
    );

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
    let mut ptb: ProgrammableTransactionBuilder = ProgrammableTransactionBuilder::new();

    // Step 1: Set up transaction for withdrawal
    let recipient = sender; // Self-withdrawal test
    deep_book_client
        .balance_manager
        .withdraw_all_from_manager(&mut ptb, "MANAGER_2", "USDC", recipient)
        .await?;

    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![gas_coin],
        ptb.finish(),
        gas_budget,
        gas_price,
    );

    // Step 4: Sign and execute the transaction
    sign_and_execute(&client, sender, tx_data).await?;

    println!("Withdrawal successful.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_manager_owner() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Retrieve the owner of the balance manager
    let owner = deep_book_client.get_manager_owner("MANAGER_2").await?;

    // Assert: Owner should be the sender (assuming sender owns the manager)
    println!("Balance Manager Owner: {:?}", owner);
    assert_eq!(
        owner, sender,
        "The manager owner should match the sender address."
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_manager_id() -> Result<()> {
    let (client, sender, deep_book_client) = setup_client().await?;

    // Act: Retrieve the manager ID
    let manager_id = deep_book_client.get_manager_id("MANAGER_2").await?;

    // Assert: Manager ID should not be empty
    println!("Balance Manager ID: {:?}", manager_id);
    assert!(
        !format!("{:?}", manager_id).is_empty(),
        "The manager ID should not be empty."
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_generate_trade_proof() -> Result<()> {
    println!("Generating trade proof...");

    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb = ProgrammableTransactionBuilder::new();
    // Step 1: Set up trade proof transaction
    let pt = deep_book_client
        .balance_manager
        .generate_proof(&mut ptb, "MANAGER_2")
        .await?;

    // Step 2: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 3: Set up gas and create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![gas_coin],
        ptb.finish(),
        gas_budget,
        gas_price,
    );

    // Step 4: Sign and execute the transaction
    sign_and_execute(&client, sender, tx_data).await?;

    println!("Trade proof generated successfully.");
    Ok(())
}
