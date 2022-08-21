use std::fs;
use std::path::PathBuf;

use chrono::{Date, Datelike, Local, TimeZone};
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
/// This function should return 2022-01-01[offset] as the earliest date.
/// It will return the last day of the current month as the latest date.
/// This behavior is desired since we want the calendar view to be focused on
/// the current day regardless of whether there is a `.json` file present.
///
/// The function is going to be used to set earliest and latest dates for the
/// calendar view.
pub fn earliest_latest() -> (Date<Local>, Date<Local>) {
    // construct latest date
    let today = Local::today();
    let (current_month, current_year) = (today.month() as u8, today.year());
    let latest_day = days_in_year_month(current_year, tMonth::try_from(current_month).unwrap());
    let latest_date = Local.ymd(current_year, current_month.into(), latest_day.into());

    // construct earliest date
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

    let earliest_date = match years.first() {
        Some(earliest_year) => {
            let earliest_month = *get_month_numbers(data_dir.join(earliest_year.to_string()))
                .first()
                .expect("earliest year directory is empty");
            Local.ymd(*earliest_year, earliest_month.into(), 1)
        }
        None => Local.ymd(current_year, current_month.into(), 1),
    };

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
