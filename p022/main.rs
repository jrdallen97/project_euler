use std::fs;

fn main() {
    let f = fs::read_to_string("p022/names.txt").expect("file not readable");
    let mut names: Vec<String> = f.split(",").map(|n| n.replace("\"", "")).collect();
    names.sort();

    let mut sum = 0;
    for (i, name) in names.iter().enumerate() {
        sum += score(i, name);
    }
    println!("sum: {sum}");
}

fn score(pos: usize, name: &String) -> usize {
    let name_score: usize = name.to_string().into_bytes().into_iter().map(|c| c as usize - 64).sum();

    (pos + 1) * name_score
}
