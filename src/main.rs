use std::fs::read_to_string;
use aoc::Garden;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));
    let steps = 26501365;

    let garden = Garden::new_from_string(&content);

    let ans = garden.get_potential_garden_plots(steps);

    println!("ANS: {ans}");
}
