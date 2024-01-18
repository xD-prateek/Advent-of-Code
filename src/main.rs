use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let lines = content.lines().count();
    let mut col_map: HashMap<usize, usize> = HashMap::new();
    let ans = content.lines().enumerate().fold(0usize, |mut acc, (line_idx, line)| {
        line.chars().enumerate().for_each(|(i, c)| {
            acc += match c {
                'O' => {
                    lines - *col_map.entry(i).and_modify(|pos| *pos += 1).or_insert(0)
                },
                c => {
                    if c == '#' {
                        col_map.entry(i).and_modify(|pos| *pos = line_idx).or_insert(line_idx);
                    }
                    0
                },
            }
        });
        acc
    });
    println!("ANS: {0}", ans);
}
