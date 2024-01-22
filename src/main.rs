use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let steps = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let ans = steps.split(',').fold(0usize, |acc, step| acc + encode(step));
    println!("ANS: {ans}");
}

fn encode(input: &str) -> usize {
    input.chars().fold(0usize, |current_value, c| {
        (current_value + c as u8 as usize) * 17 % 256
    })
}