use std::fs::read_to_string;
use aoc::SnowIsland;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let snow_island = SnowIsland::new_from_str(&content);

    let ans = snow_island.get_longest_hike();

    println!("ANS: {ans}");
}
