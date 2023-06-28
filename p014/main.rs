fn main() {
    let mut best = 1;
    let mut best_length = 1;

    // I tried using a hashmap to cache the count from each number previously seen, but it ended up
    // being slower
    for x in 1..1_000_000 {
        let count = count_chain(x);
        if count > best_length {
            best_length = count;
            best = x;
        }
    }

    println!("best: {best} (length {best_length})");
}

fn next(i: usize) -> usize {
    if i % 2 == 0 {
        i/2
    } else {
        3*i + 1
    }
}

fn count_chain(current: usize) -> usize {
    if current == 1 {
        return 1
    }
    count_chain(next(current)) + 1
}
