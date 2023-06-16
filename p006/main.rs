fn main() {
    println!("{}", square_of_sums(100) - sum_of_squares(100));
}

fn sum_of_squares(n: i64) -> i64 {
    // let mut sum = 1;
    // for x in 2..=n {
    //     sum += x.pow(2);
    // }
    // sum

    (1..=n).fold(0, |x, y| x + y.pow(2))
}

fn square_of_sums(n: i64) -> i64 {
    // let mut sum = 1;
    // for x in 2..=n {
    //     sum += x;
    // }
    // sum.pow(2)

    (1..=n).sum::<i64>().pow(2)
}
