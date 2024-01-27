use std::{collections::HashSet, rc::Rc};

pub struct Contraption {
    map: Vec<Vec<char>>,
}

impl Contraption {
    pub fn new_from_string(content: String) -> Self {
        Self {
            map: content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>(),
        }
    }

    pub fn get_max_energized_tiles(&self) -> u32 {
    	let x = self.map.len() as i32;
    	let y = self.map.get(0).unwrap().len() as i32;

    	let mut ans = 0u32;

    	(0..x).for_each(|l| {
    		ans = ans.max(self.get_energized_tiles(Context::new_from((l, -1), (0, 1))));
    		ans = ans.max(self.get_energized_tiles(Context::new_from((l, y), (0, -1))));
    	});

    	(0..y).for_each(|e| {
    		ans = ans.max(self.get_energized_tiles(Context::new_from((-1, e), (1, 0))));
    		ans = ans.max(self.get_energized_tiles(Context::new_from((x, e), (-1, 0))));
    	});

    	ans
    }

    fn get_energized_tiles(&self, ctx: Context) -> u32 {
        let mut ray_list = vec!{ Rc::new(ctx) };
        let mut ray_cache = Vec::new();

        while !ray_list.is_empty() {
             let ctx = ray_list.pop().unwrap(); 
             let new_coordinate = (ctx.coordinate.0 + ctx.del_coordinate.0, ctx.coordinate.1 + ctx.del_coordinate.1); 

             if new_coordinate.0 < 0 || new_coordinate.1 < 0 {
                continue;
             };

             if let Some(c) = self.char_at(new_coordinate) {
                let mut del_new_coordinates = Vec::new();
                if c == '.' || (c == '-' && ctx.del_coordinate.1 != 0) || (c == '|' && ctx.del_coordinate.0 != 0) {
                    del_new_coordinates.push(ctx.del_coordinate);
                }
                else if c == '\\' {
                    del_new_coordinates.push((ctx.del_coordinate.1, ctx.del_coordinate.0));
                }
                else if c == '/' {
                    del_new_coordinates.push((-ctx.del_coordinate.1, -ctx.del_coordinate.0));
                }
                else {
                    match c {
                        '|' => {
                            del_new_coordinates.push((1, 0));
                            del_new_coordinates.push((-1, 0));
                        },
                        _ => {
                            del_new_coordinates.push((0, 1));
                            del_new_coordinates.push((0, -1));
                        },
                    }
                }

                del_new_coordinates.into_iter().for_each(|del_new_coordinate| {
                     let new_context = Rc::new(Context::new_from(new_coordinate, del_new_coordinate));
                     if !ray_cache.contains(&new_context) {
                        ray_list.push(Rc::clone(&new_context));
                        ray_cache.push(new_context);
                    }
                });
             }
         }

         ray_cache.into_iter().map(|ctx| ctx.coordinate).collect::<HashSet<(i32, i32)>>().len() as u32
    }

    fn char_at(&self, pos: (i32, i32)) -> Option<char> {
        self.map.get(pos.0 as usize).and_then(|line| line.get(pos.1 as usize)).copied()
    }
}

#[derive(PartialEq)]
struct Context {
    coordinate: (i32, i32),
    del_coordinate: (i32, i32),
}

impl Context {
    fn new_from(coordinate: (i32, i32), del_coordinate: (i32, i32)) -> Self {
        Self {
            coordinate,
            del_coordinate,
        }
    }
}