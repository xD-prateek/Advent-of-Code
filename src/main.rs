use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let expansion_rate = 1000000;
    let mut galaxies = content.lines().enumerate().flat_map(|(y, line)| line.char_indices().filter(|&(_, c)| c == '#').map(move |(x, _)| (y as u64, x as u64))).collect::<Vec<(u64, u64)>>();

    let empty_y = (0..galaxies.iter().map(|&(y, _)| y).max().unwrap_or_default()).filter(|num| galaxies.iter().all(|(y, _)| y != num)).collect::<Vec<u64>>();
    let empty_x = (0..galaxies.iter().map(|&(_, x)| x).max().unwrap_or_default()).filter(|num| galaxies.iter().all(|(_, x)| x != num)).collect::<Vec<u64>>();

    galaxies.iter_mut().for_each(|(y, x)| {
     *y += empty_y.iter().filter(|&line| line < y).count() as u64 * (expansion_rate - 1);
     *x += empty_x.iter().filter(|&pos| pos < x).count() as u64 * (expansion_rate - 1);    
 });

    let ans = galaxies.iter().enumerate().flat_map(|(i, &g1)| galaxies.iter().skip(i + 1).map(move |&g2| (g1, g2))).fold(0u64, |acc, (g1, g2)| acc + g2.0.abs_diff(g1.0) + g2.1.abs_diff(g1.1));
    println!("ANS: {0}", ans);

}
