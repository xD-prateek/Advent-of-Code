use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.split_terminator("\n\n").fold(0usize, |acc, s| acc + 100 * get_above_rows(s) + get_previous_columns(s));
    println!("ANS: {0}", ans);
}

fn get_previous_columns(valley: &str) -> usize {
    let mut lines = valley.lines();
    let p = lines.next().unwrap().char_indices().collect::<Vec<(usize, char)>>().windows(2).filter(|&s| s[0].1 == s[1].1).map(|s| (s[0].0, s[1].0)).collect::<Vec<(usize, usize)>>();
    let split = lines.fold(p, |mut acc, line| {
        acc.retain(|c| line.chars().nth(c.0).unwrap() == line.chars().nth(c.1).unwrap());
        acc
    });
    match split.get(0) {
        Some(k) if split.len() == 1 => k.1,
        _ => 0,
    }
}

fn get_above_rows(valley: &str) -> usize {
    // println!("valley horiz: {0}", valley);
    0
}