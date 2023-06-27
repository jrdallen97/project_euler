fn main() {
    for a in 1..1000 {
        for b in (a+1)..=1000 {
            match get_c(a, b) {
                Some(c) => {
                    let sum = a + b + c;
                    if sum == 1000 {
                        println!("a: {a}, b: {b}, c: {c}, sum: {sum}");
                        let p = a * b * c;
                        println!("product abc: {p}");
                    }
                },
                None => (),
            }
        }
    }

}

fn get_c(a: u32, b: u32) -> Option<u32> {
    let c2 = (a.pow(2) + b.pow(2)) as f32;
    let c = c2.sqrt();
    if c % 1.0 == 0.0 {
        return Some(c as u32);
    }
    None
}
