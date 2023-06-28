// Copied from p003
// Turns out that version had bugs, I've fixed them here
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

fn main() {
    let mut i = 0;
    let mut tri = 0;

    loop {
        i += 1;
        tri += i;
        let f = factors(tri);

        println!("{tri}: {f:?}");

        if f.len() > 10 {
            println!("{tri} has {} divisors", f.len());
            break;
        }
    }
}
