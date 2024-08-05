use chrono::{Datelike, NaiveDate, Utc};
use Message::*;

// We now no longer panic on invalid input. Instead, we exit with a more informative code.
fn main() -> Result<(), Error> {
    // Side effects
    let today = get_today();
    let input = get_arg();

    // Pure code
    let message = calc_message(input, today)?;

    // Side effects
    print_message(message);
    Ok(())
}

fn get_today() -> NaiveDate {
    Utc::now().date_naive()
}

fn get_arg() -> Option<String> {
    std::env::args().nth(1)
}

fn calc_message(input: Option<String>, today: NaiveDate) -> Result<Message, Error> {
    let dob = calc_dob(input)?;
    let birthday = calc_birthday(today, dob)?;
    let message = if birthday == today {
        HappyBirthday {
            age: calc_age(today, dob),
        }
    } else if birthday < today {
        HadBirthday
    } else {
        Wait {
            days: calc_days_until_birthday(today, birthday) as u32,
        }
    };
    Ok(message)
}

fn calc_dob(input: Option<String>) -> Result<NaiveDate, Error> {
    let date_string = input.ok_or(Error::NoDateOfBirth)?;
    NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").map_err(|_| Error::BadDateOfBirth(date_string))
}

fn calc_birthday(today: NaiveDate, dob: NaiveDate) -> Result<NaiveDate, Error> {
    if dob <= today {
        // For simplicity, we don't handle leap years.
        Ok(dob.with_year(today.year()).unwrap())
    } else {
        Err(Error::BornInTheFuture)
    }
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

#[derive(Debug, PartialEq)]
enum Error {
    NoDateOfBirth,
    BadDateOfBirth(String),
    BornInTheFuture,
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
        assert_eq!(result, Ok(HadBirthday));
    }

    #[test]
    fn test_born_in_future() {
        let today = NaiveDate::from_ymd(2024, 08, 06);
        let result = calc_message(Some("2030-08-05".to_string()), today);
        assert_eq!(result, Err(Error::BornInTheFuture));
    }
}
