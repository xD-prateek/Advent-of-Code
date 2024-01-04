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
    let matched = contiguous_springs.pop().unwrap();
    println!("To match: {0}", matched);
    let mut springs = row_of_springs.char_indices();
    // put in loop
    let mut ans = 0u32;
    let permute = match contiguous_springs.iter().sum::<usize>() - contiguous_springs.len() {
        0 => 0,
        n => n - 1,
    };
    for i in 0..permute {
        let springs_iter = springs.by_ref();
        if springs_iter.skip(i).take(matched).all(|(_, c)| c != '.') {
            println!("MATCHED!");
            // its is a valid match
            match springs_iter.next() {
                Some((i, c)) if c != '#' && !contiguous_springs.is_empty() => ans += permutations(&row_of_springs[i + 1..], contiguous_springs),
                _ => return 1,
            }
        }
    }
    ans
}
