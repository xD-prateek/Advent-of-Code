use std::fs::read_to_string;
use aoc::Aplenty;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let (workflows, _) = content.split_once("\n\n").unwrap_or_else(|| panic!("error reading input"));

    let rating_start = 1;
    let rating_end = 4000;
    let aplenty = Aplenty::new_from_values(workflows, rating_start, rating_end);

    let ans = aplenty.get_rating_of_accepted_parts();
    println!("ANS: {ans}");
}
