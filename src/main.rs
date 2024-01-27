mod contraption;

use std::fs::read_to_string;

use contraption::Contraption;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let contraption = Contraption::new_from_string(content);
    let ans = contraption.get_max_energized_tiles();

    println!("ANS: {ans}");
}
