mod factory;

use factory::Factory;
use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let factory = Factory::new_from_string(content, 3);

    let ans = factory.find_min_heat_loss();
    println!("ANS: {ans}");

}
