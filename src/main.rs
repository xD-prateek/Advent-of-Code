use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let copies = 5;
    let ans = get_arrangements(content, copies);

    println!("ANS: {0}", ans);
}

fn get_arrangements(content: String, copies: usize) -> u64 {
    let mut cache = Cache::new();
    content.lines().fold(0u64, |acc, line| {
        let mut record_iter = line.split_whitespace().into_iter();
        let spring_record = record_iter.next().unwrap_or_else(|| panic!("Input error! Spring map not found."));
        let (spring_map, contiguous_springs) = ((0..copies).map(|_| String::from(spring_record)).collect::<Vec<String>>().join("?"), record_iter.next().unwrap_or_else(|| panic!("Input error! Contiguous spring array not found.")).split(',').map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("Error parsing. Invalid Input!"))).collect::<Vec<usize>>().repeat(copies));
        acc + cache.permutations(&spring_map, &contiguous_springs)
    })
}


struct Cache(HashMap<(String, Vec<usize>), u64>);

impl Cache {
    fn new() -> Self {
        Self(HashMap::new())
    } 

    fn permutations(&mut self, row_of_springs: &str, contiguous_springs: &[usize]) -> u64 {
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
                (0..no_of_iterations).fold(0u64, |acc, i| {
                    let mut springs_iter = springs.clone();
                    match springs_iter.by_ref().skip(i).take(matched).all(|(_, c)| c != '.') {
                        true => {
                            if let Some((j, c)) = springs_iter.next() {
                                match c != '#' {
                                    true => {
                                        acc + match contiguous_springs.len() {
                                            1 if !row_of_springs[j..].contains('#') => 1,
                                            _ => {
                                                match self.0.get(&(String::from(&row_of_springs[i..]), Vec::from(contiguous_springs))) {
                                                    Some(&val) => val,
                                                    None => {
                                                        let a = self.permutations(&row_of_springs[j + 1..], &contiguous_springs[1..]);
                                                        self.0.insert((String::from(&row_of_springs[i..]), Vec::from(contiguous_springs)), a);
                                                        a
                                                    },
                                                }
                                            },
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
}