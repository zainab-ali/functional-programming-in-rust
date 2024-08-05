use chrono::{Datelike, NaiveDate, Utc};
use Message::*;

fn main() -> () {
    let today = today();
    let date_of_birth = date_of_birth();
    let message = calc_message(date_of_birth, today);
    print_message(message);
}

#[derive(Debug, PartialEq)]
enum Message {
    HappyBirthday { age: u32 },
    HadBirthday,
    Wait { days: u32 },
}

fn print_message(message: Message) -> () {
    match message {
        HappyBirthday { age } => println!("Happy birthday! Congratulations on becoming {}!", age),
        HadBirthday => println!("You've already had your birthday. I hope you had fun!"),
        Wait { days } => println!("It's not your birthday yet. Wait for {} more days.", days),
    }
}

fn calc_message(date_of_birth: NaiveDate, today: NaiveDate) -> Message {
    let birthday = birthday(today, date_of_birth);
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

fn days_until_birthday(today: NaiveDate, birthday: NaiveDate) -> i64 {
    birthday.signed_duration_since(today).num_days()
}
fn age(today: NaiveDate, date_of_birth: NaiveDate) -> u32 {
    // The dob is after today.
    today.years_since(date_of_birth).unwrap()
}
fn date_of_birth() -> NaiveDate {
    // There is no argument specified
    let date_string = std::env::args().nth(1).unwrap();
    NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").unwrap()
}
fn birthday(today: NaiveDate, date_of_birth: NaiveDate) -> NaiveDate {
    // This will fail if the user was born on a leap year
    date_of_birth.with_year(today.year()).unwrap()
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
        let date_of_birth = NaiveDate::from_ymd(2000, 02, 29);
        let result = calc_message(date_of_birth, today);
        assert_eq!(result, Message::HadBirthday);
    }
}
