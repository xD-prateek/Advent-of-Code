use std::fs::read_to_string;

use aoc::Graph;

fn main() {
    let file_name = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let graph = Graph::new_from_str(&content);

    let ans = graph.get_groups();

    println!("ANS: {ans}");
}