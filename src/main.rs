use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let no_of_cubes = vec!{
        ("red", 12u32),
        ("green", 13),
        ("blue", 14)
    };

    let cubes = no_of_cubes.into_iter().collect::<HashMap<&str, u32>>();
    let mut ans = 0u32;

    content.lines().for_each(|line| {
        if let [id_str, sets_str] = line.split(':').collect::<Vec<&str>>().as_slice() {
            if sets_str.split(';').collect::<Vec<&str>>().iter().all(|set| {
                set.split(',').all(|val| {
                    if let [no_str, color] = val.split_whitespace().collect::<Vec<&str>>().as_slice() {
                        no_str.parse::<u32>().unwrap() < cubes.get(color).cloned().unwrap_or_else(|| panic!("Color cannot be found in map!"))
                    }
                    else {
                        panic!("Some error in input.");
                    }
                })
            }) {
                println!("{0}", id_str);
                ans += id_str.split_whitespace().nth(1).and_then(|s| s.parse::<u32>().ok()).expect("Not able to fetch ID");
            }
        }
    });
    println!("ANS: {0}", ans);
}
