use std::collections::{HashMap, HashSet, VecDeque};

pub struct Stack (Vec<Brick>);

impl Stack {
    pub fn new_from_str(content: &str) -> Self {
        let mut stack = content.lines().map(Brick::new_from_str).collect::<Vec<Brick>>();
        stack.sort_by_key(|brick| brick.0.2);
        stack.iter_mut().fold(HashMap::new(), |mut cache: HashMap<(usize, usize), usize>, brick| {
            let coordinates = brick.get_contained_xy_coordinates();
            let z = coordinates.iter().map(|coor| cache.get(coor).copied().unwrap_or(0)).max().unwrap_or_else(|| panic!("error finding max.")) + 1;
            brick.set_z_coordinate(z);
            coordinates.into_iter().for_each(|coor| { cache.insert(coor, brick.1.2); });
            cache
        });

        Self(stack)
    }

    pub fn get_bricks_safe_to_disintegrate(&self) -> usize {
        let mut supports = vec!{ Vec::new(); self.0.len() };
        let mut supported_by = vec!{ Vec::new(); self.0.len() };
        self.0.iter().enumerate().for_each(|(i, lower_brick)| {
            self.0.iter().enumerate().skip(i).filter(|(_, upper_brick)| upper_brick.overlaps(lower_brick) && upper_brick.0.2 == lower_brick.1.2 + 1).for_each(|(j, _)| {
                supports.get_mut(i).unwrap_or_else(|| panic!("index out of bounds. {j}")).push(j);
                supported_by.get_mut(j).unwrap_or_else(|| panic!("index out of bounds. {j}")).push(i);
            });
        });

        (0..self.0.len()).fold(0, |acc, i| {
            let mut q = VecDeque::from_iter(supports.get(i).unwrap_or_else(|| panic!("index out of bounds. {i}")).iter().filter(|&&k| supported_by.get(k).unwrap_or_else(|| panic!("index out of bounds. {k}")).len() == 1).copied());
            let mut falling = q.iter().copied().collect::<HashSet<usize>>();
            falling.insert(i);
            while let Some(j) = q.pop_front() {
                supports.get(j).unwrap_or_else(|| panic!("index out of bounds. {j}")).iter().for_each(|&k| {
                    if !falling.contains(&k) && supported_by.get(k).unwrap_or_else(|| panic!("index out of bounds. {k}")).iter().all(|l| falling.contains(l)) {
                        falling.insert(k);
                        q.push_back(k);
                    }
                });
            }
            acc + falling.len() - 1
        })
    }
}

struct Brick(Coordinate, Coordinate);

impl Brick {
    fn new_from_str(brick: &str) -> Self {
        let mut coordinate_iter = brick.split('~').map(Coordinate::new_from_str);
        let first_coor = coordinate_iter.next().unwrap_or_else(|| panic!("error reading brick."));
        let second_coor = coordinate_iter.next().unwrap_or_else(|| panic!("error reading brick."));
        assert!(first_coor.2 <= second_coor.2);
        Self(first_coor, second_coor)
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.0.0.max(other.0.0) <= self.1.0.min(other.1.0) && self.0.1.max(other.0.1) <= self.1.1.min(other.1.1)
    }

    fn get_contained_xy_coordinates(&self) -> Vec<(usize, usize)> {
        (self.0.0..=self.1.0).flat_map(|x| (self.0.1..=self.1.1).map(move |y| (x, y))).collect::<Vec<(usize, usize)>>()
    }

    fn set_z_coordinate(&mut self, z: usize) {
        let (lower, upper) = match self.0.2 < self.1.2 {
            true => (&mut self.0.2, &mut self.1.2),
            false => (&mut self.1.2, &mut self.0.2),
        };
        *upper = *upper + z - *lower;
        *lower = z;
    }
}

struct Coordinate(usize, usize, usize);

impl Coordinate {
    fn new_from_str(coor: &str) -> Self {
        let mut num_iter = coor.split(',').map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("error parsing coordinate: {}", coor)));
        Self(num_iter.next().unwrap_or_else(|| panic!("error getting 1st element in {}", coor)), num_iter.next().unwrap_or_else(|| panic!("error getting 2nd element in {}", coor)), num_iter.next().unwrap_or_else(|| panic!("error getting 3rd element in {}", coor)))
    }
}
