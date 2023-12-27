pub struct Map {
    map: Vec<Vec<char>>,
    start: Option<(usize, usize)>,
}

impl From<String> for Map {
    fn from(value: String) -> Self {
        let mut start = None;
        let map = value.lines().enumerate().map(|(i, line)| line.chars().enumerate().map(|(j, ch)| {
            if ch == 'S' {
                start = Some((i, j));
            }
            ch
        }).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        Self {
            map,
            start,
        }
    }
}

impl Map {
    pub fn get_farthest_distance(&self) -> u32 {
        match self.start {
            None => 0,
            Some(start) => {
                // get starting points
                let mut next_coordinates = Vec::new();
                if let Some(info) = self.check_north(start) {
                    next_coordinates.push(info);
                }
                if let Some(info) = self.check_south(start) {
                    next_coordinates.push(info);
                }
                if let Some(info) = self.check_east(start) {
                    next_coordinates.push(info);
                }
                if let Some(info) = self.check_west(start) {
                    next_coordinates.push(info);
                }
                let mut ans = 0u32;
                let mut next_coordinates_iter = next_coordinates.into_iter();
                let mut take_next_path = true;
                while take_next_path {
                    if let Some(point) = next_coordinates_iter.next() {
                        match self.max_distance(point.0, point.1) {
                            Path::Closed(d) => {
                                ans = ans.max((d + 1) / 2);
                                take_next_path = false;
                            },
                            Path::Open(d) => {
                                ans = ans.max(d);
                                take_next_path = true;
                            },
                        }
                    }
                    else {
                        break;
                    }
                }
                ans
            }, 
        }
    }

    fn max_distance(&self, mut current_coordinate: (usize, usize), mut current_char: char) -> Path {
        if let Some(mut previous_coordinate) = self.start {
            let mut max_dist = 1u32;
            loop {
                // println!("current_coordinate: {0:?}, current_char: {1}", current_coordinate, current_char);
                // process current char
                let dir = match current_char {
                    '-' => match previous_coordinate.1 + 1 == current_coordinate.1 {
                        true => Some(Direction::East),
                        false => Some(Direction::West),
                    },
                    '|' => match previous_coordinate.0 + 1 == current_coordinate.0 {
                        true => Some(Direction::South),
                        false => Some(Direction::North),
                    },
                    '7' => match previous_coordinate.1 + 1 == current_coordinate.1 {
                        true => Some(Direction::South),
                        false => Some(Direction::West),
                    },
                    'J' => match previous_coordinate.1 + 1 == current_coordinate.1 {
                        true => Some(Direction::North),
                        false => Some(Direction::West),
                    },
                    'L' => match previous_coordinate.0 + 1 == current_coordinate.0 {
                        true => Some(Direction::East),
                        false => Some(Direction::North),
                    },
                    'F' => match current_coordinate.1 + 1 == previous_coordinate.1 {
                        true => Some(Direction::South),
                        false => Some(Direction::East),
                    },
                _ =>  None, // some different character
            };

            if let Some(dir) = dir {
                match self.next(dir, &mut previous_coordinate, &mut current_coordinate) {
                    Some(ch) => {
                        if ch == 'S' {
                            return Path::Closed(max_dist);
                        }
                        current_char = ch;
                        max_dist += 1;
                    },
                    None => return Path::Open(max_dist), // next is out of bounds or not connected to pipe
                }
            }
            else {
                return Path::Open(max_dist); // some other character
            }
        }  
    }
    else {
        Path::Closed(0)
    }
}

fn next(&self, dir: Direction, previous_coordinate: &mut (usize, usize), current_coordinate: &mut (usize, usize)) -> Option<char> {
    if let Some(info) = match dir {
        Direction::North => self.check_north(current_coordinate.clone()),
        Direction::East => self.check_east(current_coordinate.clone()),
        Direction::South => self.check_south(current_coordinate.clone()),
        Direction::West => self.check_west(current_coordinate.clone()),
    } {
        (*previous_coordinate, *current_coordinate) = (*current_coordinate, info.0);
        Some(info.1)
    }
    else {
        None
    }
}


fn check_north(&self, mut coordinate: (usize, usize)) -> Option<Info> {
    match coordinate {
        (0, _) => None,
        _ => {
            coordinate.0 -= 1;
            match self.map.get(coordinate.0).unwrap().get(coordinate.1).unwrap() {
                &ch@ ('7' | '|' | 'F' | 'S') => Some(Info(coordinate, ch)),
                _ => None,
            }
        },
    }
}

fn check_south(&self, mut coordinate: (usize, usize)) -> Option<Info> {
    coordinate.0 += 1;
    match self.map.get(coordinate.0) {
        None => None,
        Some(line) => {
            match line.get(coordinate.1).unwrap() {
                &ch@ ('J' | '|' | 'L' | 'S') => Some(Info(coordinate, ch)),
                _ => None,
            }
        }
    }
}

fn check_east(&self, mut coordinate: (usize, usize)) -> Option<Info> {
    coordinate.1 += 1;
    match self.map.get(coordinate.0).unwrap().get(coordinate.1) {
        None => None,
        Some(ch) => match ch {
            &ch@ ('J' | '-' | '7' | 'S') => Some(Info(coordinate, ch)),
            _ => None,
        }
    }
}

fn check_west(&self, mut coordinate: (usize, usize)) -> Option<Info> {
    match coordinate {
        (_, 0) => None,
        _ => {
            coordinate.1 -= 1;
            match self.map.get(coordinate.0).unwrap().get(coordinate.1).unwrap() {
                &ch@ ('L' | '-' | 'F' | 'S') => Some(Info(coordinate, ch)),
                _ => None,
            }
        },
    }
}
}

enum Path {
    Closed(u32),
    Open(u32),
}

struct Info((usize, usize), char);

enum Direction {
    North,
    East,
    West,
    South,
}