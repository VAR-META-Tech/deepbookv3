// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_sdk::types::transaction::Transaction;

use crate::utils::config::DeepBookConfig;

#[derive(Debug, Clone)]
pub struct BalanceManagerContract {
    config: DeepBookConfig,
}

impl BalanceManagerContract {
    pub fn new(config: DeepBookConfig) -> Self {
        Self { config }
    }

    // pub fn create_and_share_balance_manager(&self) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         let manager = tx.move_call(
    //             format!("{}::balance_manager::new", config.deepbook_package_id),
    //             vec![],
    //         );
    //         tx.move_call(
    //             "0x2::transfer::public_share_object".to_string(),
    //             vec![manager],
    //         );
    //     }
    // }

    // pub fn deposit_into_manager(
    //     &self,
    //     manager_key: &str,
    //     coin_key: &str,
    //     amount_to_deposit: f64,
    // ) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         tx.set_sender_if_not_set(config.address);
    //         let manager_id = config.get_balance_manager(manager_key).address.clone();
    //         let coin = config.get_coin(coin_key);
    //         let deposit_input = (amount_to_deposit * coin.scalar as f64) as u64;
    //         let deposit = coin_with_balance(coin.coin_type.clone(), deposit_input);
    //         tx.move_call(
    //             format!("{}::balance_manager::deposit", config.deepbook_package_id),
    //             vec![tx.object(manager_id.clone()), deposit],
    //         );
    //     }
    // }

    // pub fn withdraw_from_manager(
    //     &self,
    //     manager_key: &str,
    //     coin_key: &str,
    //     amount_to_withdraw: f64,
    //     recipient: &str,
    // ) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         let manager_id = config.get_balance_manager(manager_key).address.clone();
    //         let coin = config.get_coin(coin_key);
    //         let withdraw_input = (amount_to_withdraw * coin.scalar as f64) as u64;
    //         let coin_object = tx.move_call(
    //             format!("{}::balance_manager::withdraw", config.deepbook_package_id),
    //             vec![tx.object(manager_id.clone()), tx.pure_u64(withdraw_input)],
    //         );
    //         tx.transfer_objects(vec![coin_object], recipient.to_string());
    //     }
    // }

    // pub fn withdraw_all_from_manager(
    //     &self,
    //     manager_key: &str,
    //     coin_key: &str,
    //     recipient: &str,
    // ) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         let manager_id = config.get_balance_manager(manager_key).address.clone();
    //         let coin = config.get_coin(coin_key);
    //         let withdrawal_coin = tx.move_call(
    //             format!(
    //                 "{}::balance_manager::withdraw_all",
    //                 config.deepbook_package_id
    //             ),
    //             vec![tx.object(manager_id.clone())],
    //         );
    //         tx.transfer_objects(vec![withdrawal_coin], recipient.to_string());
    //     }
    // }

    // pub fn check_manager_balance(
    //     &self,
    //     manager_key: &str,
    //     coin_key: &str,
    // ) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         let manager_id = config.get_balance_manager(manager_key).address.clone();
    //         let coin = config.get_coin(coin_key);
    //         tx.move_call(
    //             format!("{}::balance_manager::balance", config.deepbook_package_id),
    //             vec![tx.object(manager_id.clone())],
    //         );
    //     }
    // }

    // pub fn generate_proof(&self, manager_key: &str) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         let balance_manager = config.get_balance_manager(manager_key);
    //         if let Some(trade_cap) = &balance_manager.trade_cap {
    //             tx.add(
    //                 self.generate_proof_as_trader(
    //                     balance_manager.address.clone(),
    //                     trade_cap.clone(),
    //                 ),
    //             );
    //         } else {
    //             tx.add(self.generate_proof_as_owner(balance_manager.address.clone()));
    //         }
    //     }
    // }

    // pub fn generate_proof_as_owner(&self, manager_id: String) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         tx.move_call(
    //             format!(
    //                 "{}::balance_manager::generate_proof_as_owner",
    //                 config.deepbook_package_id
    //             ),
    //             vec![tx.object(manager_id.clone())],
    //         );
    //     }
    // }

    // pub fn generate_proof_as_trader(
    //     &self,
    //     manager_id: String,
    //     trade_cap_id: String,
    // ) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         tx.move_call(
    //             format!(
    //                 "{}::balance_manager::generate_proof_as_trader",
    //                 config.deepbook_package_id
    //             ),
    //             vec![
    //                 tx.object(manager_id.clone()),
    //                 tx.object(trade_cap_id.clone()),
    //             ],
    //         );
    //     }
    // }

    // pub fn owner(&self, manager_key: &str) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         let manager_id = config.get_balance_manager(manager_key).address.clone();
    //         tx.move_call(
    //             format!("{}::balance_manager::owner", config.deepbook_package_id),
    //             vec![tx.object(manager_id.clone())],
    //         );
    //     }
    // }

    // pub fn id(&self, manager_key: &str) -> impl Fn(&mut Transaction) {
    //     let config = self.config.clone();
    //     move |tx: &mut Transaction| {
    //         let manager_id = config.get_balance_manager(manager_key).address.clone();
    //         tx.move_call(
    //             format!("{}::balance_manager::id", config.deepbook_package_id),
    //             vec![tx.object(manager_id.clone())],
    //         );
    //     }
    // }
}
