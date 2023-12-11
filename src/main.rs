use std::{fs::read_to_string, collections::BTreeSet};

#[derive(Eq)]
// representing destination range start, source range start, range length
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
    let seeds = content_iter.next().map(|s| s.get(0).unwrap_or_else(|| panic!("seeds not found...")).split(':').nth(1).unwrap_or_else(|| panic!("error reading seeds...")).split_whitespace().flat_map(|num| num.parse::<u64>()).unwrap().collect::<Vec<u64>>()).unwrap().chunks(2).map(|c| (c[0], c[1])).flat_map(|(num, till)| {
        (num..num+till).into_iter()
    }).collect::<Vec<u64>>();
    // println!("{0:?}", seeds);

    let mapping = content_iter.map(|v| {
        v.into_iter().skip(1).fold(BTreeSet::new(), |mut acc, m| {
            let mut map_iter = m.split_whitespace().map(|v| v.parse::<u64>().unwrap());
            acc.insert(Map(map_iter.next().unwrap(), map_iter.next().unwrap(), map_iter.next().unwrap()));
            acc
        })
    }).collect::<Vec<BTreeSet<Map>>>();

    let ans = seeds.into_iter().map(|seed| {
        mapping.iter().fold(seed, |source, m| {
           match m.into_iter().find(|&ele| source >= ele.1 && source < ele.1 + ele.2) {
                Some(ele) => source - ele.1 + ele.0,
                None => source,
            }
        })
    }).min().unwrap();
    println!("ANS: {0}", ans);
}
