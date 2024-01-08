use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.split_terminator("\n\n").fold(0usize, |acc, s| {
        // let a = get_above_rows(s.1);
        // let p = get_previous_columns(s.1);
        // if a != 0 && p != 0 {
        //     println!("{0}", s.1);
        //     println!("a: {0}, p: {1}, at {2}\n", a, p, s.0);
        // }
        // 0
        acc + 100 * get_above_rows(s) + get_previous_columns(s)
    });
    println!("ANS: {0}", ans);
}

fn get_previous_columns(valley: &str) -> usize {
    let mut lines = valley.lines();
    let p = lines.next().unwrap().char_indices().collect::<Vec<(usize, char)>>().windows(2).filter(|&s| s[0].1 == s[1].1).map(|s| (s[0].0, s[1].0)).collect::<Vec<(usize, usize)>>();
    let split = lines.fold(p, |mut acc, line| {
        acc.retain(|c| {
            let (left, right) = line.split_at(c.0);
            right.chars().zip(left.chars().rev()).all(|(a, b)| a == b)
        });
        acc
    });
    println!("split: {0:?}", split);
    match split.get(0) {
        Some(k) if split.len() == 1 => k.1,
        _ => 0,
    }
}

fn get_above_rows(valley: &str) -> usize {
    let mut lines = valley.lines().enumerate().peekable();
    let mut potential_lines = Vec::new();
    while let Some(line) = lines.next() {
        if let Some(&next_line) = lines.peek() {
            if line.1 == next_line.1 {
                potential_lines.push(next_line.0);
            }
        }
    }

    potential_lines.into_iter().fold(0usize, |acc, i| {
        let valley_of_lines = valley.lines().collect::<Vec<&str>>();
        let top = &valley_of_lines[0..i];
        let bottom = &valley_of_lines[i..];

        match bottom.into_iter().zip(top.into_iter().rev()).all(|(&a, &b)| a == b) {
            true => i,
            false => acc,
        }
    })
}