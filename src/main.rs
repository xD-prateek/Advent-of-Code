use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.lines().fold(0u32, |acc, line| {
        let mut record_iter = line.split_whitespace().into_iter();
        let (spring_map, mut contiguous_springs) = (String::from(record_iter.next().unwrap_or_else(|| panic!("Input error! Spring map not found."))), record_iter.next().unwrap_or_else(|| panic!("Input error! Contiguous spring array not found.")).split(',').map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("Error parsing. Invalid Input!"))).rev().collect::<Vec<usize>>());

        acc + permutations(&spring_map, &mut contiguous_springs)
    });

    println!("ANS: {0}", ans);
}

fn permutations(row_of_springs: &str, contiguous_springs: &mut Vec<usize>) -> u32 {
    println!("row_of_springs: {0}, contiguous_springs: {1:?}", row_of_springs, contiguous_springs);
    let row_of_springs = row_of_springs.trim_start_matches(|c| c == '.');
    if row_of_springs.is_empty() {
        return 0;
    }
    match contiguous_springs.pop() {
        None => 0,
        Some(to_match) => {
            if contiguous_springs.is_empty() {
                return match row_of_springs.contains('#') {
                    true => 0,
                    false => 1,
                };
            }
            let mut springs_iter = row_of_springs.char_indices();
            let mut combinations = 0u32;
            for _ in 0..contiguous_springs.iter().sum::<usize>() + contiguous_springs.len() - 1 {
                for i in 0..to_match {
                    match springs_iter.next() {
                        Some((_, c)) => {
                            if c == '.' {
                                contiguous_springs.push(to_match);
                                combinations += permutations(&row_of_springs[i..], contiguous_springs);
                                break;
                            }
                        },
                        None => return 0,
                    }
                }
                if let Some((i, c)) = springs_iter.next() {
                    if c != '#' {
                        combinations += permutations(&row_of_springs[i..], contiguous_springs);
                        break;
                    }
                    else {
                        panic!("Invalid input. Springs did not match.");
                    }
                }
                else {
                    return 1;
                }
            }
            combinations
        },
    }
}
