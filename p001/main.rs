fn main() {
    let mut sum = 0;
    for n in 1..999 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    println!("{}", sum);
}
