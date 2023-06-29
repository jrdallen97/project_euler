fn main() {
    // Flip it so we're always targetting (0, 0), then we don't need to know the goal.

    // This is way too slow! It works, but takes ~10 mins.
    // let start = (20, 20);
    // let routes = traverse(start, 0);
    // println!("routes: {routes}");

    // It's fast enough for this though, which gives us an idea of how the dimensions affect the
    // end result.
    // From experimentation we can see that the value at (m, n) is equal to (m, n-1) + (m-1, n).
    // By using this instead, we could recursively work back to a known value and do a lot fewer
    // total calculations. We could also cache the repeated calculations so we don't have to
    // brute-force each pair twice.
    // for x in 1..=5 {
    //     for y in 1..=5 {
    //         let routes = _traverse((x, y), 0);
    //         print!("{routes:0>3} ");
    //     }
    //     println!();
    // }
    // Outputs:
    // 002 003 004 005 006
    // 003 006 010 015 021
    // 004 010 020 035 056
    // 005 015 035 070 126
    // 006 021 056 126 252

    let routes = fancy_maths(20);
    println!("routes: {routes}");
}

// Brute force, recursively finding all possible routes.
// Fine for sizes up to ~15x15, but gets exponentially slower.
// It also doesn't work for straight lines...
fn _traverse(current: (u8, u8), mut routes: usize) -> usize {
    if current == (0, 0) {
        return routes + 1;
    }

    if current.0 != 0 {
        routes = _traverse((current.0 - 1, current.1), routes);
    }
    if current.1 != 0 {
        routes = _traverse((current.0, current.1 - 1), routes);
    }
    routes
}

// Apparently this uses a branch of maths known as "combinatorics".
// I got algorithm from the problem overview after finding the answer with my brute-force approach.
// I think it's similar to a binomial, except in this case we can simplify since we're working on a
// square.
fn fancy_maths(size: usize) -> usize {
    let mut result = 1;
    for i in 1..=size {
        result = result * (size+i)/i;
    }
    result
}

