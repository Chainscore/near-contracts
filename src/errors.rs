// Storage errors.
pub const ERR10_ACC_NOT_REGISTERED: &str = "E10: account not registered";
pub const ERR11_INSUFFICIENT_STORAGE: &str = "E11: insufficient $NEAR storage deposit";
pub const ERR12_TOKEN_NOT_WHITELISTED: &str = "E12: token not whitelisted";
pub const ERR13_LP_NOT_REGISTERED: &str = "E13: LP not registered";
pub const ERR14_LP_ALREADY_REGISTERED: &str = "E14: LP already registered";
pub const ERR15_NO_STORAGE_CAN_WITHDRAW: &str = "E15: no storage can withdraw";
pub const ERR16_STORAGE_WITHDRAW_TOO_MUCH: &str = "E16: storage withdraw too much";
pub const ERR17_DEPOSIT_LESS_THAN_MIN_STORAGE: &str = "E17: deposit less than min storage";
pub const ERR18_TOKENS_NOT_EMPTY: &str = "E18: storage unregister tokens not empty";

// Accounts.
pub const ERR21_TOKEN_NOT_REG: &str = "E21: token not registered";
pub const ERR22_NOT_ENOUGH_TOKENS: &str = "E22: not enough tokens in deposit";
// pub const ERR23_NOT_ENOUGH_NEAR: &str = "E23: not enough NEAR in deposit";
pub const ERR24_NON_ZERO_TOKEN_BALANCE: &str = "E24: non-zero token balance";
pub const ERR25_CALLBACK_POST_WITHDRAW_INVALID: &str =
    "E25: expected 1 promise result from withdraw";

// Request
pub const ERR26_INCORRECT_NONCE: &str = "ERR26: Incorrent request nonce";
pub const ERR27_INVALID_EXPIRATION: &str = "ERR27: Invalid request expiration time";

// Action result.
pub const ERR41_WRONG_ACTION_RESULT: &str = "E41: wrong action result type";

// Contract Level
pub const ERR51_CONTRACT_PAUSED: &str = "E51: contract paused";

// owner
pub const ERR100_NOT_ALLOWED: &str = "E100: no permission to invoke this";
pub const ERR101_ILLEGAL_FEE: &str = "E101: illegal fee";
pub const ERR102_INVALID_TOKEN_ID: &str = "E102: invalid token id";
pub const ERR103_NOT_INITIALIZED: &str = "E103: contract is not initialized";


//mft
pub const ERR110_INVALID_REGISTER: &str = "E110: Invalid register";
