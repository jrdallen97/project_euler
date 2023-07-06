// Finds all the factors of num (including 1 and num).
pub fn factors(num: u64) -> Vec<u64> {
    // The highest unique factor is sqrt(num).
    // After that we'll just be finding the matching "pair" to all the factors we've already found.
    let sqrt = (num as f64).sqrt();
    let mut highest = sqrt as u64;
    let mut found: Vec<u64> = vec![];

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

// Optimised version of factors for when you only care whether something is prime or not.
pub fn is_prime(num: u64) -> bool {
    // Handle cases where this logic is wrong lol
    if num <= 3 {
        if num == 1 {
            return false;
        }
        return true;
    }

    // The highest unique factor is sqrt(num).
    // After that we'll just be finding the matching "pair" to all the factors we've already found.
    let highest = (num as f64).sqrt() as u64;

    // Starting from 2 bc we don't care about 1 and num
    // If we find any other factors it means it's not prime
    for i in 2..=highest {
        if num % i == 0 {
            return false
        }
    }

    true
}
