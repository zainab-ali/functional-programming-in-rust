use chrono::{Datelike, NaiveDate, Utc};

// We have separated the side effecting code from the pure code
// The value assigned to "today" does not change throughout the program.
//
// Pure functions are prefixed by "calc_" to make them easier to identify,
// only for the sake of this tutorial.
// These functions can be tested easily.
fn main() -> () {
    // Side effects
    let today = get_today();
    let input = get_arg();

    // Pure code
    let dob = calc_dob(input);
    let birthday = calc_birthday(today, dob);

    if birthday == today {
        println!(
            "Happy birthday! Congratulations on becoming {}!",
            calc_age(today, dob)
        );
    } else if birthday < today {
        println!("You've already had your birthday. I hope you had fun!")
    } else {
        println!(
            "It's not your birthday yet. Wait for {} more days.",
            calc_days_until_birthday(today, birthday)
        );
    }
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

fn calc_days_until_birthday(today: NaiveDate, birthday: NaiveDate) -> i64 {
    birthday.signed_duration_since(today).num_days()
}
fn calc_age(today: NaiveDate, dob: NaiveDate) -> u32 {
    today.years_since(dob).unwrap()
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
}
