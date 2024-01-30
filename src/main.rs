use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let trench = Trench::new_from_str(content);

    let ans = trench.get_area();

    println!("ANS: {ans}");
}

#[derive(Debug)]
struct Trench {
    map: Vec<(i32, i32)>,
    blocks: i32,
}

impl Trench {
    fn new_from_str(s: String) -> Self {
        let direction_map = vec!{ ('U', (-1, 0)), ('D', (1, 0)), ('R', (0, 1)), ('L', (0, -1)) }.into_iter().collect::<HashMap<char, (i32, i32)>>();

        s.lines().fold(Self { map: vec!{ (0, 0) }, blocks: 0i32 }, |mut acc, line| {
            let mut split = line.split_whitespace().take(2).collect::<Vec<&str>>().into_iter();
            let direction = direction_map.get(&split.next().unwrap_or_else(|| panic!("error reading direction.")).chars().next().unwrap()).unwrap();
            let length = split.next().unwrap_or_else(|| panic!("error reading length.")).parse::<i32>().unwrap_or_else(|_| panic!("error parsing length"));
            acc.blocks += length;
            let last = acc.map.last().unwrap();
            acc.map.push((last.0 + direction.0 * length, last.1 + direction.1 * length));
            acc
        })
    }

    fn get_area(&self) -> u32 {
        ((self.blocks + self.map.windows(2).fold(0i32, |acc, pos| {
            acc + pos[0].0 * pos[1].1 - pos[0].1 * pos[1].0
        }).abs()) / 2 + 1) as u32
    }
}
