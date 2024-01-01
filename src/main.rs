use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.lines().fold(0u32, |acc, line| {
        let mut record_iter = line.split_whitespace().into_iter();
        let (spring_map, contiguous_springs) = (record_iter.next().unwrap_or_else(|| panic!("Input error! Spring map not found.")), record_iter.next().unwrap_or_else(|| panic!("Input error! Contiguous spring array not found.")).split(',').map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("Error parsing. Invalid Input!"))).collect::<Vec<usize>>());

        acc + permutations(spring_map, contiguous_springs)
    });

    println!("ANS: {0}", ans);
}

fn permutations(row_of_springs: &str, mut contiguous_springs: Vec<usize>) -> u32 {
    if contiguous_springs.is_empty() {
        return 0;
    }
    let min_len = contiguous_springs.iter().sum::<usize>() + contiguous_springs.len() - 1;
    if min_len > row_of_springs.len() {
        return 0;
    }
    let first_contiguous_springs_count = contiguous_springs.remove(0);
    let potential_len = min_len - first_contiguous_springs_count - 1;
    let max_split_at = row_of_springs.len() - potential_len - row_of_springs.chars().rev().skip(potential_len).take_while(|&c| c == '#').count();
    (first_contiguous_springs_count..max_split_at).fold(0u32, |acc, i| {
        let (a, b) = row_of_springs.split_at(i + 1);
        let mut a = (&a[a.len() - first_contiguous_springs_count - 1..]).chars().rev();
        let b = b.trim_start_matches(|c| c == '.');

        match a.next().map_or(false, |last| last == '.' || last == '?') && a.all(|c| c == '#' || c == '?') {
            true => acc + permutations(b, contiguous_springs.clone()),
            false => acc,
        }
    })
}
