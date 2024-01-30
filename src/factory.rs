use std::{collections::{BinaryHeap, HashSet}, rc::Rc};

pub struct Factory {
    map: Vec<Vec<u32>>,
    consecutive_step_count: u32,
}

impl Factory {
    pub fn new_from_string(content: String, consecutive_step_count: u32) -> Self {
        Self {
            map: content.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or_else(|| panic!("Error parsing input"))).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>(),
            consecutive_step_count,
        }
    }

    pub fn find_min_heat_loss(&self) -> u32 {
        let initial_node = Rc::new(Node::new_from(0, (0, 0), (0, 0), 0));

        let mut pq = BinaryHeap::from([initial_node]);

        let end_coordinate = (self.map.len() as isize - 1, self.map.get(0).unwrap().len() as isize - 1);

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut seen_nodes = HashSet::new();

        while let Some(node) = pq.pop() {
            if node.coordinate == end_coordinate {
                return node.heat;
            }

            let new_node_metadata = node.get_metadata();
            if !seen_nodes.contains(&new_node_metadata) {
                seen_nodes.insert(new_node_metadata);

                directions.iter().filter_map(|&del_coor| {
                    match del_coor == node.del_coordinate {
                        false => match del_coor == (-node.del_coordinate.0, -node.del_coordinate.1) {
                            false => Some((del_coor, 1)),
                            true => None,
                        },
                        true => match node.consecutive_steps < self.consecutive_step_count {
                            false => None,
                            true => Some((del_coor, node.consecutive_steps + 1)),
                        },
                    }
                }).for_each(|(del_coordinate, step)| {
                    let new_coordinate = (node.coordinate.0 + del_coordinate.0, node.coordinate.1 + del_coordinate.1);
                    if let Some(heat) = self.get_heat_loss_at_block(new_coordinate) {
                        let new_node = Node::new_from(heat + node.heat, new_coordinate, del_coordinate, step);
                        pq.push(Rc::new(new_node));
                    }
                });
            }
        }
        println!("Node not found.");
        0
    }

    fn get_heat_loss_at_block(&self, coordinate: (isize, isize)) -> Option<u32> {
        match coordinate.0 < 0 || coordinate.1 < 0 {
            true => None,
            false => self.map.get(coordinate.0 as usize).and_then(|line| line.get(coordinate.1 as usize)).copied(),
        }
    }
}

struct Node {
    heat: u32,
    coordinate: (isize, isize),
    del_coordinate: (isize, isize),
    consecutive_steps: u32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate && self.del_coordinate == other.del_coordinate && self.heat == other.heat
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       other.heat.cmp(&self.heat)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl Node {
    fn new_from(heat: u32, coordinate: (isize, isize), del_coordinate: (isize, isize), consecutive_steps: u32) -> Self {
        Self {
            heat,
            coordinate,
            del_coordinate,
            consecutive_steps,
        }
    }

    fn get_metadata(&self) -> ((isize, isize), (isize, isize), u32) {
        (self.coordinate, self.del_coordinate, self.consecutive_steps)
    }
}