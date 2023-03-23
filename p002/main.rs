fn main() {
    let mut sum = 0;
    let mut prev = 1;
    let mut curr = 2;

    while curr <= 4_000_000 {
        if curr % 2 == 0 {
            sum += curr
        }

        (prev, curr) = (curr, prev + curr);
    }
    println!("{}", sum);
}
