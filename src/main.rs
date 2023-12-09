use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.lines().fold(0u32, |mut acc, line| {
        let mut nums = line.split(':').nth(1).unwrap().split('|').map(|nums| nums.split_whitespace().map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>());
        let (winning_nums, my_nums) = (nums.next().unwrap(), nums.next().unwrap());
        let wins = my_nums.into_iter().filter(|i| winning_nums.contains(i)).count() as u32;
        if wins > 0 {
            acc += 2u32.pow(wins - 1);
        }
        acc
    });

    println!("ANS: {0}", ans);
}
