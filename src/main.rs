use std::{collections::BTreeMap, fs::read_to_string};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let factory = Factory::new_from_string(content, 3);

    let ans = factory.find_min_heat_loss();
    println!("ANS: {ans}");

}

struct Factory {
    map: Vec<Vec<u32>>,
    pq: BTreeMap<u32, Node>,
    consecutive_step_count: u32,
}

impl Factory {
    fn new_from_string(content: String, consecutive_step_count: u32) -> Self {
        let initial_node = Node {
            x: 0,
            y: 0,
            del_x: 0,
            del_y: 0,
            consecutive_steps: 0,
        };

        Self {
            map: content.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or_else(|| panic!("Error parsing input"))).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>(),
            pq: BTreeMap::from([(0, initial_node)]),
            consecutive_step_count,
        }
    }

    fn find_min_heat_loss(&self) -> u32 {
       todo!(); 
    }
}

struct Node {
    x: usize,
    y: usize,
    del_x: usize,
    del_y: usize,
    consecutive_steps: u32,
}