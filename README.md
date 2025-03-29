
# 🔱 DeepBook V3 SDK (Rust)

> A high-level Rust SDK for interacting with the DeepBook V3 protocol on Sui.

---

## 🚀 Overview

`deepbookv3` is a Rust library designed to simplify integrations with DeepBook V3 smart contracts on the Sui blockchain. It abstracts away low-level Move commands and provides programmable transaction support, account/balance handling, pool analytics, and much more.

This SDK is **battle-tested** on:
- ✅ Testnet
- ✅ Mainnet (with correct package + pool + coin configs)

---

## 📦 Installation

### Option 1: Use from Git

```toml
[dependencies]
deepbookv3 = { git = "https://github.com/VAR-META-Tech/deepbookv3" }
```

### Option 2: Use locally (for development)

Clone the repo and add this in your project's `Cargo.toml`:

```toml
[dependencies]
deepbookv3 = { path = "../deepbookv3" }
```

---

## 🔗 Example Integration

Check out a full sample implementation using this SDK here:

👉 [VAR-META-Tech/deepbookv3-client-sample](https://github.com/VAR-META-Tech/deepbookv3-client-sample)

---

## 🧱 Features

- BalanceManager support (Deposit, Withdraw, TradeCap, etc.)
- Swap exact base/quote for quote/base
- Get deep price, pool params, mid price
- Account inspection + vault balances
- Place/cancel/modify limit orders
- Admin actions: pool registration, versioning
- Dev Inspect transactions for simulation (read-only)

---

## 🛠️ Usage

### Initialize the SDK

```rust
use deepbookv3::client::DeepBookClient;

let client = SuiClientBuilder::default().build_testnet().await?;
let sender = SuiAddress::from_str("0xYOURADDRESS")?;

let deep_book = DeepBookClient::new(
    client,
    sender,
    "testnet",                // or "mainnet", "devnet"
    Some(get_mainnet_managers()),
    Some(get_mainnet_coins()),
    Some(get_mainnet_pools()),
    Some("0xADMIN_CAP_ID".to_string()),
);
```

---

### ✅ Example: Check BalanceManager Balance

```rust
let (coin_type, balance) = deep_book.check_manager_balance("MANAGER_KEY", "SUI").await?;
println!("{} Balance: {}", coin_type, balance);
```

---

### ✅ Example: Swap exact base for quote

```rust
use deepbookv3::types::SwapParams;

let params = SwapParams {
    pool_key: "DEEP_SUI".to_string(),
    amount: 1.0,
    deep_amount: 5.0,
    min_out: 0.01,
};

let mut ptb = ProgrammableTransactionBuilder::new();

let (base_result, quote_result, deep_result) = deep_book
    .deep_book
    .swap_exact_base_for_quote(&mut ptb, &params)
    .await?;
```

---

## 🧪 Testing

```bash
cargo test --all -- --nocapture
```

To run a specific file:

```bash
cargo test --test balance_manager_tests
```

> ⚠️ Some tests require admin-cap wallet or specific on-chain objects to exist.

---

## 📚 Docs

This SDK is modular:
- `client/` – Entry point and high-level interface
- `transactions/` – Low-level module for building programmable transactions
- `types/` – Shared structs like `Coin`, `Pool`, `SwapParams`
- `utils/config/` – Helpers for managing environments and configs

---

## 🔐 Environment Support

| Environment | Supported? | Notes |
|-------------|------------|-------|
| Devnet      | ✅          | Good for debugging |
| Testnet     | ✅          | Stable API |
| Mainnet     | ✅          | Use production configs |
| Localnet    | ❌          | Not currently supported |

---

## 🙌 Acknowledgements

Built with ❤️ using:
- [Sui SDK](https://github.com/MystenLabs/sui)
- [Move Language](https://move-language.github.io/)
- [DeepBook Protocol](https://github.com/MystenLabs/deepbook)

---

## 📄 License

Apache-2.0 © VARMETA
