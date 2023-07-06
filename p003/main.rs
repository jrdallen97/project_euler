use utils::factors;

fn main() {
    // Get the factors
    let mut vals = factors(600851475143);
    // Filter out the non-prime factors
    vals.retain(|val| factors(*val).len() == 2);
    // Find the max
    let min = vals.iter().max();
    match min {
        Some(min) => println!("{}", min),
        None => println!("No factors?"),
    }
}
