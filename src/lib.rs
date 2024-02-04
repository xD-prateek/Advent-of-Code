use std::collections::{HashSet, VecDeque};

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

    pub fn get_potential_garden_plots(&self, steps: usize) -> usize {
    	let mut visited = HashSet::new();
    	let mut seen = HashSet::new();

    	let mut q = VecDeque::from([(self.start, steps)]);
        let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    	while let Some(((x, y), steps)) = q.pop_front() {
    		if steps & 1 == 0 {
    			visited.insert((x, y));
    		}

    		if steps > 0 {
    			dirs.iter().map(|(del_x, del_y)| (x + del_x, y + del_y)).for_each(|coor| {
    				if coor.0 >= 0 && coor.1 >= 0 && coor.0 < self.height && coor.1 < self.width && !self.rocks.contains(&coor) && !seen.contains(&coor) {
    					q.push_back((coor, steps - 1));
    					seen.insert(coor);
    				}
    			});
    		}
    	}
        visited.len()
    }
}
