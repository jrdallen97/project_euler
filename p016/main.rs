// cargo-deps: num = "0.4.0"

use num::bigint::ToBigUint;

fn main() {
    // This is only easy because I'm using a bigint, otherwise I'd have to work out how to do this
    // with just primitives and I'm not sure how I'd do that...
    let two = 2.to_biguint().unwrap();

    let total: u32 = two.pow(1000).to_string().chars().map(|x| x.to_digit(10).unwrap()).sum();
    print!("{total}");
}
