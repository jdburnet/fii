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

    #[test]
    fn make_month() {
        let m = Month {
            name: String::from("january"),
            income: 12_345,
            expenses: 6_789,
            investments: 1_234_567,
        };
        assert_eq!(m.income, 12_345);
    }

    #[test]
    fn make_month_new() {
        let m = Month::new(
            String::from("january"),
            12_345,
            6_789,
            1_234_567,
        );
        assert_eq!(m.income, 12_345);
    }

    #[test]
    fn make_year() {
        let y = Year {
            id: 2018,
            months: Vec::new(),
        };
        assert_eq!(y.id, 2018);
    }

    #[test]
    fn make_year_new() {
        let y = Year::new(2018);
        assert_eq!(y.id, 2018);
    }
}
