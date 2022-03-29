use near_sdk::{BorshSerialize, BorshDeserialize};
use near_sdk::{env, AccountId};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ChainScoreClient{
    requests: Vec<Request>,
    cs_token: AccountId,
    cs_contract: AccountId,
    fee_provider: AccountId
}

pub struct Request{
    data: str,
    cancel_expiration: u64,
}

impl ChainScoreClient {
    pub fn create_request() {}

    pub fn send_request() {}

    pub fn cancel_request() {}

    pub fn resolve_request() {}

    pub fn set_cs_contract() {}
}