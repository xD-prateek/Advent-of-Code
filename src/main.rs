use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let mut galaxies = content.lines().enumerate().flat_map(|(y, line)| line.char_indices().filter(|&(_, c)| c == '#').map(move |(x, _)| (y as u32, x as u32))).collect::<Vec<(u32, u32)>>();

    let empty_y = (0..galaxies.iter().map(|&(y, _)| y).max().unwrap_or_default()).filter(|num| galaxies.iter().all(|(y, _)| y != num)).collect::<Vec<u32>>();
    let empty_x = (0..galaxies.iter().map(|&(y, _)| y).max().unwrap_or_default()).filter(|num| galaxies.iter().all(|(_, x)| x != num)).collect::<Vec<u32>>();

    galaxies.iter_mut().for_each(|(y, x)| {
        *y += empty_y.iter().filter(|&line| line < y).count() as u32;
        *x += empty_x.iter().filter(|&pos| pos < x).count() as u32;
    });

    let ans = galaxies.iter().enumerate().flat_map(|(i, &g1)| galaxies.iter().skip(i + 1).map(move |&g2| (g1, g2))).fold(0u32, |acc, (g1, g2)| acc + g2.0.abs_diff(g1.0) + g2.1.abs_diff(g1.1));
    println!("ANS: {0}", ans);

}
