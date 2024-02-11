use std::collections::{HashMap, HashSet};

type Graph = HashMap<(isize, isize), Vec<((isize, isize), usize)>>;

pub struct SnowIsland {
    map: Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),
}

impl SnowIsland {
    pub fn new_from_str(content: &str) -> Self {
        let map = content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let start = (0, map.first().unwrap_or_else(|| panic!("no row present.")).iter().position(|&c| c == '.').unwrap_or_else(|| panic!("cannot identify start position.")) as isize);
        let end = (map.len() as isize - 1, map.last().unwrap_or_else(|| panic!("no row present.")).iter().position(|&c| c == '.').unwrap_or_else(|| panic!("cannot identify ending position.")) as isize);

        Self {
            map,
            start,
            end,
        }

    }

    pub fn get_longest_hike(&self) -> usize {
        let poi = {
            let mut poi = self.map.iter().enumerate().map(|(x, c)| (x as isize, c)).flat_map(|(x, line)| line.iter().enumerate().filter(|(_, &c)| c == '.').map(|(y, c)| (y as isize, c)).filter(move |(y, _)| [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter().filter_map(|(del_x, del_y)| self.get_char_at((x + del_x, y + del_y))).filter(|&ch| ch != '#').count() >= 3).map(move |(y, _)| (x, y))).collect::<Vec<(isize, isize)>>();
            poi.append(&mut vec!{ self.start, self.end });
            poi
        };
        // let directions = HashMap::from([('^', vec!{ (-1, 0) }), ('>', vec!{ (0, 1) }), ('v', vec!{ (1, 0) }), ('<', vec!{ (0, -1) }), ('.', vec!{ (-1, 0), (0, 1), (1, 0), (0, -1) })]);
        let directions = [ (-1, 0), (0, 1), (1, 0), (0, -1) ];
        let adjacency_graph = poi.iter().fold(HashMap::new(), |mut acc: Graph, &point| {
            let mut q = vec!{ (point, 0) };
            let mut visited = vec!{ point };
            while let Some((coor, dist)) = q.pop() {
                if dist != 0 && poi.contains(&coor) {
                    acc.entry(point).and_modify(|list| list.push((coor, dist))).or_insert(vec!{ (coor, dist) });
                }
                else {
                    directions.iter().map(|(del_x, del_y)| (coor.0 + del_x, coor.1 + del_y)).for_each(|new_coor| {
                        if let Some(ch) = self.get_char_at(new_coor) {
                            if ch != '#' && !visited.contains(&new_coor) {
                                q.push((new_coor, dist + 1));
                                visited.push(new_coor);
                            }
                        }
                    });
                }
            }
            acc
        });
        self.get_max_distance(self.start, &adjacency_graph, &mut HashSet::new())
    }

    fn get_max_distance(&self, coor: (isize, isize), graph: &Graph, visited: &mut HashSet<(isize, isize)>) -> usize {
        match coor == self.end {
            true => 0,
            false => { 
            	visited.insert(coor);
            	let max_len = graph.get(&coor).unwrap_or(&Vec::new()).iter().fold(0, |acc, &(next_coor, dist)| {
            		match visited.contains(&next_coor) {
            			true => acc,
            			false => acc.max(dist + self.get_max_distance(next_coor, graph, visited)),
            		}
            	});
            	visited.remove(&coor);
            	max_len
            },
        }
    }

    fn get_char_at(&self, (x, y): (isize, isize)) -> Option<char> {
        match x < 0 || y < 0 {
            true => None,
            false => self.map.get(x as usize).and_then(|line| line.get(y as usize)).copied(),
        }
    }
}