use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let td = content.lines().map(|l| l.split(':').skip(1).next().unwrap().split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    let ans = td.get(0).unwrap().into_iter().zip(td.get(1).unwrap().into_iter()).map(|(&t, &d)| (t as f32, d as f32)).map(|(t, r)| {
        let d = (t * t - 4f32 * r).sqrt();
        let r1 = (t + d) / 2.0;
        let r2 = (t - d) / 2.0;
        match r1.fract() != 0.0 {
            false => r1.floor() as u32 - r2.floor() as u32 - 1,
            true => r1.floor() as u32 - r2.floor() as u32,
        }
    }).product::<u32>();
    println!("ANS: {0}", ans);
}
