use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let mut line_iter = content.lines();

    let mut directions_iter = line_iter.next().unwrap().chars().filter_map(|c| Dir::new_from_str(c)).cycle();

    let map: HashMap<&str, (&str, &str)> = line_iter.filter(|l| !l.is_empty()).fold(HashMap::new(), |mut acc, l| {
        let mut kv_iter = l.split('=');
        let key = kv_iter.next().unwrap().trim();
        let mut value_iter = kv_iter.next().unwrap().split(',').map(|v| v.trim().trim_matches(|c| c == '(' || c == ')'));
        let val = (value_iter.next().unwrap(), value_iter.next().unwrap());
        acc.insert(key, val);
        acc
    });
    // println!("{0:#?}", map);

    let mut next = "AAA";
    let mut ans = 0u32;

    while next != "ZZZ" {
        use Dir::{R, L};
        next = match directions_iter.next().unwrap() {
            L => map.get(next).unwrap().0,
            R => map.get(next).unwrap().1,
        };
        ans += 1;

    }
    println!("ANS: {0}", ans);
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
