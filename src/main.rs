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
    match contiguous_springs.pop() {
        Some(matched) => {
            println!("To match: {0}", matched);
            let mut springs = row_of_springs.char_indices();
        // put in loop
        let mut ans = 0u32;
        for i in 0..contiguous_springs.iter().sum::<usize>() - contiguous_springs.len() + matched - 1 {
            let springs_iter = springs.by_ref();
            if springs_iter.skip(i).take(matched).all(|(_, c)| c != '.') {
                println!("MATCHED! ans: {0}", ans);
                // ans += 1;
                // its is a valid match
                match springs_iter.next() {
                    Some((i, c)) if c != '#' => {
                        let next_slice = &row_of_springs[i + 1..];
                        // if the remaining length is greater than the remaining sum, then go for furthur permutations
                        // if the remaining length id equal to the remaining sum, add 1
                        // else return 0
                        ans += permutations(next_slice, contiguous_springs)
                    },
                    None => return ans + 1,
                    _ => continue,
                }
            }
        }
        ans
    }
    None => 0
}
}
