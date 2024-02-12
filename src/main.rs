use std::fs::read_to_string;
use aoc::Hail;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));
    let start = 200000000000000;
    let end = 400000000000000;

    let hail = Hail::new_from_str(&content, start, end);
    let ans = hail.determine_collisions();

    println!("ANS: {ans}");
}
