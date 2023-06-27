// Copied from p007
fn is_prime(num: u64) -> bool {
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

fn main() {
    let mut n = 3;
    let mut sum = 2;
    while n < 2_000_000 {
        if is_prime(n) {
            sum += n;
        }
        n += 2;
    }
    println!("sum: {sum}");
}
