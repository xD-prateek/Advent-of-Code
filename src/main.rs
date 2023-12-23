use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let mut line_iter = content.lines();

    let directions_iter = line_iter.next().unwrap().chars().filter_map(|c| Dir::new_from_str(c)).cycle();

    let map: HashMap<&str, (&str, &str)> = line_iter.filter(|l| !l.is_empty()).fold(HashMap::new(), |mut acc, l| {
        let mut kv_iter = l.split('=');
        let key = kv_iter.next().unwrap().trim();
        let mut value_iter = kv_iter.next().unwrap().split(',').map(|v| v.trim().trim_matches(|c| c == '(' || c == ')'));
        let val = (value_iter.next().unwrap(), value_iter.next().unwrap());
        acc.insert(key, val);
        acc
    });
    // println!("{0:#?}", map);

    let next = map.keys().filter_map(|&key| {
        match key.ends_with('A') {
            true => Some(key),
            false => None,
        }
    }).collect::<Vec<&str>>();

    let mut lcm = 1u64;
    next.into_iter().for_each(|start| {
        let mut directions = directions_iter.clone();
        let mut first_z = None;
        let mut current = start;
        let mut steps = 0u64;
        loop {
            use Dir::{ R, L };
            current = match directions.next().unwrap() {
                R => map.get(current).unwrap().1,
                L => map.get(current).unwrap().0,
            };

            if current.ends_with('Z') {
                if first_z.is_none() {
                    first_z = Some(current);
                    steps = 0;
                }
                else if current == first_z.unwrap_or_default() {
                    lcm = lcm * steps / gcd(steps, lcm);
                    break;
                }
            }
            steps += 1;
        }
    });

    println!("ANS: {0}", lcm);
}

enum Dir {
    R,
    L,
}

impl Dir {
    fn new_from_str(dir: char) -> Option<Self> {
        match dir {
            'R' => Some(Self::R),
            'L' => Some(Self::L),
            _ => None,
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    match a {
        0 => b,
        _ => gcd(b % a, a),
    }
}
