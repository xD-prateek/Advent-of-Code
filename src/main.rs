mod hand;
use hand::Hand;
use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let mut game = content.lines().map(|l| {
        let mut line_iter = l.split_whitespace().take(2);
        (Hand::new(line_iter.next().unwrap()), line_iter.next().unwrap().parse::<u64>().unwrap())
    }).collect::<Vec<(Hand, u64)>>();

    game.sort_by(|a, b| a.0.cmp(&b.0));
    let ans = game.into_iter().enumerate().map(|(i, (_, bid))| (i as u64 + 1) * bid).sum::<u64>();

    println!("ANS: {0:?}", ans);
}
