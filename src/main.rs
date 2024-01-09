use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.split_terminator("\n\n").fold(0usize, |acc, s| acc + 100 * get_above_rows(s) + get_previous_columns(s));
    println!("ANS: {0}", ans);
}

fn get_previous_columns(valley: &str) -> usize {
    let mut lines_iter = valley.lines();
    let potential_mirrors = {
        let first_line = lines_iter.next().unwrap();
        (1..first_line.len()).filter(|&i| {
            let left = &first_line[..i];
            let right = &first_line[i..];
            right.chars().zip(left.chars().rev()).all(|(a, b)| a == b)
        }).collect::<Vec<usize>>()
    };
    let mirror = lines_iter.fold(potential_mirrors, |mut acc, line| {
        acc.retain(|&i| {
            let left = &line[..i];
            let right = &line[i..];
            right.chars().zip(left.chars().rev()).all(|(a, b)| a == b)
        });
        acc
    });

    match mirror.len() {
        1 => mirror.get(0).unwrap().to_owned(),
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
        let top = &valley_of_lines[..i];
        let bottom = &valley_of_lines[i..];

        match bottom.into_iter().zip(top.into_iter().rev()).all(|(&a, &b)| a == b) {
            true => i,
            false => acc,
        }
    })
}