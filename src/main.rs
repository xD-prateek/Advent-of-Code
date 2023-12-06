use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let num_val_map = vec![
        (1u32, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];

    // map of values and strings
    let num_string_map = num_val_map.iter().map(|(val, key)| (key.to_string(), *val)).collect::<HashMap<String, u32>>();
    let num_rev_string_map = num_val_map.iter().map(|(val, key)| (key.chars().rev().collect::<String>(), *val)).collect::<HashMap<String, u32>>();

    let mut ans: u32 = 0;
    content.lines().for_each(|line| {
        // println!("{0}", line);
        let first_num = fetch_first_num(line.to_string() , &num_string_map);
        let last_num = fetch_first_num(line.chars().rev().collect::<String>(), &num_rev_string_map);
        // println!("{0}{1}", first_num.unwrap(), last_num.unwrap());
        if let (Some(f), Some(l)) = (first_num, last_num) {
            ans += f * 10 + l;
        }
    });
    println!("ANS: {0}", ans);
}

fn fetch_first_num(line: String, map: &HashMap<String, u32>) -> Option<u32> {
    let vec_of_words = map.keys().map(String::as_str).collect::<Vec<&str>>();
    for (i, c) in line.char_indices() {
        // if c.is_digit(10) {return c.to_digit(10)};
        if let Some(digit) = c.to_digit(10) {
            // println!("Number: {0}", digit);
            return Some(digit);
        }
        let mut query = c.to_string();
        let mut vec = vec_of_words.clone();
        while !vec.is_empty() {
            // println!("Query: {0}", query);
            vec.retain(|&ele| ele.starts_with(&query));
            if let Some(word) = vec.get(0) {
                if i + word.len() <= line.len() && &line[i..i + word.len()] == *word {
                    // println!("word: {0}", word);
                    return map.get(*word).cloned();
                }
            }

            if let Some(c) = line.chars().nth(i + query.len()) {
                match c.is_alphabetic() {
                    true => query.push(c),
                    false => break,
                };
            }
        }
    }
    None
}
