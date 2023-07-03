use std::fs;

// Note: I also tested this against the 100-line triangle in p067 and it still only takes ~0.15s
fn main() {
    let f = fs::read_to_string("p018/triangle.txt").expect("file not readable");
    let mut tri: Vec<Vec<u32>> = f.trim().split("\n")
        .map(|x| x.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect();

    // Start at top, work out best score for each node before continuing to next level?
    // Then we're not working out quite as many routes (cutting off the dead-end routes earlier)
    // We don't need to know the final route, just the final sum
    for x in 1..tri.len() {
        for y in 0..tri[x].len() {
            let mut best = 0;
            // Check the parent up and to the left
            if y > 0 {
                best = tri[x-1][y-1]
            }
            // Check the parent up and to the right
            if y < tri[x-1].len() && tri[x-1][y] > best {
                best = tri[x-1][y]
            }
            // Bump the val by whatever the highest parent was
            tri[x][y] += best;
        }
    }

    let max = tri.last().unwrap().iter().max().unwrap();
    println!("max: {max}");
}
