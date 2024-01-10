use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.split_terminator("\n\n").fold(0usize, |acc, s| acc + 100 * get_rows_above_mirror(s) + get_columns_before_mirror(s));
    println!("ANS: {0}", ans);
}

fn get_columns_before_mirror(valley: &str) -> usize {
    let mut lines_iter = valley.lines();
    let mut smudge_fixed = false;
    let potential_mirrors = {
        let first_line = lines_iter.next().unwrap();
        (1..first_line.len()).filter(|&i| {
            let left = &first_line[..i];
            let right = &first_line[i..];
            match right.chars().zip(left.chars().rev()).filter(|(a, b)| a != b).count() {
                1 => {
                    smudge_fixed = true;
                    true
                }
                0 => true,
                _ => false,
            }
        }).collect::<Vec<usize>>()
    };

    let mirror = lines_iter.fold(potential_mirrors, |mut acc, line| {
        acc.retain(|&i| {
            let left = &line[..i];
            let right = &line[i..];
            match smudge_fixed {
                true => right.chars().zip(left.chars().rev()).all(|(a, b)| a == b),
                false => {
                    match right.chars().zip(left.chars().rev()).filter(|(a, b)| a != b).count() {
                        1 => {
                            smudge_fixed = true;
                            true
                        }
                        0 => true,
                        _ => false,
                    }
                },
            }
        });
        acc
    });

    let ans = match mirror.len() {
        1 if smudge_fixed => mirror.get(0).unwrap().to_owned(),
        _ => 0,
    };
    println!("columns before: {0}", ans);
    ans
}

fn get_rows_above_mirror(valley: &str) -> usize {
    let ans = (1..valley.lines().count()).fold(0usize, |acc, i| {
        let valley_of_lines = valley.lines().collect::<Vec<&str>>();
        let top = &valley_of_lines[..i];
        let bottom = &valley_of_lines[i..];

        let smudge_fixed = false;
        bottom.into_iter().zip(top.into_iter().rev()).fold(0usize, |acc, (&a, &b)| {
            match a.chars().zip(b.chars()).filter(|(&a, &b)| a != b).count() {
                1 if !smudge_fixed => smudge_fixed = true,
                
            }
        });
        // return i if top and bottom only has one difference
        // else return 0/acc
        todo!();

    });
    println!("above rows: {0}", ans);
    ans
}