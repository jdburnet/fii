use std::error::Error;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Month {
    name: String,
    income: u32,
    expenses: u16,
    investments: u32,
}

impl Month {
    pub fn new(name: String, inc: u32, exp: u16, inv: u32) -> Month {
        Month {
            name: name,
            income: inc,
            expenses: exp,
            investments: inv,
        }
    }

    pub fn investment_income(&self, percent: u8) -> f64 {
        f64::from(self.investments) / 100. * f64::from(percent)
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:9}: inc={:6.2} exp={:6.2} inv={:6.2} invinc={:6.2}",
            self.name, self.income, self.expenses,
            self.investments, self.investment_income(4)
        )
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Year {
    id: u16,
    months: Vec<Month>,
}

impl Year {
    pub fn new(year: u16) -> Year {
        Year {
            id: year,
            months: Vec::new(),
        }
    }

    pub fn add_month(&mut self, m: Month) {
        self.months.push(m);
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct History {
    years: Vec<Year>,
}

impl History {
    pub fn new() -> History {
        History{ years: Vec::new() }
    }

    pub fn add_year(&mut self, y: Year) {
        self.years.push(y);
    }
}

#[cfg(test)]
mod tests {
    use toml;

    use TEST_INCOME;
    use TEST_EXPENSES;
    use TEST_INVESTMENTS;
    use TEST_YEAR;
    use TEST_MONTHLY_WITHDRAWL;

    use super::*;

    #[test]
    fn month_equality() {
        let m = Month::new(
            String::from("january"),
            TEST_INCOME,
            TEST_EXPENSES,
            TEST_INVESTMENTS,
        );
        assert_eq!(m, m)
    }

    #[test]
    fn toml_serde_month_unity() {
        let m = Month::new(
            String::from("january"),
            TEST_INCOME,
            TEST_EXPENSES,
            TEST_INVESTMENTS,
        );
        let m_ser = toml::to_string(&m).unwrap();
        let m_de = toml::from_str(&m_ser).unwrap();

        assert_eq!(m, m_de)
    }

    #[test]
    fn calc_investment_income_4_is_ok() {
        let m = Month::new(
            String::from("january"),
            TEST_INCOME,
            TEST_EXPENSES,
            TEST_INVESTMENTS,
        );
        let invinc = m.investment_income(TEST_MONTHLY_WITHDRAWL);
        let exp = f64::from(TEST_INVESTMENTS) / 100. * f64::from(TEST_MONTHLY_WITHDRAWL);
        assert_eq!(invinc, exp)
    }

    #[test]
    fn make_month() {
        let m = Month {
            name: String::from("january"),
            income: TEST_INCOME,
            expenses: TEST_EXPENSES,
            investments: TEST_INVESTMENTS,
        };
        assert_eq!(m.income, TEST_INCOME)
    }

    #[test]
    fn make_month_new() {
        let m = Month::new(
            String::from("january"),
            TEST_INCOME,
            TEST_EXPENSES,
            TEST_INVESTMENTS,
        );
        assert_eq!(m.income, TEST_INCOME)
    }

    #[test]
    fn year_equality() {
        let y = Year::new(TEST_YEAR);
        assert_eq!(y, y)
    }

    #[test]
    fn toml_serde_year_unity() {
        let y = Year::new(TEST_YEAR);
        let y_ser = toml::to_string(&y).unwrap();
        let y_de = toml::from_str(&y_ser).unwrap();

        assert_eq!(y, y_de)
    }

    #[test]
    fn make_year() {
        let y = Year {
            id: TEST_YEAR,
            months: Vec::new(),
        };
        assert_eq!(y.id, TEST_YEAR)
    }

    #[test]
    fn make_year_new() {
        let y = Year::new(TEST_YEAR);
        assert_eq!(y.id, TEST_YEAR)
    }

    #[test]
    fn add_month_to_year() {
        let m = Month {
            name: String::from("january"),
            income: TEST_INCOME,
            expenses: TEST_EXPENSES,
            investments: TEST_INVESTMENTS,
        };
        let mut y = Year::new(TEST_YEAR);
        y.add_month(m);
        assert_eq!(y.months[0].income, TEST_INCOME)
    }

    #[test]
    fn add_year_to_history() {
        let y = Year::new(TEST_YEAR);
        let mut h = History::new();
        h.add_year(y);
        assert_eq!(h.years[0].id, TEST_YEAR)
    }

    #[test]
    fn toml_serde_history_unity() {
        let y = Year::new(TEST_YEAR);
        let mut h = History::new();
        h.years.push(y);
        let h_ser = toml::to_string(&h).unwrap();
        let h_de = toml::from_str(&h_ser).unwrap();

        assert_eq!(h, h_de)
    }

    #[test]
    fn make_history_new() {
        History::new();
    }
}
