use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{LookupMap};

near_sdk::setup_alloc!();

use crate::request::Request;

mod request;
mod errors;
mod authorized_senders;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ChainScore {
    nonces: LookupMap<AccountId, u128>,
    requests: LookupMap<Vec<u8>, Request>
}

#[near_bindgen]
impl ChainScore {

}