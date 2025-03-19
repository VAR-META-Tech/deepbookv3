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
async fn test_stake() -> Result<()> {
    println!("Staking in the pool...");

    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb = ProgrammableTransactionBuilder::new();

    // Step 1: Set up stake transaction
    deep_book_client
        .governance
        .stake(&mut ptb, "SUI_DBUSDC", "MANAGER_2", 10.0)
        .await?;

    // Step 2: Fetch gas coin
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

    println!("Stake transaction successful.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_unstake() -> Result<()> {
    println!("Unstaking from the pool...");

    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb = ProgrammableTransactionBuilder::new();

    // Step 1: Set up unstake transaction
    deep_book_client
        .governance
        .unstake(&mut ptb, "SUI_DBUSDC", "MANAGER_2")
        .await?;

    // Step 2: Fetch gas coin
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

    println!("Unstake transaction successful.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_submit_proposal() -> Result<()> {
    println!("Submitting governance proposal...");

    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb = ProgrammableTransactionBuilder::new();

    // Step 1: Set up proposal submission transaction
    deep_book_client
        .governance
        .submit_proposal(&mut ptb, "SUI_DBUSDC", "MANAGER_2", 0.001, 0.002, 50.0)
        .await?;

    // Step 2: Fetch gas coin
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

    println!("Proposal submission successful.");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_vote() -> Result<()> {
    println!("Voting on a governance proposal...");

    let (client, sender, deep_book_client) = setup_client().await?;
    let mut ptb = ProgrammableTransactionBuilder::new();

    // Step 1: Set up voting transaction
    let proposal_id = "0x123456789abcdef"; // Replace with actual proposal ID
    deep_book_client
        .governance
        .vote(&mut ptb, "SUI_DBUSDC", "MANAGER_2", proposal_id)
        .await?;

    // Step 2: Fetch gas coin
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

    println!("Voting transaction successful.");
    Ok(())
}
