use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let cycles = 1_000_000_000usize;

    let total_rows = content.lines().count();

    let (mut rounded_rocks, cube_rocks) = {
        let (r, c) = content.lines().enumerate().flat_map(|(line_idx, line)| line.chars().enumerate().filter_map(move |(idx, c)|
        match c {
            '.' => None,
            c => Some((c, (line_idx, idx))),
        }
    )).partition::<Vec<(char, (usize, usize))>, _>(|&(c, _)| c == 'O');

        (r.into_iter().map(|(_, pos)| pos).collect::<Vec<(usize, usize)>>(), c.into_iter().map(|(_, pos)| pos).collect::<Vec<(usize, usize)>>())
    };

    apply_cycle(&mut rounded_rocks, &cube_rocks);

    let ans = calculate_load(total_rows, rounded_rocks);
    println!("ANS: {0}", ans);
}

fn apply_cycle(rounded_rocks: &mut Vec<(usize, usize)>, cube_rocks: &Vec<(usize, usize)>) {
    todo!();
}

fn calculate_load(total_rows: usize, rounded_rocks: Vec<(usize, usize)>) -> usize {
    rounded_rocks.into_iter().fold(0usize, |acc, (_, y)| acc + total_rows - y)
}
