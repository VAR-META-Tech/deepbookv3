mod test_helper;

use anyhow::Result;
use serial_test::serial;
use sui_sdk::types::{
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    transaction::{Command, TransactionData},
};
use test_helper::{get_gas_coin, setup_client, sign_and_execute};
use tokio::time::{Duration, sleep};

#[tokio::test]
#[serial]
async fn test_flash_loans_base_workflow() -> Result<(), anyhow::Error> {
    println!("Testing flash loan workflow...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up Programmable Transaction Builder
    let mut ptb = ProgrammableTransactionBuilder::new();

    // Step 2: Borrow base asset (flash loan)
    let loan_amount = 123.321f64;
    println!("Borrowing {} from DEEP_SUI pool...", loan_amount);
    let (coin, flash_loan) = deep_book_client
        .flash_loans
        .borrow_base_asset(&mut ptb, "DEEP_SUI", loan_amount)
        .await?;

    //implement logic arbitrage here

    // Step 3: Return the flash loan
    println!("Returning flash loan...");
    let coin_remain = deep_book_client
        .flash_loans
        .return_flashloan_base(&mut ptb, "DEEP_SUI", loan_amount, coin, flash_loan)
        .await?;

    // Step 4: Transfer any remaining funds back to sender
    println!("Transferring remaining funds...");
    let recipient_arg = ptb.pure(sender)?;
    ptb.command(Command::TransferObjects(vec![coin_remain], recipient_arg));

    // Step 5: Finish building the transaction
    let pt = ptb.finish();

    // Step 6: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 7: Set up gas and create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 8: Sign and execute the transaction
    println!("Signing and executing transaction...");
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!(
        "Flash loan test completed successfully! {:?}",
        transaction_response
    );
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_flash_loans_quote_workflow() -> Result<(), anyhow::Error> {
    println!("Testing flash loan workflow...");
    let (client, sender, deep_book_client) = setup_client().await?;

    // Step 1: Set up Programmable Transaction Builder
    let mut ptb = ProgrammableTransactionBuilder::new();

    // Step 2: Borrow base asset (flash loan)
    let loan_amount = 123.321f64;
    println!("Borrowing {} from DEEP_SUI pool...", loan_amount);
    let (coin, flash_loan) = deep_book_client
        .flash_loans
        .borrow_quote_asset(&mut ptb, "DEEP_SUI", loan_amount)
        .await?;

    //implement logic arbitrage here

    // Step 3: Return the flash loan
    println!("Returning flash loan...");
    let coin_remain = deep_book_client
        .flash_loans
        .return_flashloan_quote(&mut ptb, "DEEP_SUI", loan_amount, coin, flash_loan)
        .await?;

    // Step 4: Transfer any remaining funds back to sender
    println!("Transferring remaining funds...");
    let recipient_arg = ptb.pure(sender)?;
    ptb.command(Command::TransferObjects(vec![coin_remain], recipient_arg));

    // Step 5: Finish building the transaction
    let pt = ptb.finish();

    // Step 6: Fetch a suitable gas coin
    let gas_coin = get_gas_coin(&client, sender).await?;

    // Step 7: Set up gas and create transaction data
    let gas_budget = 5_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_coin], pt, gas_budget, gas_price);

    // Step 8: Sign and execute the transaction
    println!("Signing and executing transaction...");
    let transaction_response = sign_and_execute(&client, sender, tx_data).await?;

    println!(
        "Flash loan test completed successfully! {:?}",
        transaction_response
    );
    Ok(())
}
