use chrono::{Datelike, NaiveDate, Utc};

fn main() -> () {
    let today = get_today(); // Side effects
    let dob_string = get_arg();

    let dob = calc_dob(dob_string); // Pure
    let birthday = calc_birthday(today, dob);

    if birthday == today {
        // Printing
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
    fn test_days_until_bday() {
        let today = NaiveDate::from_ymd(2024, 08, 05);
        let birthday = NaiveDate::from_ymd(2024, 08, 26);
        let result = calc_days_until_birthday(today, today);
        assert_eq!(result, 1);
    }
}
