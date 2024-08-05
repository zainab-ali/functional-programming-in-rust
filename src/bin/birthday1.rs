use chrono::{Datelike, NaiveDate, Utc};

// This code example has a bug.
// When called at midnight on the 5th August, we see:
// ./birthday 2000-08-06
// It's not your birthday yet. Wait for 0 more days.

fn main() -> () {
    if birthday() == today() {
        println!("Happy birthday! Congratulations on becoming {}!", age());
    } else if birthday() < today() {
        println!("You've already had your birthday. I hope you had fun!")
    } else {
        println!(
            "It's not your birthday yet. Wait for {} more days.",
            days_until_birthday()
        );
    }
}

fn today() -> NaiveDate {
    Utc::now().date_naive()
}
fn birthday() -> NaiveDate {
    date_of_birth().with_year(today().year()).unwrap()
}
fn date_of_birth() -> NaiveDate {
    let date_string = std::env::args().nth(1).unwrap();
    NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").unwrap()
}

fn days_until_birthday() -> i64 {
    birthday().signed_duration_since(today()).num_days()
}
fn age() -> u32 {
    today().years_since(date_of_birth()).unwrap()
}
