use std::collections::HashMap;
use std::fmt::Display;
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Hand(String);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_kind = self.get_kind();
        let other_kind = other.get_kind();
        match self_kind == other_kind {
            true => {
                let (unmatched_s, unmatched_o) = self.0.chars().zip(other.0.chars()).find(|(s, o)| s != o).map(|(s, o)| (Card::new_from_string(s), Card::new_from_string(o))).unwrap();
                unmatched_s.cmp(&unmatched_o)
            }
            false => self_kind.cmp(&other_kind)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hand {
    pub fn new(hand: &str) -> Self {
        Hand(hand.to_string())
    }

    fn get_kind(&self) -> u8 {
        // priority from 1u32..=5
        let val = self.0.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|instance| *instance += 1).or_insert(1);
            acc
        }).values().cloned().collect::<Vec<u32>>();

        match val.len() {
            1 => 7,
            2 => match val.contains(&4) {
                true => 6,
                false => 5,
            },
            3 => match val.contains(&3) {
                true => 4,
                false => 3,
            },
            v => 6 - v as u8,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.0)
    }
}

#[derive(Eq)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(u8),
}

impl Card {
    fn new_from_string(query: char) -> Self {
        match query {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            _ => Self::Number(query.to_digit(10).unwrap() as u8),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match self == other {
            true => Ordering::Equal,
            false => match (self, other) {
                (Self::A, _) => Ordering::Greater,
                (_, Self::A) => Ordering::Less,
                (Self::K, _) => Ordering::Greater,
                (_, Self::K) => Ordering::Less,
                (Self::Q ,_) => Ordering::Greater,
                (_, Self::Q) => Ordering::Less,
                (Self::J, _) => Ordering::Greater,
                (_, Self::J) => Ordering::Less,
                (Self::T, _) => Ordering::Greater,
                (_, Self::T) => Ordering::Less,
                (Self::Number(s), Self::Number(o)) => s.cmp(o),
            },
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::A, Self::A) => true,
            (Self::K, Self::K) => true,
            (Self::Q, Self::Q) => true,
            (Self::T, Self::T) => true,
            (Card::Number(s), Card::Number(o)) => s == o,
            _ => false,
        }
    }
}
