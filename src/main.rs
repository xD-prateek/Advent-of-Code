use std::fs::read_to_string;
use aoc::Map;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));
    let final_machine = "rx";

    let map = Map::new_from_str(&content);

    let ans = map.get_cycles_till_final_machine(final_machine);

    println!("ANS: {ans}");

}
