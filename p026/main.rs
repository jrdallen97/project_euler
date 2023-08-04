fn main() {
    // Woo, it works!
    let mut best = 0;
    let mut best_len = 0;

    for i in 2..=1000 {
        match get_recurring_part(i) {
            Some(recurring_part) => {
                let len = recurring_part.len();
                if len > best_len {
                    best_len = len;
                    best = i;
                }
            },
            None => {},
        }
    }
    println!("best: {best} (length {best_len})");
}

// If not recurring, returns None.
// If recurring, returns the recurring part as Some(string).
fn get_recurring_part(divisor: usize) -> Option<String> {
    let mut numerator = 1;
    let mut out = String::new();

    let mut remainders: Vec<usize> = vec![];

    loop {
        let remainder = numerator % divisor;
        numerator = numerator / divisor;

        out.push_str(&numerator.to_string());
        if out.len() == 1 {
            out.push_str(".");
        }

        if remainder == 0 {
            // Not recurring.
            // println!("{out}");
            return None;
        } else {
            // Store remainders.
            // It's recurring if we get a remainder that we've seen before.
            let recurring_from = remainders.iter().position(|r| r == &remainder);
            match recurring_from {
                Some(from) => {
                    let recurring_part = &out[from+2..(out.len())];
                    // println!("{}({})", &out[0..from+2], recurring_part);
                    return Some(String::from(recurring_part));
                }
                None => remainders.push(remainder),
            }
        }

        numerator = remainder * 10;
    }
}
