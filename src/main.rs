use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.split_terminator("\n\n").fold(0usize, |acc, s| acc + 100 * get_rows_above_mirror(s) + get_columns_before_mirror(s));
    println!("ANS: {0}", ans);
}

fn get_columns_before_mirror(valley: &str) -> usize {
    let lines_iter = valley.lines();
    let mut potential_mirrors = vec!{0usize; lines_iter.by_ref().next().unwrap_or("").len()};
    // for each line
    //      for each i
    //          iretain the index if a != b count == 0 or 1

    lines_iter.for_each(|line| {
        let ans = potential_mirrors.iter_mut().enumerate().filter(|(&i, &val)| {
            let (left, right) = line.split_at(i + 1);
            match right.chars().zip(left.chars().rev()).filter(|(a, b)| a != b) == 1 {
                true => {
                    *val += 1;
                    true
                }
                false => false,
            }
        });
    })
}

fn get_rows_above_mirror(valley: &str) -> usize {
    let ans = (1..valley.lines().count()).fold(0usize, |acc, i| {
        let valley_of_lines = valley.lines().collect::<Vec<&str>>();
        let top = &valley_of_lines[..i].to_vec().into_iter().rev().flat_map(|c| c.chars()).collect::<String>();
        let bottom = &valley_of_lines[i..].into_iter().flat_map(|c| c.chars()).collect::<String>();

        match top.chars().zip(bottom.chars()).filter(|(a, b)| a != b).count() == 1 {
            true => i,
            false => acc,
        }
    });
    println!("above rows: {0}", ans);
    ans
}