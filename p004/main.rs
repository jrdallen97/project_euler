fn is_palin(s: &str) -> bool {
    let r = s.chars().rev().collect::<String>();
    return s == r;
}

fn main() {
    let mut found: Vec<i32> = vec![];
    for x in 100..=999 {
        for y in 100..=999 {
            let n = x * y;
            let s = n.to_string();
            if is_palin(&s) {
                found.push(n);
            }
        }
    }
    let max = found.iter().max();
    match max {
        Some(max) => println!("Largest palindrome: {}", max),
        None => println!("No palindromes found"),
    }
}
