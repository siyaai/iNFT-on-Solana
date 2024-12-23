// Pda Seed Constants
pub const PROGRAM_STATE_SEED: &[u8] = b"program-state";
pub const COLLECTION_CONFIG_SEED: &[u8] = b"collection_config";

//  
pub const MAX_URI_LENGTH: usize = 200;
pub const MAX_NAME_LENGTH: usize = 32;
pub const MAX_SYMBOL_LENGTH: usize = 10;
pub const MIN_NAME_LENGTH: usize = 3;
pub const MIN_SYMBOL_LENGTH: usize = 2;

// supply
pub const MIN_MAX_SUPPLY: u64 = 1;
pub const MAX_MAX_SUPPLY: u64 = 10000;

// fee
pub const MAX_ROYALTY_BASIS_POINTS: u16 = 10000; // 100%
pub const REDEEM_FEE_BPS: u16 = 500; // 5%
pub const MAX_CREATOR_SHARE: u8 = 100;

// queue
pub const MAX_QUEUE_SIZE: usize = 10_000;

// token
pub const FEE_RECEIVER: &str = "DfD5WCDk11NwW1uirpvCszKSvfnAJUR3xHoFJ9E1noAn";

