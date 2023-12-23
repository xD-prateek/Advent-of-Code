use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans = content.lines().fold(0i32, |acc, line| {
        let history = line.split_whitespace().map(|num| num.parse::<i32>().unwrap_or_default()).collect::<Vec<i32>>();
        let extrapolate = get_next(history);
        acc + extrapolate
    });

    println!("ANS: {0}", ans);
}

fn get_next(reading: Vec<i32>) -> i32 {
    match reading.iter().all(|&num| num == 0) {
        true => 0,
        false => reading.first().unwrap() - get_next(reading.windows(2).map(|ele| ele[1] - ele[0]).collect::<Vec<i32>>()),
    }
}
