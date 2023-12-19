use std::{ fs::read_to_string, fmt::Display };

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap();

    let game = content.lines().map(|l| {
        let mut line_iter = l.split_whitespace().take(2);
        (Hand::new(line_iter.next().unwrap()), line_iter.next().unwrap().parse::<u32>().unwrap())
    }).collect::<Vec<(Hand, u32)>>().sort_by(|a, b| a.0.cmp(&b.0));

    println!("{0:?}", game);
}

#[derive(Eq, Debug)]
struct Hand(String);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       todo!(); 
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hand {
    fn new(hand: &str) -> Self {
        Hand(hand.to_string())
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.0)
    }
}
