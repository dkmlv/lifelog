use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use chrono::prelude::*;
use cursive::reexports::time::{util::days_in_year_month, Month as tMonth};
use etcetera::base_strategy::{choose_base_strategy, BaseStrategy};
use serde::{Deserialize, Serialize};

/// An object containing diary entries for a given month.
#[derive(Serialize, Deserialize)]
pub struct MonthLog {
    /// The name of the month in capitalized case (eg `November`)
    month: String,
    /// Full gregorian year (eg `2022`)
    year: u32,
    /// Vector of daily entries that have the day rating and text.
    /// The number of entries is dependent on the number of days in the given
    /// month (if a month has 30 days, the vector will have 30 entries).
    entries: Vec<Entry>,
}

impl MonthLog {
    /// Construct a new `MonthLog` for a given month and year.
    ///
    /// The object is filled up with default entries. The number of entries
    /// depends on the number of days in the given month.
    fn new(month: &str, year: &str) -> Self {
        let month_days = days_in_year_month(
            year.parse::<i32>().unwrap(),
            month.parse::<tMonth>().unwrap(),
        );

        let mut entries: Vec<Entry> = Vec::new();
        for _ in 0..month_days {
            entries.push(Entry::default())
        }

        MonthLog {
            month: month.to_string(),
            year: year.parse().unwrap(),
            entries: entries,
        }
    }

    /// Construct a `MonthLog` from JSON file.
    fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut data = String::new();

        File::open(path)
            .expect("failed to open data file")
            .read_to_string(&mut data)
            .expect("failed to read data file");

        serde_json::from_str(&data).expect("failed to parse json file")
    }

    /// Return the `MonthLog` for the current month.
    ///
    /// If the JSON file from which the object can be constructed does not exist,
    /// construct a brand new object.
    pub fn current_month_log() -> Self {
        let month_year = Local::today().format("%B/%Y").to_string();
        let [month, year]: [&str; 2] = month_year
            .split("/")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();

        let path = data_dir().join(year);
        if !path.exists() {
            fs::create_dir(&path).expect("failed to create directory in data dir");
        }

        let data_file = path.join(format!("{}.json", month));

        if data_file.exists() {
            Self::from_file(data_file)
        } else {
            Self::new(month, year)
        }
    }

    /// Return the user entry for the given day.
    pub fn get_entry(&self, day: usize) -> &Entry {
        &self.entries[day - 1]
    }

    /// Return the user entry for today.
    pub fn get_todays_entry(&self) -> &Entry {
        let day = Local::today().format("%d").to_string();
        let day: usize = day.parse().unwrap();
        self.get_entry(day)
    }

    /// Retrun the path at which this `MonthLog` should be saved.
    fn path(&self) -> PathBuf {
        data_dir()
            .join(self.year.to_string())
            .join(format!("{}.json", &self.month))
    }

    /// Create and save JSON file to disk by serializing data with `serde`.
    pub fn save_to_disk(&self) {
        let data = serde_json::to_string(self).unwrap();
        fs::write(self.path(), data).unwrap();
    }
}

/// An entry for a given day with the rating for the day and some user text.
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Entry {
    /// Rating for a given day.
    ///
    /// User will be able to choose on a scale of `-2` to `2`.
    /// * `-2` - horrible
    /// * `-1` - bad
    /// * `0` - okay
    /// * `1` - good
    /// * `2` - awesome
    ///
    /// * `42` - default value for an empty entry.
    ///
    /// Inspired by [a blog post](https://ihatereality.space/03-a-place-to-pause/),
    /// which itself was inspired by
    /// [another blog post](https://optozorax.github.io/p/5-point-ratings-are-wrong/).
    rating: i8,
    /// Text for the diary entry.
    /// If the entry is empty, field will be "wow, such empty"
    text: String,
}

impl Default for Entry {
    /// Defaults are used to populate the vector for a new `MonthLog`
    fn default() -> Self {
        Entry {
            rating: 42,
            text: "wow, such empty".to_string(),
        }
    }
}

impl Entry {
    /// Check if the entry is empty.
    pub fn is_default(&self) -> bool {
        *self == Entry::default()
    }
}

/// Return the location of the data directory.
///
/// Choice of the data directory location is dependent on the underlying os.
///
/// All of the user's diary entries will be saved in `.json` files.
/// The folder structure will be sth like this:
///
/// ```text
/// lifelog
/// ├── 2022
/// │   ├── January.json
/// │   ├── February.json
/// │   └── ...
/// └── 2023
///     ├── January.json
///     ├── February.json
///     └── ...
/// ```
fn data_dir() -> PathBuf {
    let strategy = choose_base_strategy().expect("failed to find config directory");
    let mut path = strategy.data_dir();
    path.push("lifelog");
    path
}
