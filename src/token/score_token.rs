/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault,
    PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "<svg width='1080' height='1080' viewBox='0 0 1080 1080' fill='none' xmlns='http://www.w3.org/2000/svg'><g clip-path='url(#clip0_605_141)'><rect width='1080' height='1080' fill='url(#paint0_linear_605_141)'/><circle cx='1833.83' cy='959.828' r='418.173' transform='rotate(29.1239 1833.83 959.828)' fill='#803A3A'/><g filter='url(#filter1_f_605_141)'><ellipse cx='142.04' cy='24.9419' rx='559.615' ry='618.815' transform='rotate(54.5131 142.04 24.9419)' fill='url(#paint1_linear_605_141)'/></g><path fill-rule='evenodd' clip-rule='evenodd' d='M431.685 242.04C487.044 220.855 547.217 215.536 605.434 226.683C663.651 237.83 717.603 265 761.221 305.136C804.839 345.273 836.392 396.784 852.332 453.875C857.401 472.03 846.793 490.856 828.638 495.924C810.484 500.993 791.658 490.385 786.589 472.23C774.065 427.373 749.273 386.9 715.002 355.364C680.731 323.828 638.34 302.48 592.598 293.722C546.856 284.964 499.577 289.143 456.081 305.789C412.584 322.435 374.595 350.887 346.387 387.945C318.178 425.004 300.87 469.198 296.408 515.557C291.947 561.916 300.509 608.6 321.132 650.358C341.755 692.116 373.62 727.291 413.144 751.927C452.668 776.562 498.281 789.681 544.854 789.808C563.703 789.859 578.941 805.18 578.89 824.028C578.839 842.877 563.518 858.115 544.669 858.064C485.395 857.904 427.341 841.207 377.038 809.852C326.735 778.498 286.179 733.729 259.932 680.583C233.684 627.436 222.787 568.02 228.465 509.018C234.144 450.016 256.173 393.769 292.074 346.603C327.976 299.438 376.325 263.226 431.685 242.04Z' fill='white'/><path opacity='0.6' fill-rule='evenodd' clip-rule='evenodd' d='M532.83 296.321C583.511 293.722 633.743 306.969 676.553 334.22C719.362 361.472 752.624 401.377 771.721 448.395C790.817 495.412 794.8 547.209 783.117 596.594C778.778 614.936 760.39 626.287 742.048 621.948C723.706 617.609 712.354 599.221 716.693 580.879C725.113 545.29 722.243 507.963 708.481 474.08C694.719 440.197 670.749 411.439 639.898 391.8C609.048 372.161 572.848 362.616 536.325 364.488C499.801 366.361 464.767 379.559 436.087 402.25C407.407 424.941 386.503 455.999 376.279 491.112C366.054 526.225 367.016 563.65 379.031 598.191C391.046 632.732 413.518 662.675 443.327 683.862C473.135 705.05 508.801 716.43 545.372 716.424C564.221 716.42 579.503 731.697 579.507 750.546C579.51 769.395 564.233 784.677 545.384 784.681C494.637 784.69 445.145 768.898 403.782 739.497C362.419 710.097 331.236 668.546 314.563 620.616C297.891 572.685 296.556 520.753 310.743 472.029C324.931 423.305 353.938 380.207 393.736 348.72C433.534 317.233 482.149 298.919 532.83 296.321Z' fill='white'/><path opacity='0.3' fill-rule='evenodd' clip-rule='evenodd' d='M483.555 381.54C508.122 371.999 534.569 368.296 560.812 370.723C587.055 373.149 612.375 381.638 634.777 395.521C650.798 405.45 655.737 426.487 645.808 442.508C635.879 458.53 614.842 463.469 598.821 453.54C585.406 445.226 570.243 440.143 554.528 438.69C538.813 437.237 522.975 439.454 508.264 445.167C493.552 450.88 480.368 459.933 469.753 471.611C459.137 483.289 451.379 497.274 447.091 512.462C442.802 527.65 442.101 543.627 445.041 559.133C447.982 574.638 454.484 589.249 464.036 601.813C473.587 614.376 485.926 624.549 500.081 631.53C514.235 638.511 529.817 642.108 545.599 642.038C564.448 641.954 579.795 657.166 579.879 676.015C579.962 694.863 564.75 710.211 545.902 710.294C519.547 710.411 493.526 704.404 469.89 692.747C446.254 681.09 425.648 664.102 409.698 643.122C393.748 622.142 382.89 597.744 377.98 571.851C373.069 545.958 374.241 519.278 381.402 493.915C388.563 468.552 401.518 445.199 419.245 425.698C436.973 406.197 458.988 391.08 483.555 381.54Z' fill='white'/></g><defs><filter id='filter1_f_605_141' x='-759.551' y='-857.323' width='1803.18' height='1764.53' filterUnits='userSpaceOnUse' color-interpolation-filters='sRGB'><feFlood flood-opacity='0' result='BackgroundImageFix'/><feBlend mode='normal' in='SourceGraphic' in2='BackgroundImageFix' result='shape'/><feGaussianBlur stdDeviation='151' result='effect1_foregroundBlur_605_141'/></filter><linearGradient id='paint0_linear_605_141' x1='14.9063' y1='19' x2='1284.75' y2='415.846' gradientUnits='userSpaceOnUse'><stop stop-color='#333352'/><stop offset='0.213542' stop-color='#15324B'/><stop offset='0.515625' stop-color='#112135'/><stop offset='0.776042' stop-color='#1A414F'/><stop offset='1' stop-color='#326473'/></linearGradient><linearGradient id='paint1_linear_605_141' x1='560.167' y1='-348.692' x2='75.4992' y2='338.005' gradientUnits='userSpaceOnUse'><stop stop-color='#8C76A9'/><stop offset='1' stop-color='#8C76A9' stop-opacity='0'/></linearGradient><clipPath id='clip0_605_141'><rect width='1080' height='1080' fill='white'/></clipPath></defs></svg>";


#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FungibleToken,
    Metadata,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Example NEAR fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(StorageKey::FungibleToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        this
    }

    // TODO: Test closing account
    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}