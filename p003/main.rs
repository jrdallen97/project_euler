fn factors(num: i64) -> Vec<i64> {
    // The highest unique factor is sqrt(num).
    // After that we'll just be finding the matching "pair" to all the factors we've already found.
    let highest = (num as f64).sqrt() as i64;
    let mut found: Vec<i64> = vec![];

    // Handle the sqrt case to avoid double-counting
    if num % highest == 0 {
        found.push(highest)
    }

    for i in 1..highest {
        if num % i == 0 {
            found.push(i);
            found.push(num/i);
        }
    }

    found
}

fn main() {
    // Get the factors
    let mut vals = factors(600851475143);
    // Filter out the non-prime factors
    vals.retain(|val| factors(*val).len() == 2);
    // Find the max
    let min = vals.iter().max();
    match min {
        Some(min) => println!("{}", min),
        None => println!("No factors?"),
    }
}
