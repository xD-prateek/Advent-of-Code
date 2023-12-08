use std::fs::read_to_string;

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    println!("{0}", content);
    // convert content to 2D vec
    let data = content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut ans = 0u32;

    let del_move = vec!{ -1i32, 0, 1};
    let movement = del_move.iter().flat_map(|&x| del_move.iter().map(move |&y| (x, y))).collect::<Vec<(i32, i32)>>();

    data.iter().enumerate().flat_map(|(i, line)| {
        line.iter().enumerate().map(move |(j, &c)| (i as i32, j as i32, c))
    }).filter(|(_, _, c)| *c != '.' && !(*c).is_numeric()).for_each(|(i, j, c)| {
            // println!("found {c} at {i}, {j}");
            let mut coordinates = movement.iter().map(|(x, y)| (i + x, j + y)).filter_map(|(x, y)| {
                if x >= 0 && y >= 0 { Some((x as usize, y as usize)) }
                else {None}
            }).collect::<Vec<(usize, usize)>>();
            // get part number from the index
            coordinates.iter().for_each(|(i, j)| {
                let line = data.get(*i);
                if let Some(mut no) = line.and_then(|l| l.get(*j).and_then(|c| c.to_digit(10))) {
                    // println!("{0} found at ({i}, {j})", no);
                    // move right
                    let mut current_j = *j + 1;
                    while let Some(next_num) = line.unwrap().get(current_j).and_then(|a| a.to_digit(10)) {
                        no = no * 10 + next_num;
                        current_j += 1;
                    }
                    // move left
                    current_j = *j - 1;
                    while let Some(prev_num) = line.unwrap().get(current_j).and_then(|a| a.to_digit(10)) {

                    }
                }
            });
        });
    println!("ANS: {0}", ans);
}
