use utils::factors;

// Takes a number and all the abundant numbers and returns whether the number is the sum of two
// abundant numbers.
fn is_sum_of_two_abundants(i: u64, abundant_numbers: &Vec<u64>) -> bool {
    for d in abundant_numbers.iter().take_while(|x| x < &&i) {
        if abundant_numbers.contains(&(i - d)) {
            return true;
        }
    }
    return false;
}

// This approach is pretty slow since there are a lot of abundant numbers to check when we get to
// the higher numbers...
fn main() {
    // First calculate all the abundant numbers in the range.
    let mut abundant_numbers: Vec<u64> = vec![];
    for i in 2..=28123 {
        // Work out if the sum of proper divisors > i.
        if factors(i).into_iter().take_while(|x| x < &i).sum::<u64>() > i {
            abundant_numbers.push(i);
        }
    }

    let mut sum = 0;
    for i in 1..=28123 {
        if !is_sum_of_two_abundants(i, &abundant_numbers) {
            // println!("{i}");
            sum += i;
        }
    }
    println!("{sum}");
}
