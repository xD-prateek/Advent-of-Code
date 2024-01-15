use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.split_terminator("\n\n").fold(0usize, |acc, s| acc + 100 * get_rows_above_mirror(s) + get_columns_before_mirror(s));
    println!("ANS: {0}", ans);
}

fn get_columns_before_mirror(valley: &str) -> usize {
    let lines_iter = valley.lines();
    let mut potential_mirrors = (1..lines_iter.clone().next().unwrap_or("").len()).map(|i| (i, false)).collect::<Vec<(usize, bool)>>();

    lines_iter.for_each(|line| {
        potential_mirrors.retain_mut(|(idx, i)| {
            let (left, right) = line.split_at(*idx);
            match right.chars().zip(left.chars().rev()).filter(|(a, b)| a != b).count() {
                0 => true,
                1 if !*i => {
                    *i = true;
                    true
                }
                _ => false,
            }
        });
    });
    match potential_mirrors.into_iter().find(|c| c.1) {
        Some(c) => c.0,
        None => 0,
    }
}

fn get_rows_above_mirror(valley: &str) -> usize {
    let valley_of_lines = valley.lines().collect::<Vec<&str>>();
    (1..valley.lines().count()).fold(0usize, |acc, i| {
        let top = &valley_of_lines[..i].to_vec().into_iter().rev().flat_map(|c| c.chars()).collect::<String>();
        let bottom = &valley_of_lines[i..].into_iter().flat_map(|c| c.chars()).collect::<String>();

        match top.chars().zip(bottom.chars()).filter(|(a, b)| a != b).count() == 1 {
            true => i,
            false => acc,
        }
    })
}