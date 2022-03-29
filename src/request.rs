use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, CryptoHash,
    PromiseOrValue
};

use crate::errors::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Request {
    id: Vec<u8>,

    sender: AccountId,
    spec_id: CryptoHash,
    data_version: u64,
    payment: Balance,

    callback_contract: AccountId,
    callback_function_id: Vec<u8>,

    cancel_expiration: u64,

    confirmations: Vec<Confirmation>,
    final_data: Option<u128>
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct RequestId {
    account: AccountId,
    nonce: u128,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Confirmation {
    confirmation_id: CryptoHash,
    from: AccountId,
    data: u128
}

impl Request {
    pub fn new(
        nonce: u128,

        sender: AccountId,
        spec_id: CryptoHash,
        data_version: u64,
        payment: Balance,

        callback_contract: AccountId,
        callback_function_id: Vec<u8>,

        cancel_expiration: u64
    ) -> Self {
        assert!(cancel_expiration > env::block_timestamp(), "{}", ERR26_INCORRECT_NONCE);

        let request_id: RequestId = RequestId {
            account: sender.clone(),
            nonce
        };

        let request_id_encoded = request_id.try_to_vec().unwrap();

        Self {
            id: env::keccak256(request_id_encoded.as_slice()),

            sender,
            spec_id,
            data_version,
            payment,

            callback_contract,
            callback_function_id,

            cancel_expiration,
            confirmations: Vec::new(),
            final_data: None
        }
    }

    pub fn add_confirmation(&self, conf: Confirmation) {
        
    }
}

