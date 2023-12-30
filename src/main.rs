mod map;
use map::Map;
use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();
    // let ans = Map::from(content).get_enclosed_area();
   let ans = Map::from(content).get_enclosed_area();
   println!("ANS: {0}", ans);
}
