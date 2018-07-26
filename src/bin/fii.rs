extern crate clap;

extern crate fii;

use clap::{App, SubCommand};

use fii::core::{Year, Month, History};
use fii::data::{APP, save};

const ADDMONTH: &str = "add-month";
const SHOWMONTH: &str = "show-month";

fn start_new(y: Year) -> History {
    let mut h = History::new();
    h.add_year(y);
    h
}

fn main() {
    let args = App::new("fii")
        .about("Track your FI progress")
        .subcommand(SubCommand::with_name(ADDMONTH)
            .about("input information about a given month"))
        .subcommand(SubCommand::with_name(SHOWMONTH)
            .about("show information about a given month"))
        .get_matches();

    // TODO get help out of app and then get matches
    // use print help message if no subcommand is given

    match args.subcommand() {
        (ADDMONTH, Some(matches)) => {},
        (SHOWMONTH, Some(matches)) => {},
        _ => println!("{}", args.usage()),
    }
}

#[cfg(test)]
mod tests {
    use fii::{TEST_INCOME, TEST_EXPENSES, TEST_INVESTMENTS, TEST_YEAR};

    use super::*;

    fn get_month(n: String) -> Month {
        Month::new(
            n,
            TEST_INCOME,
            TEST_EXPENSES,
            TEST_INVESTMENTS,
        )
    }

    fn get_year() -> Year {
        let m = get_month(String::from("january"));
        let mut y = Year::new(TEST_YEAR);
        y.add_month(m);
        y
    }

    #[test]
    fn able_to_start_new() {
        let y = get_year();
        let exp = y.id;
        let h = start_new(y);
        assert_eq!(h.years[0].id, exp)
    }
}
