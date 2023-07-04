// cargo-deps: num = "0.4.0"

use num::bigint::ToBigUint;

// I feel like using bigint isn't really in the spirit of the challenge, but it's just so easy.
fn main() {
    let mut val = 1.to_biguint().unwrap();
    for i in 2..=100 {
        let i = i.to_biguint().unwrap();
        val = val * i;
    }
    println!("{}", val.to_str_radix(10).chars().fold(0, |x, y| x + y.to_digit(10).unwrap()));
}
