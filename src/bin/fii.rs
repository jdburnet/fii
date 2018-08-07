use std::fs::File;
use std::io;
use std::path::PathBuf;

extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate time;
extern crate toml;

extern crate fii;

use app_dirs::{AppDataType, app_root};
use clap::{App, Arg, SubCommand};

use fii::core::{Year, Month, History};
use fii::data::{APP, save, load};

const ADDMONTH: &str = "add-month";
const SHOWMONTH: &str = "show-month";
const DATAFILE: &str = "data";

fn start_new() -> History {
    let n = time::now();
    let y = Year::new(n.tm_year + 1900);  // tm_year 'years since 1900'
    let mut h = History::new();
    h.add_year(y);
    h
}

fn get_datapath(datafile: &str) -> PathBuf {
    let mut datapath = app_root(AppDataType::UserData, &APP).unwrap();
    datapath.push(datafile);
    datapath
}

fn load_history(datafile: &PathBuf) -> History {
    match load(datafile.to_str().unwrap()) {
        Err(e) => panic!("unable to load data file: {:?}", e),
        Ok(ref s) if *s == String::from("") => start_new(),
        Ok(s) => toml::from_str(&s).expect("data file has been corrupted"),
    }
}

fn insert_month(year_id: i32, m: Month, datafile: &PathBuf) -> io::Result<()> {
    let mut hist = load_history(datafile);
    let mut y = match hist.years.iter().position(|year| year.id == year_id) {
        Some(idx) => hist.years.remove(idx),
        None => Year::new(year_id),
    };
    y.add_month(m);
    hist.add_year(y);
    let mut datafile = File::create(datafile.to_str().unwrap())
        .expect("unable to create data file");
    save(&mut datafile, &toml::to_string(&hist).unwrap())
}

fn main() {
    let args = App::new("fii")
        .about("Track your FI progress")
        .subcommand(SubCommand::with_name(ADDMONTH)
            .about("input information about a given month")
            .arg(Arg::with_name("year")
                .help("year containing month of interest")
                .required(true))
            .arg(Arg::with_name("month")
                .help("first three letters of month (e.g. jan, apr, mar)")
                .required(true))
            .arg(Arg::with_name("income")
                .help("amount that came in this month")
                .required(true))
            .arg(Arg::with_name("expenses")
                .help("amount that went out this month")
                .required(true))
            .arg(Arg::with_name("investments")
                .help("size of total investments at the end of the month")
                .required(true)))
        .subcommand(SubCommand::with_name(SHOWMONTH)
            .about("show information about a given month")
            .arg(Arg::with_name("year")
                .help("Year containing month of interest")
                .required(true))
            .arg(Arg::with_name("month")
                .help("first three letters of month (e.g. jan, apr, mar)")
                .required(true)))
        .get_matches();

    match args.subcommand() {
        (ADDMONTH, Some(matches)) => {
            let m = Month::new(
                value_t!(matches, "month", String).unwrap(),
                value_t!(matches, "income", i32).unwrap(),
                value_t!(matches, "expenses", i32).unwrap(),
                value_t!(matches, "investments", i32).unwrap(),
            );
            let datapath = get_datapath(DATAFILE);
            insert_month(value_t!(matches, "year", i32).unwrap(), m, &datapath);
        },
        (SHOWMONTH, Some(matches)) => {
            let datapath = get_datapath(DATAFILE);
            let hist = load_history(&datapath);
            let year = hist.years
                .iter()
                .filter(|x| x.id == value_t!(matches, "year", i32).unwrap())
                .collect::<Vec<&Year>>()[0];
            let month = year.months
                .iter()
                .filter(|x| x.name == value_t!(matches, "month", String).unwrap())
                .collect::<Vec<&Month>>()[0];
            println!("{}", month);
        },
        _ => println!("{}", args.usage()),
    };
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::env::temp_dir;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    use fii::{TEST_INCOME, TEST_EXPENSES, TEST_INVESTMENTS, TEST_YEAR};

    use super::*;

    const TESTFILE: &str = "test";

    fn time_seed() -> u64 {
        let now = SystemTime::now();
        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        hasher.finish()
    }

    fn test_path() -> PathBuf {
        let mut p = temp_dir();
        p.push(&time_seed().to_string());
        p
    }

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
        let h = start_new();
        let exp = time::now().tm_year + 1900;
        assert_eq!(h.years[0].id, exp)
    }

    #[test]
    fn add_one_month() {
        let p = test_path();
        let jan = get_month(String::from("jan"));
        insert_month(TEST_YEAR, jan, &p);
        let h = load_history(&p);
        assert_eq!(h.years[0].months.len(), 1)
    }

    #[test]
    fn add_two_months() {
        let p = test_path();
        let jan = get_month(String::from("jan"));
        insert_month(TEST_YEAR, jan, &p);
        let feb = get_month(String::from("feb"));
        insert_month(TEST_YEAR, feb, &p);
        let h = load_history(&p);
        assert_eq!(h.years[0].months.len(), 2)
    }

    #[test]
    fn add_same_month_twice() {
        let p = test_path();
        let jan = get_month(String::from("jan"));
        insert_month(TEST_YEAR, jan, &p);
        let jan = get_month(String::from("jan"));
        insert_month(TEST_YEAR, jan, &p);
        let h = load_history(&p);
        assert_eq!(h.years[0].months.len(), 1)
    }

    #[test]
    fn add_new_year() {
        let p = test_path();
        let jan = get_month(String::from("jan"));
        insert_month(TEST_YEAR, jan, &p);
        let jan = get_month(String::from("jan"));
        insert_month(TEST_YEAR + 1, jan, &p);
        let h = load_history(&p);
        assert_eq!(h.years.len(), 2);
        assert_eq!(h.years[0].months.len(), 1);
        assert_eq!(h.years[1].months.len(), 1);
    }
}
