extern crate fii;

use fii::core::{Year, Month, History};
use fii::data::{APP, save};

fn start_new(y: Year) -> History {
    let mut h = History::new();
    h.add_year(y);
    h
}

fn main() {
    let jan = Month::new(String::from("jan"), 123, 456, 789);
    let mut year = Year::new(2018);
    year.add_month(jan);
    let hist = start_new(year);

    hist.show_year(2018).expect("unable to show year");
}

#[cfg(test)]
mod tests {
    use std::panic;

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
