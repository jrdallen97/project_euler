use std::fs;

// Copied from p018 verbatim with comments removed for brevity.
fn main() {
    let f = fs::read_to_string("p067/triangle.txt").expect("file not readable");
    let mut tri: Vec<Vec<u32>> = f.trim().split("\n")
        .map(|x| x.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect();

    for x in 1..tri.len() {
        for y in 0..tri[x].len() {
            let mut best = 0;
            if y > 0 {
                best = tri[x-1][y-1]
            }
            if y < tri[x-1].len() && tri[x-1][y] > best {
                best = tri[x-1][y]
            }
            tri[x][y] += best;
        }
    }

    let max = tri.last().unwrap().iter().max().unwrap();
    println!("max: {max}");
}
