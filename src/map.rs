pub struct Map {
    map: Vec<Vec<char>>,
    start: Option<(usize, usize)>,
    y_limits: (usize, usize),
    x_limits: (usize, usize),
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
        match start {
            Some(s) => Self {
                map,
                start,
                y_limits:(s.0, s.0),
                x_limits: (s.1, s.1),
            },
            None => {
             let y_limits = (0, map.len());
             let x_limits = (0, map.get(0).unwrap().len());
             Self {
                map,
                start,
                y_limits,
                x_limits,
            }
        }
    }
}
}

impl Map {
    fn get_pipe_coordinates(&mut self) -> Vec<Info> {
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

                let mut previous_coordinate = start;
                let nc = next_coordinate.unwrap();
                let mut current_coordinate = nc.0;
                let mut current_char = nc.1;
                let mut path = vec!{ Info(start, 'S') };

                while 'S' != current_char {
                    self.y_limits = (current_coordinate.0.min(self.y_limits.0), current_coordinate.0.max(self.y_limits.1));
                    self.x_limits = (current_coordinate.1.min(self.x_limits.0), current_coordinate.1.max(self.x_limits.1));
                    path.push(Info(current_coordinate, current_char));
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

    pub fn get_enclosed_area(&mut self) -> u32 {
       let pipe = self.get_pipe_coordinates(); 

       let no_of_enclosed_points = (self.y_limits.0 + 1..self.y_limits.1).flat_map(|y| (self.x_limits.0 + 1..self.x_limits.1).map(move |x| (y, x))).filter(|&ele| pipe.iter().find(|info| info.0 == ele).is_none()).fold(0u32, |acc, (y, x)| {
        let mut passes = pipe.iter().filter(|info| x == info.0.1 && y > info.0.0 && '|' != info.1).peekable();
        let mut count = 0u32;
        while let Some(info) = passes.next() {
            match info.1 {
                '-' => count += 1,
                _ => count += match (info.1, passes.next().map(|info| info.1)) {
                    ('J', Some('F')) | ('7', Some('L')) | ('F', Some('J')) | ('L', Some('7')) => 1,
                    _ => 0,
                },
            }
        }
        acc + (count & 1)
    });
       no_of_enclosed_points
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