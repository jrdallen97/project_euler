fn main() {
    let mut n = 1;
    let mut sum = 1;
    let mut size = 3;
    while size <= 1001 {
        for _ in 0..4 {
            n += size - 1;
            sum += n;
        }
        size += 2;
    }
    println!("sum: {sum}");
}
