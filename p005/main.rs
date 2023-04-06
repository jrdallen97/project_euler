// cargo-deps: num = "0.4.0"

use num::integer::lcm;

fn _naiive(max_divisor: i64) -> i64 {
    let mut n = 1;
    'outer: loop {
        for i in 2..=max_divisor {
            if n % i != 0 {
                n += 1;
                continue 'outer;
            }
        }
        return n;
    }
}

// Add a few simple optimisations that make it much faster (5s -> 0.15s)
fn _smarter(max_divisor: i64) -> i64 {
    // Start at max_divisor as anything lower will never work
    let mut n = max_divisor;

    'outer: loop {
        // Reverse iterator so it fails faster
        // Don't check max_divisor as that'll always be true
        for i in (2..max_divisor).rev() {
            if n % i != 0 {
                // Inc by max_divisor so we can check less things
                n += max_divisor;
                continue 'outer;
            }
        }
        return n;
    }
}

// I accidentally realised this solves it lol
// Just work out the lowest common multiple of all the numbers
fn _cheating(max_divisor: i64) -> i64 {
    (2..=max_divisor).fold(1, |x, y| lcm(x, y))
}

fn main() {
    // println!("{}", _naiive(20));
    println!("{}", _smarter(20));
    // println!("{}", _cheating(20));
}
