use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let steps = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));
    let number_of_boxes = 256;

    let boxes = steps.split(',').fold(HashMap::with_capacity(number_of_boxes), |mut boxes: HashMap<u8, Vec<Lens>>, step| {
        let (label, action) = step.split_once(|c: char| !c.is_alphabetic()).unwrap();
        let box_no = encode(label);
        match action.parse::<u8>() {
            Ok(focal_length) => {
                boxes.entry(box_no).and_modify(|b| match b.iter_mut().find(|lens| lens.label == label) {
                        Some(lens) => lens.focal_length = focal_length,
                        None => b.push(Lens::new_from_values(label, focal_length)),
                    }).or_insert(vec!{ Lens::new_from_values(label, focal_length) });
            },
            Err(_) => if let Some(b) = boxes.get_mut(&box_no) {
                b.retain(|lens| lens.label != label);
                if b.is_empty() {
                    boxes.remove(&box_no);
                }
            },
        }
        boxes
    });

    let ans = calculate_focal_length(boxes);
    println!("ANS: {ans}");
}

struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

impl<'a> Lens<'a> {
    fn new_from_values(label: &'a str, focal_length: u8) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

fn encode(input: &str) -> u8 {
    input.chars().fold(0usize, |current_value, c| (current_value + c as u8 as usize) * 17 % 256) as u8
}

fn calculate_focal_length(boxes: HashMap<u8, Vec<Lens>>) -> usize {
    boxes.into_iter().fold(0usize, |acc, (k, b)| acc + b.into_iter().enumerate().fold(0usize, |sum_of_lenses, (pos, lens)| sum_of_lenses + (k as usize + 1) * (pos + 1) * lens.focal_length as usize))
}