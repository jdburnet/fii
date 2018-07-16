#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct Month {
    name: String,
    income: u32,
    expenses: u16,
    investments: u32,
}

impl Month {
    fn new(name: String, inc: u32, exp: u16, inv: u32) -> Month {
        Month {
            name: name,
            income: inc,
            expenses: exp,
            investments: inv,
        }
    }

    fn investment_income(&self, percent: u8) -> f64 {
        f64::from(self.investments) / 100. * f64::from(percent)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct Year {
    id: u16,
    months: Vec<Month>,
}

impl Year {
    fn new(year: u16) -> Year {
        Year {
            id: year,
            months: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use toml;

    use super::*;

    const INCOME: u32 = 12_345;
    const EXPENSES: u16 = 6_789;
    const INVESTMENTS: u32 = 1_234_567;
    const YEAR: u16 = 2018;
    const MONTHLY_WITHDRAWL: u8 = 4;

    #[test]
    fn month_equality() {
        let m = Month::new(
            String::from("january"),
            INCOME,
            EXPENSES,
            INVESTMENTS,
        );
        assert_eq!(m, m);
    }

    #[test]
    fn toml_serde_month_unity() {
        let m = Month::new(
            String::from("january"),
            INCOME,
            EXPENSES,
            INVESTMENTS,
        );
        let m_ser = toml::to_string(&m).unwrap();
        let m_de = toml::from_str(&m_ser).unwrap();

        assert_eq!(m, m_de)
    }

    #[test]
    fn calc_investment_income_4_is_ok() {
        let m = Month::new(
            String::from("january"),
            INCOME,
            EXPENSES,
            INVESTMENTS,
        );
        let invinc = m.investment_income(MONTHLY_WITHDRAWL);
        let exp = f64::from(INVESTMENTS) / 100. * f64::from(MONTHLY_WITHDRAWL);
        assert_eq!(invinc, exp);
    }

    #[test]
    fn make_month() {
        let m = Month {
            name: String::from("january"),
            income: INCOME,
            expenses: EXPENSES,
            investments: INVESTMENTS,
        };
        assert_eq!(m.income, INCOME);
    }

    #[test]
    fn make_month_new() {
        let m = Month::new(
            String::from("january"),
            INCOME,
            EXPENSES,
            INVESTMENTS,
        );
        assert_eq!(m.income, INCOME);
    }

    #[test]
    fn year_equality() {
        let y = Year::new(YEAR);
        assert_eq!(y, y);
    }

    #[test]
    fn toml_serde_year_unity() {
        let y = Year::new(YEAR);
        let y_ser = toml::to_string(&y).unwrap();
        let y_de = toml::from_str(&y_ser).unwrap();

        assert_eq!(y, y_de);
    }

    #[test]
    fn make_year() {
        let y = Year {
            id: YEAR,
            months: Vec::new(),
        };
        assert_eq!(y.id, YEAR);
    }

    #[test]
    fn make_year_new() {
        let y = Year::new(YEAR);
        assert_eq!(y.id, YEAR);
    }
}