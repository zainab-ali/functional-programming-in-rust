use chrono::{Datelike, NaiveDate, Utc};
use Message::*;

// The printing code is also side effecting.
// We have separated it out by creating a Message enum.
// The "calc_message" function can now be easily tested.

fn main() -> () {
    // Side effects
    let today = get_today();
    let input = get_arg();

    // Pure code
    let message = calc_message(input, today);

    // Side effects
    print_message(message);
}

fn get_today() -> NaiveDate {
    Utc::now().date_naive()
}

fn get_arg() -> Option<String> {
    std::env::args().nth(1)
}

fn calc_message(input: Option<String>, today: NaiveDate) -> Message {
    let dob = calc_dob(input);
    let birthday = calc_birthday(today, dob);
    if birthday == today {
        HappyBirthday {
            age: calc_age(today, dob),
        }
    } else if birthday < today {
        HadBirthday
    } else {
        Wait {
            days: calc_days_until_birthday(today, birthday) as u32,
        }
    }
}

fn calc_dob(input: Option<String>) -> NaiveDate {
    NaiveDate::parse_from_str(&input.unwrap(), "%Y-%m-%d").unwrap()
}
fn calc_birthday(today: NaiveDate, dob: NaiveDate) -> NaiveDate {
    dob.with_year(today.year()).unwrap()
}

fn calc_days_until_birthday(today: NaiveDate, birthday: NaiveDate) -> i64 {
    birthday.signed_duration_since(today).num_days()
}
fn calc_age(today: NaiveDate, dob: NaiveDate) -> u32 {
    today.years_since(dob).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_days_until_birthday() {
        let today = NaiveDate::from_ymd(2024, 08, 05);
        let birthday = NaiveDate::from_ymd(2024, 08, 06);
        let result = calc_days_until_birthday(today, birthday);
        assert_eq!(result, 1);
    }
    
    #[test]
    fn test_had_birthday() {
        let today = NaiveDate::from_ymd(2024, 08, 06);
        let result = calc_message(Some("2000-08-05".to_string()), today);
        assert_eq!(result, HadBirthday);
    }
}
