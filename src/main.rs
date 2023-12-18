use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let mut string_iter = content.lines().map(|l| l.split(':').skip(1).next().unwrap().chars().filter(|&c| !c.is_whitespace()).collect::<String>()).map(|s| s.parse::<f64>().unwrap());
    let (t, r) = (string_iter.next().unwrap(), string_iter.next().unwrap());

    let d = (t * t - 4.0 * r).sqrt();
    let r1 = (t + d) / 2.0;
    let r2 = (t - d) / 2.0;
    let ans = match r1.fract() != 0.0 {
        false => r1.floor() as u32 - r2.floor() as u32 - 1,
        true => r1.floor() as u32 - r2.floor() as u32,
    };
    println!("ANS: {0}", ans);
}
