use utils::is_prime;

fn main() {
    let mut best = (0, 0, 0);
    for a in -999..=999 {
        for b in -1000..=1000 {
            let count = conseq_primes(a, b);
            if count > best.2 {
                best = (a, b, count);
            }
        }
    }
    println!("best is a: {}, b: {} ({} primes)", best.0, best.1, best.2);
    println!("product: {}", best.0 * best.1);
}

fn conseq_primes(a: isize, b: isize) -> isize {
    let mut n: isize = 0;
    loop {
        let val = n.pow(2) + (a * n) + b;
        if val > 0 && is_prime(val as u64) {
            n += 1;
        } else {
            return n
        }
    }
}
