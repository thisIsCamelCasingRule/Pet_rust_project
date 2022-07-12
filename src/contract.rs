use std::cmp::Ordering;

use crate::account::VAccount;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{env, Balance, PanicOnDefault};
use near_sdk::{near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pub(crate) owner_id: AccountId,
    pub(crate) accounts: LookupMap<AccountId, VAccount>,
}

#[near_bindgen]
impl Contract {}
