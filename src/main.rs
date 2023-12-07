use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let ans:u32 = content.lines().map(|line| {
        line.split(':').nth(1).unwrap_or_default().split(';').flat_map(|set| {
            set.split(',').filter_map(|subset| {
                let mut str_iter = subset.split_whitespace();
                Some((str_iter.next()?.parse::<u32>().unwrap_or_default(), str_iter.next()?,))
            })
        }).fold(HashMap::new(), |mut min_cubes, (no, color)| {
            min_cubes.entry(color).and_modify(|v| *v = no.max(*v)).or_insert(no);
            min_cubes
        }).values().product::<u32>()
    }).sum();
    println!("ANS: {0}", ans);
}
