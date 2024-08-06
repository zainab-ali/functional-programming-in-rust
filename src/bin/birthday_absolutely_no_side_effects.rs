use chrono::{Datelike, NaiveDate, Utc};
use Message::*;

fn main() -> () {
    let today = get_today(); // Side effects
    let dob_string = get_arg();

    let message = calc_message(dob_string, today);

    print_message(message); // Side effects
}

fn get_today() -> NaiveDate {
    Utc::now().date_naive()
}

fn get_arg() -> Option<String> {
    std::env::args().nth(1)
}
fn calc_dob(date_arg: Option<String>) -> NaiveDate {
    NaiveDate::parse_from_str(&date_arg.unwrap(), "%Y-%m-%d").unwrap()
}
fn calc_birthday(today: NaiveDate, dob: NaiveDate) -> NaiveDate {
    dob.with_year(today.year()).unwrap()
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

fn calc_message(dob_string: Option<String>, today: NaiveDate) -> Message {
    let dob = calc_dob(dob_string);
    let birthday = calc_birthday(today, dob);
    if birthday == today {
        Message::HappyBirthday {
            age: calc_age(today, dob),
        }
    } else if birthday < today {
        Message::HadBirthday
    } else {
        Message::Wait {
            days: calc_days_until_birthday(today, birthday) as u32,
        }
    }
}

fn calc_days_until_birthday(today: NaiveDate, birthday: NaiveDate) -> i64 {
    birthday.signed_duration_since(today).num_days()
}
fn calc_age(today: NaiveDate, date_of_birth: NaiveDate) -> u32 {
    // The dob is after today.
    today.years_since(date_of_birth).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn born_on_a_leap_year() {
    //     let today = NaiveDate::from_ymd(2025, 01, 01);
    //     let date_of_birth = NaiveDate::from_ymd(2000, 02, 29);
    //     let result = calc_message(date_of_birth, today);
    //     assert_eq!(result, Message::HadBirthday);
    // }
}
