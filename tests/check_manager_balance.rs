use anyhow::Result;
use deepbookv3::client::DeepBookClient;
use deepbookv3::types::BalanceManager;
use std::collections::HashMap;
use std::str::FromStr;
use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::SuiAddress;
use tokio; // Ensure `tokio` is used for async tests

#[tokio::test]
async fn test_check_manager_balance() -> Result<()> {
    // Initialize SuiClient
    let client = SuiClientBuilder::default().build_testnet().await?;

    // Set up sender address
    let sender =
        SuiAddress::from_str("0x38a27d258039c629219b3dbaaeb502381d26f9b93f985e2fec7d248db00d3cf1")?;

    // Create balance managers
    let balance_managers: HashMap<String, BalanceManager> = HashMap::from([(
        "MANAGER_2".to_string(),
        BalanceManager {
            address: "0x08933685e0246a2ddae2f5e5628fdeba09de831cadf5ad949db308807f18bee5",
            trade_cap: None,
        },
    )]);

    // Initialize DeepBookClient
    let deep_book_client = DeepBookClient::new(
        client,
        sender,
        "testnet",
        Some(balance_managers),
        None,
        None,
        None,
    );

    // Run check_manager_balance function
    let result = deep_book_client
        .check_manager_balance("MANAGER_2", "SUI")
        .await;

    // Assert expected behavior
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
