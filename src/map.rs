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
                if let Some(ch) = self.check_north(start) {
                    next_coordinates.push(((start.0 - 1, start.1), ch));
                }
                if let Some(ch) = self.check_south(start) {
                    next_coordinates.push(((start.0 + 1, start.1), ch));
                }
                if let Some(ch) = self.check_east(start) {
                    next_coordinates.push(((start.0, start.1 + 1), ch));
                }
                if let Some(ch) = self.check_west(start) {
                    next_coordinates.push(((start.0, start.1 - 1), ch));
                }
                let mut ans = 0u32;
                let mut next_coordinates_iter = next_coordinates.into_iter();
                let mut take_next_path = true;
                while take_next_path {
                    if let Some(point) = next_coordinates_iter.next() {
                        match self.max_distance(start, point.0, point.1) {
                            Path::Closed(d) => {
                                println!("Found it to be closed with length: {0}", d);
                                ans = ans.max((d + 1) / 2);
                                take_next_path = false;
                            },
                            Path::Open(d) => {
                                println!("Found it to be open with length: {0}", d);
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

    fn max_distance(&self, previous: (usize, usize), current: (usize, usize), current_char: char) -> Path {
        println!("Previous: {0:?}, Current: {1:?}, Current char: {2}", previous, current, current_char);
        match current_char {
            '-' => match previous.1 + 1 == current.1 {
                true => self.go_east(current),
                false => self.go_west(current),
            },
            '|' => match previous.0 + 1 == current.0 {
                true => self.go_south(current),
                false => self.go_north(current),
            },
            '7' => match previous.0 + 1 == current.0 {
                true => self.go_west(current),
                false => self.go_south(current),
            },
            'J' => match previous.1 + 1 == current.1 {
                true => self.go_north(current),
                false => self.go_west(current),
            },
            'L' => match previous.0 + 1 == current.0 {
                true => self.go_east(current),
                false => self.go_north(current),
            },
            'F' => match current.0 + 1 == previous.0 {
                true => self.go_east(current),
                false => self.go_south(current),
            },
            'S' => Path::Closed(0),
            _ => Path::Open(0), // some different character
        }
    }

    fn go_north(&self, current: (usize, usize)) -> Path {
        match self.check_north(current) {
            Some(ch) => match self.max_distance(current, (current.0 - 1, current.1), ch) {
                Path::Closed(val) => Path::Closed(val + 1),
                Path::Open(val) => Path::Open(val + 1),
            },
            None => Path::Open(1), // path does not exist
        }
    }

    fn go_east(&self, current: (usize, usize)) -> Path {
        match self.check_east(current) {
            Some(ch) => match self.max_distance(current, (current.0, current.1 + 1), ch) {
                Path::Closed(val) => Path::Closed(val + 1),
                Path::Open(val) => Path::Open(val + 1),
            },
            None => Path::Open(1),
        }
    }

    fn go_south(&self, current: (usize, usize)) -> Path {
        match self.check_south(current) {
            Some(ch) => match self.max_distance(current, (current.0 + 1, current.1), ch) {
                Path::Closed(val) => Path::Closed(val + 1),
                Path::Open(val) => Path::Open(val + 1),
            }
            None => Path::Open(1),
        }
    }

    fn go_west(&self, current: (usize, usize)) -> Path {
        match self.check_west(current) {
            Some(ch) => match self.max_distance(current, (current.0, current.1 - 1), ch) {
                Path::Closed(val) => Path::Closed(val + 1),
                Path::Open(val) => Path::Open(val + 1),
            },
            None => Path::Open(1),
        }
    }

    fn check_north(&self, coordinate: (usize, usize)) -> Option<char> {
        match coordinate {
            (0, _) => None,
            _ => {
                match self.map.get(coordinate.0 - 1).unwrap().get(coordinate.1).unwrap() {
                    &ch@ ('7' | '|' | 'F' | 'S') => Some(ch),
                    _ => None,
                }
            },
        }
    }

    fn check_south(&self, coordinate: (usize, usize)) -> Option<char> {
        match self.map.get(coordinate.0 + 1) {
            None => None,
            Some(line) => {
                match line.get(coordinate.1).unwrap() {
                    &ch@ ('J' | '|' | 'L' | 'S') => Some(ch),
                    _ => None,
                }
            }
        }
    }

    fn check_east(&self, coordinate: (usize, usize)) -> Option<char> {
        match self.map.get(coordinate.0).unwrap().get(coordinate.1 + 1) {
            None => None,
            Some(ch) => match ch {
                &ch@ ('J' | '-' | '7' | 'S') => Some(ch),
                _ => None,
            }
        }
    }

    fn check_west(&self, coordinate: (usize, usize)) -> Option<char> {
        match coordinate {
            (_, 0) => None,
            _ => {
                match self.map.get(coordinate.0).unwrap().get(coordinate.1 - 1).unwrap() {
                    &ch@ ('L' | '-' | 'F' | 'S') => Some(ch),
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
