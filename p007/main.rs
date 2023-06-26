// More-or-less copied from p003
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
    let mut count = 2;
    let mut current = 3;
    loop {
        current += 2;
        if is_prime(current) {
            count += 1;
            if count == 10_001 {
                break;
            }
        }
    }

    println!("prime #{count} is {current}");
}
