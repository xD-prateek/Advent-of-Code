use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let no_of_lines = content.lines().count() as u32;
    let ans = content.lines().enumerate().fold((0..no_of_lines).map(|key| (key, 1)).collect::<HashMap<u32, u32>>(), |mut acc, (l_index, line)| {
        let l_index = l_index as u32;
        let mut nums = line.split(':').nth(1).unwrap().split('|').map(|nums| nums.split_whitespace().map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>());
        let (winning_nums, my_nums) = (nums.next().unwrap(), nums.next().unwrap());
        let wins = my_nums.into_iter().filter(|i| winning_nums.contains(i)).count() as u32;
        let cards = acc.get(&l_index).unwrap_or(&1).clone();
        (1..=wins.min(no_of_lines - l_index - 1)).for_each(|i| {
            acc.entry(l_index + i).and_modify(|v| *v += cards).or_insert(2u32);
        });
        acc
    }).values().sum::<u32>();
    println!("ANS: {0}", ans);
}
