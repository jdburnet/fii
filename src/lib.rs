extern crate app_dirs;
extern crate clap;
extern crate toml;
#[macro_use]
extern crate serde_derive;

pub mod core;
pub mod data;

pub const TEST_INCOME: u32 = 12_345;
pub const TEST_EXPENSES: u16 = 6_789;
pub const TEST_INVESTMENTS: u32 = 1_234_567;
pub const TEST_YEAR: u16 = 2018;
pub const TEST_MONTHLY_WITHDRAWL: u8 = 4;
