use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let mut ans: u32 = 0;
    content.lines().for_each(|line| {
        // println!("{0}", line);
        let first_num = line.chars().find(|ele| ele.is_digit(10));
        let last_num = line.chars().rev().find(|ele| ele.is_digit(10));
        if let (Some(f), Some(l)) = (first_num, last_num) {
            // println!("{0}{1}", f, l);
            ans += 10 * f.to_digit(10).unwrap() + l.to_digit(10).unwrap();
        }
    });
    println!("ANS: {0}", ans);
}
