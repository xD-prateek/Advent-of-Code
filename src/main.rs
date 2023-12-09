use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug, Clone)]
struct Num {
    val: u32,
    start_end: (u32, u32),
}

fn fetch_numbers_with_position(data: &str) -> HashMap<u32, Vec<Num>> {
    data.lines().enumerate().flat_map(|(i, line)| {
        let mut val = 0u32;
        let mut start = 0u32;

        let iter = line.chars().enumerate().peekable();
        iter.filter_map(move |(j, c)| {
            if let Some(num) = c.to_digit(10) {
                if val == 0 {
                    start = j as u32;
                }
                val  = val * 10 + num;
                if j == line.len() - 1 && val != 0 {
                    let r = Some((i as u32, Num {
                        val,
                        start_end: (start, j as u32),
                    }));
                    val = 0;
                    r
                }
                else {
                    None
                }
            }
            else if val != 0 {
                let r = Some((i as u32, Num {
                    val,
                    start_end: (start, j as u32),
                }));
                val = 0;
                r
            }
            else {
                None
            }
        })
    }).fold(HashMap::new(), |mut acc, (i, num)| {
            acc.entry(i).and_modify(|v| v.push(num.clone())).or_insert(vec!{num});
            acc
        })
}

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let mut ans = 0u32;
    let mut nums = fetch_numbers_with_position(&content);
    // println!("{0}", content);
    // println!("{0:#?}", nums);

    let del_move = vec!{-1i32, 0, 1};
    let movement = del_move.iter().flat_map(|&x| del_move.iter().filter_map(move |&y| {
        if x == 0 && y == 0 { None }
        else { Some((x, y)) }
    })).collect::<Vec<(i32, i32)>>();

    content.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().filter(|&(_, c)| c != '.' && !c.is_digit(10)).for_each(|(j, _c)| {
            // println!("found {0} at ({1}, {2})", _c, i, j);
            movement.iter().filter_map(|(x, y)| {
                let v = i as i32 + x;
                let h = j as i32 + y;
                if v < 0 || h < 0 {
                    None
                }
                else {
                    Some((v as u32, h as u32))
                }
            }).filter(|&(x, y)| content.lines().nth(x as usize).and_then(|l| l.chars().nth(y as usize)).unwrap().is_digit(10)).for_each(|(x, y)| {
                    // println!("Checking ({1}, {2}) for {0}", c, x, y);
                    if let Some(v) = nums.get_mut(&x) {
                        v.retain(|num| {
                            if num.start_end.0 <= y && num.start_end.1 >= y {
                                ans += num.val;
                                // println!("added {0} to ans. Ans = {1}", num.val, ans);
                                false
                            }
                            else {
                                true
                            }
                        });
                        if v.is_empty() { nums.remove(&x); }
                    }
                });
        })
    });
    println!("{0}", ans);
}
