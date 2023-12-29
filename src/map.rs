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
    pub fn get_pipe_coordinates(&self) -> Vec<(usize, usize)> {
        match self.start {
            None => vec!{},
            Some(start) => {
                // get starting points
                let mut next_coordinate = self.check_north(start);
                if next_coordinate.is_none() {
                    next_coordinate = self.check_east(start);
                }
                if next_coordinate.is_none() {
                    next_coordinate = self.check_south(start);
                }
                assert!(next_coordinate.is_some(), "Loop not present in input.");

                let mut path: Vec<(usize, usize)> = vec!{ start };
                let mut previous_coordinate = start;
                let nc = next_coordinate.unwrap();
                let mut current_coordinate = nc.0;
                let mut current_char = nc.1;

                while 'S' != current_char {
                    println!("previous coordinate: {1:?}, current coordinate: {0:?}, current_char: {2}", previous_coordinate, current_coordinate, current_char);
                    path.push(current_coordinate);
                    let dir = match Pipe::new(current_char).unwrap() {
                        Pipe::Horizontal => match previous_coordinate.1 + 1 == current_coordinate.1 {
                            true => Direction::East,
                            false => Direction::West,
                        },
                        Pipe::Vertical => match previous_coordinate.0 + 1 == current_coordinate.0 {
                            true => Direction::South,
                            false => Direction::North,
                        },
                        Pipe::Seven => match previous_coordinate.1 + 1 == current_coordinate.1 {
                            true => Direction::South,
                            false => Direction::West,
                        },
                        Pipe::J => match previous_coordinate.1 + 1 == current_coordinate.1 {
                            true => Direction::North,
                            false => Direction::West,
                        },
                        Pipe::L => match previous_coordinate.0 + 1 == current_coordinate.0 {
                            true => Direction::East,
                            false => Direction::North,
                        },
                        Pipe::F => match current_coordinate.1 + 1 == previous_coordinate.1 {
                            true => Direction::South,
                            false => Direction::East,
                        },
                    };
                    current_char = self.next(dir, &mut previous_coordinate, &mut current_coordinate);
                }
                path
            }
        }
    }


    fn next(&self, dir: Direction, previous_coordinate: &mut (usize, usize), current_coordinate: &mut (usize, usize)) -> char {
        let info = match dir {
            Direction::North => self.check_north(current_coordinate.clone()),
            Direction::East => self.check_east(current_coordinate.clone()),
            Direction::South => self.check_south(current_coordinate.clone()),
            Direction::West => self.check_west(current_coordinate.clone()),
        }.unwrap(); 
        (*previous_coordinate, *current_coordinate) = (*current_coordinate, info.0);
        info.1
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

    pub fn get_enclosed_area(&self) {
       let pipe = self.get_pipe_coordinates(); 
       println!("Coordinates: {0:?}", pipe);
    }
}

struct Info((usize, usize), char);

enum Direction {
    North,
    East,
    West,
    South,
}

enum Pipe {
    Horizontal,
    Vertical,
    Seven,
    F,
    J,
    L,
}

impl Pipe {
    fn new(c: char) -> Option<Self> {
        match c {
            '7' => Some(Self::Seven),
            'J' => Some(Self::J),
            'F' => Some(Self::F),
            'L' => Some(Self::L),
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            _ => None,
        }
    }
} 