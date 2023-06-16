// More-or-less copied from p003
fn factors(num: u64) -> Vec<u64> {
    // Handle cases where this logic is wrong lol
    if num <= 3 {
        if num == 1 {
            return vec![1];
        }
        return vec![1, num];
    }

    // The highest unique factor is sqrt(num).
    // After that we'll just be finding the matching "pair" to all the factors we've already found.
    let highest = (num as f64).sqrt() as u64;
    let mut found: Vec<u64> = vec![];

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
    let mut count = 2;
    let mut current = 3;
    loop {
        current += 2;
        if factors(current).len() == 2 {
            count += 1;
            println!("{} is prime", current);
            if count == 10_001 {
                break;
            }
        }
    }
}
