use crate::*;

pub const REGISTER_BOND: Balance = 500_000_000_000_000_000_000_000; // 0.5 Near for register, worker can withdraw when they left
pub const SUBMIT_BOND: Balance = 10_000_000_000_000_000_000_000; // 0.01 Near for each submission
pub const MINIMUM_PRICE_PER_TASK: Balance = 10_000_000_000_000_000_000_000;
pub const MAXIMUM_PRICE_PER_TASK: Balance = 100_000_000_000_000_000_000_000_000; // MAXIMUM 100 Near per task
pub const MAXIMUM_DESCRIPTION_LENGTH: usize = 10000;
pub const MAXIMUM_COVER_LETTER_LENGTH: usize = 10000;
pub const MAXIMUM_PROPOSAL_AT_ONE_TIME: u16 = 200;
pub const MAXIMUM_REQUEST_ACTIVE_PER_USER: u16 = 10;
pub const DEFAULT_GAS_TO_PAY: Gas = 20_000_000_000_000;
pub const MAX_TITLE_LENGTH: usize = 100;
