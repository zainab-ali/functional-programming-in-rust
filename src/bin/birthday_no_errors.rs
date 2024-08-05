use std::fmt::Write;

use chrono::{Datelike, NaiveDate, Utc};
use Message::*;

fn main() -> Result<(), Error> {
    let today = today();
    let date_string = std::env::args().nth(1);
    let message = calc_message(date_string, today)?;
    print_message(message);
    Ok(())
}
#[derive(Debug, PartialEq)]
enum Error {
    NoDateOfBirth,
    BadDateOfBirth(String),
    BornInTheFuture,
}

#[derive(Debug, PartialEq)]
enum Message {
    HappyBirthday { age: u32 },
    HadBirthday,
    Wait { days: u32 },
    NoBirthday,
}

fn print_message(message: Message) -> () {
    match message {
        HappyBirthday { age } => println!("Happy birthday! Congratulations on becoming {}!", age),
        HadBirthday => println!("You've already had your birthday. I hope you had fun!"),
        Wait { days } => println!("It's not your birthday yet. Wait for {} more days.", days),
        NoBirthday => println!("Sadly no birthday for you this year."),
    }
}

fn calc_message(date_string: Option<String>, today: NaiveDate) -> Result<Message, Error> {
    let date_of_birth = date_of_birth(date_string, today)?;
    let message = match birthday(today, date_of_birth) {
        None => NoBirthday,
        Some(birthday) => {
            if birthday == today {
                Message::HappyBirthday {
                    age: age(today, date_of_birth),
                }
            } else {
                if birthday < today {
                    Message::HadBirthday
                } else {
                    Message::Wait {
                        days: days_until_birthday(today, birthday) as u32,
                    }
                }
            }
        }
    };
    Ok(message)
}

fn days_until_birthday(today: NaiveDate, birthday: NaiveDate) -> i64 {
    birthday.signed_duration_since(today).num_days()
}
fn age(today: NaiveDate, date_of_birth: NaiveDate) -> u32 {
    // The dob is after today.
    today.years_since(date_of_birth).unwrap()
}
fn date_of_birth(date_string: Option<String>, today: NaiveDate) -> Result<NaiveDate, Error> {
    // There is no argument specified
    match date_string {
        None => Err(Error::NoDateOfBirth),
        Some(str) => NaiveDate::parse_from_str(&str, "%Y-%m-%d")
            .map_err(|_| Error::BadDateOfBirth(str))
            .and_then(|date| {
                if date <= today {
                    Ok(date)
                } else {
                    Err(Error::BornInTheFuture)
                }
            }),
    }
}
fn birthday(today: NaiveDate, date_of_birth: NaiveDate) -> Option<NaiveDate> {
    // This will fail if the user was born on a leap year
    date_of_birth.with_year(today.year())
}

fn today() -> NaiveDate {
    Utc::now().date_naive()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn born_on_a_leap_year() {
        let today = NaiveDate::from_ymd(2025, 01, 01);
        let result = calc_message(Some("2000-02-29".to_string()), today);
        assert_eq!(result, Ok(Message::HadBirthday));
    }
}
