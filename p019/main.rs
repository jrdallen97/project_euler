// cargo-deps: chrono = "0.4.26"

use chrono::{NaiveDate, Datelike};

fn main() {
    println!("{}", _with_chrono());
}

// Using a date library makes this pretty easy.
// This could be reduced to a one-liner, but it's pretty ugly.
fn _with_chrono() -> i32 {
    // Start from the first Sunday in the time range
    let weeks = NaiveDate::from_ymd_opt(1900, 1, 7).unwrap().iter_weeks();

    let mut count = 0;
    for date in weeks.take_while(|d| d.year() < 2001) {
        // Wait til we get to 1901 to start counting
        if date.year() >= 1901 && date.day() == 1 {
            count += 1;
        }
    }
    count
}
