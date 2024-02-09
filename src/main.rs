use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let stack = Stack::new_from_str(&content);

    // if stack.assert_mono_width() {
    //     println!("All have width of 1");
    // }
    // else {
    //     println!("Some have width of 2");
    // }
    println!("stack: {stack:#?}");
}

#[derive(Debug)]
struct Stack (Vec<Brick>);

impl Stack {
    fn new_from_str(content: &str) -> Self {
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        let stack = content.lines().fold(Vec::new(), |mut acc: Vec<Brick>, line| {
            let mut brick = Brick::new_from_str(line);
            // determine the stabilized position of brick 
            let coordinates = brick.get_contained_xy_coordinates();
            // println!("coordinates for {brick:?} are {coordinates:?}");
            let z = coordinates.iter().map(|coor| cache.get(coor).copied().unwrap_or(1)).max().unwrap_or_else(|| panic!("error finding max."));
            // set brick values
            println!("max z coor: {z}");
            brick.set_z_coordinate(z);
            // add to cache
            coordinates.into_iter().for_each(|coor| { cache.insert(coor, z); });
            acc.push(brick);
            acc
        });
        Self(stack)
    }

    fn assert_mono_width(&self) -> bool {
        self.0.iter().fold(true, |acc , brick| {
            let count =  [brick.0.0 as isize - brick.1.0 as isize, brick.0.1 as isize - brick.1.1 as isize, brick.0.2 as isize - brick.1.2 as isize].into_iter().filter(|&e| e != 0).count();
            let b = match count {
                0 => {
                    println!("all matched for brick: {brick:?}");
                    false
                }
                1 => true,
                _ => {
                    println!("two did not match for brick: {brick:?}");
                    false
                }
            };
            acc && b
        })
    }
}

#[derive(Debug)]
struct Brick(Coordinate, Coordinate);

impl Brick {
    fn new_from_str(brick: &str) -> Self {
        let mut brick_iter = brick.split('~').map(|b| Coordinate::new_from_str(b));
        Self(brick_iter.next().unwrap_or_else(|| panic!("error reading brick.")), brick_iter.next().unwrap_or_else(|| panic!("error reading brick.")))
    }

    fn get_contained_xy_coordinates(&self) -> Vec<(usize, usize)> {
        (self.0.0..=self.1.0).flat_map(|x| (self.0.1..=self.1.1).map(move |y| (x, y))).collect::<Vec<(usize, usize)>>()
    }

    fn get_top_point(&self) -> usize {
        self.0.2.max(self.1.2)
    }

    fn set_z_coordinate(&mut self, z: usize) {
        let min = self.0.2.min(self.1.2);
        if self.0.2 == min {
            self.0.2 = z;
        }
        if self.1.2 == min {
            self.0.2 = z;
        }
    }
}

#[derive(Debug)]
struct Coordinate(usize, usize, usize);

impl Coordinate {
    fn new_from_str(coor: &str) -> Self {
        let mut num_iter = coor.split(',').map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("error parsing coordinate: {}", coor)));
        Self(num_iter.next().unwrap_or_else(|| panic!("error getting 1st element in {}", coor)), num_iter.next().unwrap_or_else(|| panic!("error getting 2nd element in {}", coor)), num_iter.next().unwrap_or_else(|| panic!("error getting 3rd element in {}", coor)))
    }
}