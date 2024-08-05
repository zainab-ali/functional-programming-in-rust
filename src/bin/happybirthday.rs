use chrono::{NaiveDate, Utc, Datelike};


fn main() -> std::io::Result<()> {
    let date_of_birth_str = std::env::args().nth(1).unwrap();
    let date_of_birth = NaiveDate::parse_from_str(&date_of_birth_str, "%Y-%m-%d").unwrap();
    let now = Utc::now().date_naive();
    if let Some(age) = is_birthday(date_of_birth, now) {
	println!("Congratulations on reaching the ripe old age of {}!", age);
    }
    Ok(())
}

fn is_birthday(date_of_birth: NaiveDate, now: NaiveDate) -> Option<i32> {
    if date_of_birth.month() == now.month() && date_of_birth.day() == now.day() {
	Some(now.year() - date_of_birth.year())
    } else {
	None
    }
}
