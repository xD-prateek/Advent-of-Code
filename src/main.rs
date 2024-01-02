use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.lines().fold(0u32, |acc, line| {
        let mut record_iter = line.split_whitespace().into_iter();
        let (spring_map, contiguous_springs) = (String::from(record_iter.next().unwrap_or_else(|| panic!("Input error! Spring map not found."))), record_iter.next().unwrap_or_else(|| panic!("Input error! Contiguous spring array not found.")).split(',').map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("Error parsing. Invalid Input!"))).rev().collect::<Vec<usize>>());

        acc + permutations(&spring_map, contiguous_springs)
    });

    println!("ANS: {0}", ans);
}

fn permutations(row_of_springs: &str, mut contiguous_springs: Vec<usize>) -> u32 {
    // trim '.'; this will reduce the recursive calls
    let row_of_springs = row_of_springs.trim_start_matches(|c| c == '.');
    if let Some(matched) = contiguous_springs.pop() {
        let mut str_iter = row_of_springs.chars();
        if str_iter.by_ref().take(matched).all(|c| c != '.') {
            if let Some(period) = str_iter.next() {
                if period == '#' {
                   contiguous_springs.push(matched);
                   permutations(&row_of_springs[1..], contiguous_springs)
                }
                else {
                    // continue by splitting the remaining string
                    permutations(&row_of_springs[matched + 1..], contiguous_springs)
                }
            }
            else {
                1
            }
        }
        else {
            // call permutations with the remaining string
            contiguous_springs.push(matched);
            permutations(&row_of_springs[1..], contiguous_springs)
        }

    }
    else {
        0
    }
}
