use serde::Deserialize;
use sui_sdk::types::{collection_types::VecSet, id::ID};

#[derive(Debug, Clone)]
pub struct BalanceManager {
    pub address: &'static str,
    pub trade_cap: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct Coin {
    pub address: &'static str,
    pub coin_type: &'static str,
    pub scalar: u64,
}

#[derive(Debug, Clone)]
pub struct Pool {
    pub address: &'static str,
    pub base_coin: &'static str,
    pub quote_coin: &'static str,
}

#[derive(Debug, Clone)]
pub struct DeepbookPackageIds {
    pub deepbook_package_id: &'static str,
    pub registry_id: &'static str,
    pub deep_treasury_id: &'static str,
}

// Trading constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    NoRestriction,
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

// Self matching options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfMatchingOptions {
    SelfMatchingAllowed,
    CancelTaker,
    CancelMaker,
}

#[derive(Debug, Clone)]
pub struct PlaceLimitOrderParams {
    pub pool_key: String,
    pub balance_manager_key: String,
    pub client_order_id: String,
    pub price: f64,
    pub quantity: f64,
    pub is_bid: bool,
    pub expiration: Option<u64>,
    pub order_type: Option<OrderType>,
    pub self_matching_option: Option<SelfMatchingOptions>,
    pub pay_with_deep: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct PlaceMarketOrderParams {
    pub pool_key: String,
    pub balance_manager_key: String,
    pub client_order_id: String,
    pub quantity: f64,
    pub is_bid: bool,
    pub self_matching_option: Option<SelfMatchingOptions>,
    pub pay_with_deep: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ProposalParams {
    pub pool_key: String,
    pub balance_manager_key: String,
    pub taker_fee: f64,
    pub maker_fee: f64,
    pub stake_required: f64,
}

#[derive(Debug, Clone)]
pub struct SwapParams {
    pub pool_key: String,
    pub amount: f64,
    pub deep_amount: f64,
    pub min_out: f64,
}

#[derive(Debug, Clone)]
pub struct CreatePoolAdminParams {
    pub base_coin_key: String,
    pub quote_coin_key: String,
    pub tick_size: f64,
    pub lot_size: f64,
    pub min_size: f64,
    pub whitelisted: bool,
    pub stable_pool: bool,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub deepbook_package_id: String,
    pub registry_id: String,
    pub deep_treasury_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Mainnet,
    Testnet,
}

/// Represents an account in the DeepBook system
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    pub epoch: u64,
    pub open_orders: VecSet<u128>,
    pub taker_volume: u128,
    pub maker_volume: u128,
    pub active_stake: u64,
    pub inactive_stake: u64,
    pub created_proposal: bool,
    pub voted_proposal: Option<ID>,
    pub unclaimed_rebates: Balances,
    pub settled_balances: Balances,
    pub owed_balances: Balances,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Balances {
    pub base: u64,
    pub quote: u64,
    pub deep: u64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct OrderDeepPrice {
    pub asset_is_base: bool,
    pub deep_per_asset: u64,
}
