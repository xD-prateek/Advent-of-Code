use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let (workflows, parts) = content.split_once("\n\n").unwrap_or_else(|| panic!("error reading input"));

    let workflows = workflows.lines().fold(HashMap::new(), |mut acc, line| {
        let (name, rules) = line.split_once('{').unwrap_or_else(|| panic!("error fetching names and rules"));
        acc.insert(name, rules[..rules.len() - 1].split(',').collect::<Vec<&str>>());
        acc
    });

    let ans = parts.lines().fold(0usize, |acc, line| {
        let part = Part::new_from_string(line);
        match pass_through_workflows(&part, &workflows) {
            State::Accepted => acc + part.get_total(),
            State::Rejected => acc,
            State::Other(_) => panic!("unexpected state"),
        }
    });

    println!("ANS: {ans}");
}

fn pass_through_workflows<'a>(part: &Part, workflows: &'a HashMap<&str, Vec<&str>>) -> State<'a> {
    let mut next = "in";

    loop {
        let rules = workflows.get(next).unwrap_or_else(|| panic!("no rule found with name {}", next));
        let state = part.get_state_from_rules(rules);
        if let State::Other(s) = state {
            next = s;
        }
        else {
            break state;
        }
    }
}

enum State<'a> {
    Accepted,
    Rejected,
    Other(&'a str),
}

struct Part {
    cool: usize,
    musical: usize,
    aerodynamic: usize,
    shiny: usize,
}

impl Part {
    fn new_from_string(p: &str) -> Self {
        p[1..p.len() - 1].split(',').fold(Self { cool: 0, musical: 0, aerodynamic: 0, shiny: 0 }, |mut acc, rule| {
            let (characterstic, value) = rule.split_once('=').unwrap_or_else(|| panic!("error reading rules"));
            let value = value.parse::<usize>().unwrap_or_else(|_| panic!("eror parsing part value"));
            match characterstic {
                "x" => acc.cool = value,
                "m" => acc.musical = value,
                "a" => acc.aerodynamic = value,
                "s" => acc.shiny = value,
                _ => panic!("unusual characterstic found"),
            };
            acc
        })
    }

    fn get_total(&self) -> usize {
        self.cool + self.musical + self.aerodynamic + self.shiny
    }

    fn get_state_from_rules<'a>(&self, rules: &'a Vec<&str>) -> State<'a> {
        for &rule in rules {
            if let Some((condition, redirect)) = rule.split_once(':') {
                let mut condition_iter = condition.split_inclusive(|c| c == '<' || c == '>').into_iter();
                let characterstic_str = condition_iter.next().unwrap();
                let characterstic_val = match &characterstic_str[..1] {
                    "x" => self.cool,
                    "m" => self.musical,
                    "a" => self.aerodynamic,
                    "s" => self.shiny,
                    _ => panic!("unusual characterstic found"),
                };
                let val = condition_iter.next().unwrap().parse::<usize>().unwrap_or_else(|_| panic!("error parsing value os rule"));
                if match &characterstic_str[1..] {
                    "<" => characterstic_val < val,
                    ">" => characterstic_val > val,
                    _ => panic!("invalid comparator found"),
                } {
                    return self.get_state_from_rule(redirect);
                }
            }
            else {
                    return self.get_state_from_rule(rule);
            }
        }
        State::Other("")
    }

    fn get_state_from_rule<'a>(&self, s: &'a str) -> State<'a> {
        match s {
            "A" => State::Accepted,
            "R" => State::Rejected,
            _ => State::Other(s),
        }
    }
}