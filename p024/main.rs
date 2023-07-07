fn factorial(n: usize) -> usize {
    (1..=n).product()
}

// We know that the number of permutations for a string of length n is `n!`, so the number of
// permutations assuming a specific value of the first character in n is `(n-1)!`.
// We can use this to recursively work out what character lies at each position of the target
// permutation.
fn nth_permutation(target: usize, mut digits: String) -> String {
    // When we're only looking for the 1st permutation, just return the rest of the digits as-is.
    if target == 1 {
        return digits;
    }

    // As we count up the digits, we can essentially increment the permutation counter by `(n-1)!`.
    let step = factorial(digits.len() - 1);

    let mut current = 0;
    for idx in 0..digits.len() {
        // Continue skipping permutations until the next digit change would take us over the target.
        let next = current + step;
        if next < target {
            current = next;
            continue;
        }

        // Pull out the digit at idx and pass the rest on to work out the next digit.
        let val = digits.remove(idx).to_string();
        return val + &nth_permutation(target - current, digits);
    }

    panic!("target higher than number of possible permutations");
}

fn main() {
    println!("{}", nth_permutation(1_000_000, String::from("0123456789")));
}
