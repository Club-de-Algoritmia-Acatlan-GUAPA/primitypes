pub const USERNAME_MIN_SIZE: usize = 5;
pub const USERNAME_MAX_SIZE: usize = 25;
pub const USERNAME_FORBIDDEN_CHARACTERS: [char; 10] =
    ['@', '<', '>', '/', '\\', '{', '}', '(', ')', '"'];

pub const MAX_SUBMISSION_FILE_SIZE_IN_BYTES: usize = 71680;
pub const MAX_SUBMISSION_FILE_SIZE_IN_KB: usize = 70;
pub const SUBMISSION_ID_BITS: u128 = 128;
pub const TIMESTAMP_BITS: u128 = 41;
pub const CONTEST_ID_BITS: u128 = 26;
pub const PROBLEM_ID_BITS: u128 = 32;
pub const UUID_TIME_MID_BITS: u128 = 16;
pub const EMPTY_BITS: u128 =
    SUBMISSION_ID_BITS - (TIMESTAMP_BITS + CONTEST_ID_BITS + PROBLEM_ID_BITS + UUID_TIME_MID_BITS);
