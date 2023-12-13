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

    seeds_range.chunks(2).for_each(|nt| {
        println!("Checking for: ({0}, {1})", nt[0], nt[1]);
        println!("{0:?}",develop((nt[0],nt[1]), mapping.get(0).unwrap()));
        println!("")
    });
    // let ans = seeds_range.chunks(2).fold(u64::MAX,|mut acc, sr| {
    //     let seed = sr[0];
    //     let range = sr[1];
    //     for s in seed..seed + range {
    //         acc = acc.min(mapping.iter().fold(s, |source, m| {
    //             match m.into_iter().find(|&ele| source >= ele.1 && source < ele.1 + ele.2) {
    //                 Some(ele) => source - ele.1 + ele.0,
    //                 None => source,
    //             }
    //         }))
    //     }
    //     acc
    // });
    // println!("ANS: {0}", ans);
}

fn develop(nt: (u64, u64), bt: &BTreeSet<Map>) -> Vec<(u64, u64)> {
    bt.into_iter().filter(|&ele| !(nt.0 + nt.1 < ele.1 && nt.0 >= ele.1 + ele.2)).fold(vec!{(nt)}, |acc, m| {
        acc.iter().fold(Vec::new(), |mut b_acc, n| {
            if n.0 < m.1 && n.0 + n.1 < m.1 + m.2 {
                b_acc.push((n.0, m.1));
                b_acc.push((m.0, n.0 + n.1 - m.1 + m.0));
            }
            else if n.0 > m.1 && n.0 + n.1 > m.1 + m.2 {
                b_acc.push((n.0 - m.1 + m.0, m.0 + m.2));
                b_acc.push((m.0 + m.2, n.0 + n.1));
            }
            else if n.0 >= m.1 && n.0 + n.1 <= m.1 + m.2 {
                b_acc.push((m.0, n.0 - m.1 + m.0));
                b_acc.push((n.0 - m.1 + m.0, n.0 + n.1 - m.1 + m.0));
                b_acc.push((n.0 + n.1 - m.1 + m.0, m.0 + m.2));
            }
            else if n.0 < m.1 && n.0 + n.1 > m.1 + m.2 {
                b_acc.push((n.0, m.1));
                b_acc.push((m.0, m.0 + m.2));
                b_acc.push((m.1 + m.2, n.0 + n.1));
            }
            else {
                b_acc.push((n.0, n.1));
            }
            b_acc
        })
    });
    match bt.into_iter().find(|&ele| nt.0 >= ele.1 && nt.0 < ele.1 + ele.2) {
        Some(m) => {
            println!("found {0:?}", m);
            match nt.0 + nt.1 <= m.1 + m.2 {
                true => vec!{(m.0 - m.1 + nt.0, nt.1)},
                false => {
                    let mut ans_vec = vec!{(m.0 - m.1 + nt.0, m.1 + m.2 - nt.0)};
                    ans_vec.append(&mut develop((m.1 + m.2 - nt.0, 2 * nt.0 + nt.1 - m.1 - m.2), bt));
                    ans_vec
                }
            }
        },
        None => vec!{nt},
    }
}
