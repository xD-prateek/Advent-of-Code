use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let cycles = 1_000_000_000usize;
    let mut content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input.")).lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    apply_cycles(&mut content, cycles);

    let ans = calculate_load(content);
    println!("ANS: {ans}");
}

fn apply_cycles(content: &mut Vec<Vec<char>>, cycle_count: usize) {
    let mut state_cache = Vec::new();
    let mut matched = 0usize;
    loop {
        apply_single_cycle(content);
        matched += 1;
        match state_cache.contains(content) {
            true => break,
            false => state_cache.push(content.clone()),
        }
    }
    let buffer = state_cache.iter().position(|state| state == content).unwrap() + 1;
    *content = state_cache.get((cycle_count - buffer) % (matched - buffer) + buffer - 1).unwrap().to_owned();
}

fn apply_single_cycle(content: &mut Vec<Vec<char>>) {
    (0..4).for_each(|_| {
        transform_clockwise(content);
        tilt_east(content);
    });
}

fn transform_clockwise(content: &mut Vec<Vec<char>>) {
    let rows = content.len();
    let cols = content.first().unwrap_or(&Vec::new()).len();

    *content = (0..cols).map(|y| (0..rows).rev().map(|x| content.iter().nth(x).unwrap().iter().nth(y).unwrap().to_owned()).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
}

fn tilt_east(content: &mut Vec<Vec<char>>) {
    content.iter_mut().for_each(|line| {
        line.split_mut(|&c| c == '#').for_each(|chunk| chunk.sort_by(|&a, &b| match (a, b) {
                ('O', '.') => std::cmp::Ordering::Greater,
                ('.', 'O') => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            }));
    });
}

fn calculate_load(content: Vec<Vec<char>>) -> usize {
    let total_lines = content.len();
    content.into_iter().enumerate().fold(0usize, |acc, line| acc + line.1.into_iter().filter(|&c| c == 'O').count() * (total_lines - line.0))
}