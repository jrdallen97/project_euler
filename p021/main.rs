// Reused again lol (from p003 & p012)
fn factors(num: i64) -> Vec<i64> {
    // The highest unique factor is sqrt(num).
    // After that we'll just be finding the matching "pair" to all the factors we've already found.
    let sqrt = (num as f64).sqrt();
    let mut highest = sqrt as i64;
    let mut found: Vec<i64> = vec![];

    // If the sqrt is a whole number, it will be a factor.
    // Handle it here to prevent double-counting it.
    if sqrt % 1.0 == 0.0 {
        found.push(highest);
        // We don't need to check this one again, so decrement highest.
        highest -= 1;
    }

    for i in 1..=highest {
        if num % i == 0 {
            found.push(i);
            found.push(num/i);
        }
    }

    found.sort();
    found
}

fn divisor_sum(num: i64) -> i64 {
    factors(num).into_iter().take_while(|x| x < &num).sum()
}

fn main() {
    let mut sum = 0;

    for i in 2..10000 {
        let r1 = divisor_sum(i);
        let r2 = divisor_sum(r1);
        if i == r2 && r1 != r2 {
            // Just add the first pair so we don't double-count the other one
            sum += i;
        }
    }

    println!("sum: {sum}");
}
