use utils::is_prime;

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
