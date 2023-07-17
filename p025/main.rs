// cargo-deps: num = "0.4.0"

use num::bigint::ToBigUint;

fn main() {
    let mut x = 1.to_biguint().unwrap();
    let mut y = 1.to_biguint().unwrap();
    let mut i = 2;
    while y.to_string().len() < 1000 {
        let temp = y.clone();
        y += x;
        x = temp;
        i += 1;
    }
    println!("{i}");
}
