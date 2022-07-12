use crate::*;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;
use near_sdk::{env, AccountId, Balance};

#[derive(BorshDeserialize, BorshSerialize)]
pub(crate) struct Account {
    pub(crate) storage_balance: U128,
    pub(crate) storage_used: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub(crate) enum VAccount {
    Current(Account),
}

impl VAccount {}
