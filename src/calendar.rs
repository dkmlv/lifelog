use std::fs;
use std::path::PathBuf;

use chrono::{Date, Local, TimeZone};
use cursive::reexports::time::{util::days_in_year_month, Month as tMonth};

use super::month_log;

/// Read data dir and return a tuple containing earliest and latest date.
///
/// For example, if the data dir looks like:
///
/// ```text
/// lifelog
/// ├── 2022
/// │   ├── January.json
/// │   ├── February.json
/// │   └── ...
/// └── 2023
///     ├── January.json
///     ├── ...
///     └── October.json
/// ```
///
/// This function should return: (2022-01-01[offset], 2023-10-31[offset]).
///
/// The function is going to be used to set earliest and latest dates for the
/// calendar view.
pub fn earliest_latest() -> (Date<Local>, Date<Local>) {
    // get earliest year and latest year
    let data_dir = month_log::data_dir();
    let iter = fs::read_dir(&data_dir).expect("failed to read data directory");

    let mut years: Vec<i32> = iter
        .map(|entry| {
            entry
                .expect("failed to get a directory entry")
                .file_name()
                .to_str()
                .expect("failed to convert OsStr to &str, invalid Unicode")
                .parse()
                .expect("failed to parse a year &str to i32")
        })
        .collect();

    years.sort();
    let (earliest_year, latest_year) = (*years.first().unwrap(), *years.last().unwrap());

    // get earliest and latest months
    let earliest_year_dir = data_dir.join(earliest_year.to_string());
    let latest_year_dir = data_dir.join(latest_year.to_string());

    let earliest_months = get_month_numbers(earliest_year_dir.clone());
    let earliest_month = *earliest_months.first().unwrap();

    let latest_months = if earliest_year_dir != latest_year_dir {
        get_month_numbers(latest_year_dir)
    } else {
        earliest_months
    };
    let latest_month = *latest_months.last().unwrap();

    // finally, construct earliest and latest dates for the calendar
    let earliest_date = Local.ymd(earliest_year, earliest_month.into(), 1);

    let latest_month_name = tMonth::try_from(latest_month).unwrap();
    let latest_day = days_in_year_month(latest_year, latest_month_name);
    let latest_date = Local.ymd(latest_year, latest_month.into(), latest_day.into());

    (earliest_date, latest_date)
}

/// Get a vector of sorted numbers (representing months in the specified path).
///
/// First read contents of path, remove the `.json` filename extension, convert
/// month to number and finally sort and return.
fn get_month_numbers(path: PathBuf) -> Vec<u8> {
    let iter = fs::read_dir(path).expect("failed to read directory");

    let mut months: Vec<u8> = iter
        .map(|entry| {
            let month = entry.expect("failed to get a directory entry").file_name();

            // month at this point also has the file extension ('.json')
            let month = &month.to_str().unwrap()[..(month.len() - 5)];
            month_number(month)
        })
        .collect();

    months.sort();
    months
}

/// Given a month name, return the month number.
fn month_number(month: &str) -> u8 {
    match month {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => panic!("unexpcted input: {}", month),
    }
}
