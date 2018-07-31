extern crate app_dirs;
extern crate clap;
extern crate time;
extern crate toml;
#[macro_use]
extern crate serde_derive;

pub mod core;
pub mod data;

pub const TEST_INCOME: i32 = 12_345;
pub const TEST_EXPENSES: i32 = 6_789;
pub const TEST_INVESTMENTS: i32 = 1_234_567;
pub const TEST_YEAR: i32 = 2018;
pub const TEST_MONTHLY_WITHDRAWL: i32 = 4;
