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
}

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
    use super::*;

    const INCOME: u32 = 12_345;
    const EXPENSES: u16 = 6_789;
    const INVESTMENTS: u32 = 1_234_567;
    const YEAR: u16 = 2018;
    const MONTHLY_WITHDRAWL: u8 = 4;

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
