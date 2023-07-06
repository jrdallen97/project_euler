use utils::factors;

fn main() {
    let mut i = 0;
    let mut tri = 0;

    loop {
        i += 1;
        tri += i;
        let f = factors(tri);

        // println!("{tri}: {f:?}");

        if f.len() > 500 {
            println!("{tri} has {} divisors", f.len());
            break;
        }
    }
}
