use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.lines().fold(0u32, |acc, line| {
        let mut record_iter = line.split_whitespace().into_iter();
        let (spring_map, mut contiguous_springs) = (String::from(record_iter.next().unwrap_or_else(|| panic!("Input error! Spring map not found."))), record_iter.next().unwrap_or_else(|| panic!("Input error! Contiguous spring array not found.")).split(',').map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("Error parsing. Invalid Input!"))).collect::<Vec<usize>>());

        acc + permutations(&spring_map, &mut contiguous_springs)
    });

    println!("ANS: {0}", ans);
}

fn permutations(row_of_springs: &str, contiguous_springs: &[usize]) -> u32 {
    let row_of_springs = row_of_springs.trim_matches(|c| c == '.');
    match contiguous_springs.first() {
        Some(&matched) => {
            let springs = row_of_springs.char_indices();
            let no_of_iterations = {
                let remaining_len = match contiguous_springs[1..].iter().sum::<usize>() + contiguous_springs[1..].len() {
                    0 => 0,
                    n => n - 1,
                };
                match row_of_springs.len() + 1 < remaining_len + matched {
                    false => {
                        let max_iterations = row_of_springs.len() + 1 - remaining_len - matched;
                        match row_of_springs.find('#') {
                            Some(back_limit) => max_iterations.min(back_limit + 1),
                            None => max_iterations,
                        }
                    },
                    true => return 0,
                }
            };
            (0..no_of_iterations).fold(0u32, |acc, i| {
                let mut springs_iter = springs.clone();
                match springs_iter.by_ref().skip(i).take(matched).all(|(_, c)| c != '.') {
                    true => {
                        if let Some((j, c)) = springs_iter.next() {
                            match c != '#' {
                                true => {
                                    acc + match contiguous_springs.len() {
                                        1 if !row_of_springs[j..].contains('#') => 1,
                                        _ => permutations(&row_of_springs[j + 1..], &contiguous_springs[1..]),
                                    }
                                },
                                false => acc,
                            }
                        }
                        else {
                            acc + 1
                        }
                    },
                    false => acc,
                }
            })
        }
        None => 0,
    }
}
