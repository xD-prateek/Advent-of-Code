use std::fs::read_to_string;
use aoc::Stack;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let stack = Stack::new_from_str(&content);

    let ans = stack.get_bricks_safe_to_disintegrate();

    println!("ANS: {ans}");
}
