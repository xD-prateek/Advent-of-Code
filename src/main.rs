use std::{fs::read_to_string, collections::BTreeSet};

// representing destination range start, source range start, range length
#[derive(Eq, Debug)]
struct Map(u64, u64, u64);

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        (self.1..self.2).contains(&other.2)
    }
}

fn main() {
    let file_name: &str = "src/input.txt";
    let almanac = read_to_string(file_name).unwrap();
    let s = almanac.lines().collect::<Vec<&str>>().split(|&line| line.is_empty()).map(|l| l.to_vec()).collect::<Vec<Vec<&str>>>();
    let mut content_iter = s.into_iter();
    let seeds_range = content_iter.next().into_iter().flat_map(|s| s.get(0).unwrap_or_else(|| panic!("seeds not found...")).split(':').nth(1).unwrap_or_else(|| panic!("error reading seeds...")).split_whitespace().map(|num| num.parse::<u64>().unwrap())).collect::<Vec<u64>>();

    let mapping = content_iter.map(|v| {
        v.into_iter().skip(1).fold(BTreeSet::new(), |mut acc, m| {
            let mut map_iter = m.split_whitespace().map(|v| v.parse::<u64>().unwrap());
            acc.insert(Map(map_iter.next().unwrap(), map_iter.next().unwrap(), map_iter.next().unwrap()));
            acc
        })
    }).collect::<Vec<BTreeSet<Map>>>();

    let ans = seeds_range.chunks(2).map(|nr| get_min_location((nr[0], nr[1]), &mapping)).min().unwrap();

    println!("ANS: {0}", ans);
}

fn get_min_location(seeds: (u64, u64), bt_map: &Vec<BTreeSet<Map>>) -> u64 {
    bt_map.into_iter().fold(vec!{seeds}, |carry_forward, m| {
        carry_forward.into_iter().fold(Vec::new(), |mut acc, seed| {
            let mut new_map = create_mapping(seed, m).unwrap();
            acc.append(&mut new_map);
            acc
        })
    }).into_iter().map(|(first, _)| first).min().unwrap()
}

fn create_mapping(seeds: (u64, u64), bt: &BTreeSet<Map>) -> Result<Vec<(u64, u64)>, String> {
    let mut map_iter = bt.into_iter().filter(|&m| !(m.1 + m.2 < seeds.0 || m.1 >= seeds.0 + seeds.1)).peekable();
    if let None = map_iter.peek() {
        return Ok(vec!{seeds});
    }
    // until is not included
    let mut until = seeds.0;

    let mut transformed = Vec::new();
    while let Some(m) = map_iter.next() {
        if m.1 <= seeds.0 {
            if m.1 + m.2 < seeds.0 + seeds.1 {
                until = m.1 + m.2;
                transformed.push((seeds.0 + m.0 - m.1 , until - seeds.0));
            }
            else {
                until = seeds.0 + seeds.1;
                transformed.push((seeds.0 + m.0 - m.1, seeds.1));
            }
        }
        else {
            if m.1 > until {
                transformed.push((until, m.1 - until));
            }
            if seeds.0 + seeds.1 > m.1 && seeds.0 + seeds.1 <= m.1 + m.2 {
                until = seeds.0 + seeds.1;
                transformed.push((m.0, until - m.1));
            }
            else {
                until = m.1 + m.2;
                transformed.push((m.0, m.2));
            }
        }
        if map_iter.peek() == None && until != seeds.0 + seeds.1 {
            transformed.push((until, seeds.0 + seeds.1 - until));
        }
    }
    Ok(transformed)
}
