use utils::is_prime;

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
