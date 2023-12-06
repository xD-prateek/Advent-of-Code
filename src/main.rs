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

    println!("{0:?}", num_string_map);
    println!("{0:?}", num_rev_string_map);

    let mut ans: u32 = 0;
    content.lines().for_each(|line| {
        let first_num = fetch_first_num(line.to_string() , &num_string_map);
        let last_num = fetch_first_num(line.chars().rev().collect::<String>(), &num_rev_string_map);

        if let (Some(f), Some(l)) = (first_num, last_num) {
            ans += f * 10 + l;
        }
    });
    println!("ANS: {0}", ans);
}

fn fetch_first_num(line: String, map: &HashMap<String, u32>) -> Option<u32> {
    let vec_of_words = map.keys().map(|ele| ele.as_str()).collect::<Vec<&str>>();
    let word = get_word(&line, vec_of_words);
    // if let Some(occurance) = map.keys().find(|&s| line.contains(s)) {
    //     println!("{0}", occurance);
    // }
    match word {
        Some(w) => todo!(),
        None => None,
    }
}

fn get_word(line: &str, vec_of_words: Vec<&str>) -> Option<&str> {
    line.chars().skip_while(|ele| {
    
    });
    None
}
