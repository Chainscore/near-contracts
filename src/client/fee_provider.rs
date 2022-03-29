use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance};
use near_sdk::collections::{LookupMap};

near_sdk::setup_alloc!();

use crate::request::Request;

mod request;
mod errors;
mod authorized_senders;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FeeProvider {
    fees: LookupMap<SpecId, Balance>
}

#[near_bindgen]
impl FeeProvider {
    #[init]
    pub fn init() -> Self {
        Self {
            fees: LookupMap::new()
        }
    }

    pub fn set_fee(&self, spec_id: SpecId, new_fee: Balance) {
        &self.fees[spec_id] = new_fee;
    }

    pub fn get_fee(&self, spec_id: SpecId) {
        &self.fees[spec_id] = new_fee;
    }
}