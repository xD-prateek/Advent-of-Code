use std::{collections::{HashSet, VecDeque}, mem::swap, usize};

pub struct Garden {
    rocks: Vec<(isize, isize)>,
    width: isize,
    height: isize,
    start: (isize, isize),
}

impl Garden {
    pub fn new_from_string(content: &str) -> Self {
        let height = content.lines().count() as isize;
        let width = content.lines().next().unwrap().len() as isize;
        let mut start = None;
        let rocks = content.lines().enumerate().fold(Vec::new(), |mut acc, (x, line)| {
            let mut new_rocks = line.chars().enumerate().filter_map(|(y, ch)| {
                match ch {
                    '#' => Some((x as isize, y as isize)),
                    'S' => {
                        start = Some((x as isize, y as isize));
                        None
                    }, 
                    _ => None,
                }
            }).collect::<Vec<(isize, isize)>>();
            acc.append(&mut new_rocks);
            acc
        });

        if let Some(start) = start {
            Self {
                rocks,
                width,
                height,
                start,
            }
        }
        else {
            panic!("no start value found.");
        }
    }

    pub fn get_garden_plots(u1: usize, u2: usize, u3: usize, n: usize) -> usize {
        let a = u1  / 2;
        let b = u2 - 3 * a;
        let c = u3 - a - b;
        a * n.pow(2) + b * n + c
    }

    pub fn get_potential_garden_plots(&self, steps: usize) -> usize {
    	let mut visited = HashSet::new();
    	let mut seen = HashSet::new();

    	let mut q = VecDeque::from([(self.start, steps)]);
        let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];

        let mut count = steps.rem_euclid(self.height as usize);
        let step = self.height as usize * 2;
        let mut prev_visited_len = 0;
        let mut prev_prev_visited_len = 0;
        let mut prev_second_diff = 0;
        println!("count: {count}");
    	while let Some(((x, y), reverse_steps)) = q.pop_front() {
    		if reverse_steps & 1 == 0 {
    			visited.insert((x, y));
    		}

            if steps == count + reverse_steps + 1 {
                let new_prev_second_diff = visited.len() - prev_visited_len - prev_prev_visited_len;
                if new_prev_second_diff != prev_second_diff {
                    prev_second_diff = new_prev_second_diff;
                    prev_prev_visited_len += prev_second_diff;
                    prev_visited_len += prev_prev_visited_len;
                    count += step;
                }
                else {
                    return Self::get_garden_plots(new_prev_second_diff, prev_prev_visited_len, prev_visited_len - prev_prev_visited_len, (steps - count) / self.height as usize / 2 + 3);
                }
            }

    		if reverse_steps > 0 {
    			dirs.iter().map(|(del_x, del_y)| (x + del_x, y + del_y)).for_each(|coor| {
    				if !self.contains_rock(&coor) && !seen.contains(&coor) {
    					q.push_back((coor, reverse_steps - 1));
    					seen.insert(coor);
    				}
    			});
    		}
    	}
        0
    }

    fn contains_rock(&self, (x, y): &(isize, isize)) -> bool {
        self.rocks.contains(&(x.rem_euclid(self.height), y.rem_euclid(self.width)))
    }
}
