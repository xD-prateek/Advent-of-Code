use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let trench = Trench::new_from_str(content);

    let ans = trench.get_area();

    println!("ANS: {ans}");
}

struct Trench {
    map: Vec<(isize, isize)>,
    blocks: isize,
}

impl Trench {
    fn new_from_str(s: String) -> Self {
        let directions_map = vec!{ (0, 1), (1, 0), (0, -1), (-1, 0) };

        s.lines().fold(Self { map: vec!{ (0, 0) }, blocks: 0isize }, |mut acc, line| {
            let (length, direction) = line.split_whitespace().nth(2).unwrap()[2..8].split_at(5);
            let direction = directions_map.get(direction.parse::<usize>().unwrap_or_else(|_| panic!("error parsing length."))).unwrap_or_else(|| panic!("wrong direction input."));
            let length = isize::from_str_radix(length, 16).unwrap_or_else(|_| panic!("error converting hexadecimal to decimal."));
            acc.blocks += length;
            let last = acc.map.last().unwrap();
            acc.map.push((last.0 + direction.0 * length, last.1 + direction.1 * length));
            acc
        })
    }

    fn get_area(&self) -> usize {
        ((self.blocks + self.map.windows(2).fold(0isize, |acc, pos| {
            acc + pos[0].0 * pos[1].1 - pos[0].1 * pos[1].0
        }).abs()) / 2 + 1) as usize
    }
}
