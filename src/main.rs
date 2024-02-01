use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

fn main() {
    let file_name: &str = "src/input.txt";
    let content = read_to_string(file_name).unwrap_or_else(|_| panic!("Error reading input."));

    let (workflows, _) = content.split_once("\n\n").unwrap_or_else(|| panic!("error reading input"));

    let rating_start = 1;
    let rating_end = 4000;
    let aplenty = Aplenty::new_from_values(workflows, rating_start, rating_end);

    let ans = aplenty.get_rating_of_accepted_parts();
    println!("ANS: {ans}");
}

struct Aplenty<'a> {
    workflows: HashMap<&'a str, Vec<Rule<'a>>>,
    start: usize,
    end: usize,
}

impl<'a> Aplenty<'a> {
    fn new_from_values(workflows: &'a str, start: usize, end: usize) -> Self {
        let workflows = workflows.lines().fold(HashMap::new(), |mut acc, line| {
            let (name, rules) = line.split_once('{').unwrap_or_else(|| panic!("error fetching names and rules"));
            // convert rules to Rule
            acc.insert(name, rules[..rules.len() - 1].split(',').map(|r| Rule::new_from_str(r)).collect::<Vec<Rule>>());
            acc
        });

        Self {
            workflows,
            start,
            end,
        }
    }

    fn get_rating_of_accepted_parts(&self) -> usize {
        let xmas = [ (self.start, self.end + 1); 4 ];
        self.count("in", xmas)
    }

    fn count(&self, currrent_rule_id: &str, xmas: [(usize, usize); 4]) -> usize {
        let mut xmas = xmas;
        match currrent_rule_id {
            "A" => xmas.iter().map(|(start, end)| end - start).product(),
            "R" => 0,
            _ => {
                let rules = self.workflows.get(currrent_rule_id).unwrap_or_else(|| panic!("{0} rule not found.", currrent_rule_id));
                let mut total = 0;
                for rule in rules {
                    if let Some(comparator) = &rule.comparator {
                        let idx = match comparator.category {
                            Category::Cool => 0,
                            Category::Musical => 1,
                            Category::Aerodynamic => 2,
                            Category::Shiny => 3,
                        };

                        let &(lo, hi) = xmas.get(idx).unwrap();

                        let (true_half, false_half) = match comparator.comparator {
                            Ordering::Less => {
                                ((lo, comparator.value), (comparator.value, hi))
                            },
                            Ordering::Greater => {
                                ((comparator.value + 1, hi), (lo, comparator.value + 1))
                            },
                            Ordering::Equal => panic!("equal not supported in input"),
                        };

                        if true_half.0 < true_half.1 {
                            let mut xmas = xmas.clone();
                            *xmas.get_mut(idx).unwrap() = true_half;
                            total += self.count(rule.redirect, xmas);
                        }

                        if false_half.0 < false_half.1 {
                            *xmas.get_mut(idx).unwrap() = false_half;
                        }
                    }
                    else {
                        total += self.count(rule.redirect, xmas);
                    }
                }
                total
            },
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    comparator: Option<ComparatorRule>,
    redirect: &'a str,
}

impl<'a> Rule<'a> {
    fn new_from_str(rule: &'a str) -> Self {
        if let Some((condition, redirect)) = rule.split_once(':') {
            Self {
                comparator: Some(ComparatorRule::new_from_str(condition)),
                redirect,
            }
        }
        else {
            Self {
                comparator: None,
                redirect: rule,
            }
        }
    }
}

#[derive(Debug)]
struct ComparatorRule {
    category: Category,
    comparator: Ordering,
    value: usize,
}

impl ComparatorRule {
    fn new_from_str(c: &str) -> Self {
        let mut ch = c.chars();
        let category = Category::new_from_str(ch.next().unwrap());
        let comparator = match ch.next().unwrap_or_else(|| panic!("unable to read comparator for rule: {c}")) {
            '>' => Ordering::Greater,
            '<' => Ordering::Less,
            _ => panic!("unable to read comparator for rule: {c}"),
        };
        let value = ch.collect::<String>().parse::<usize>().unwrap_or_else(|_| panic!("Invalid comparator found."));

        Self {
            category,
            comparator,
            value,
        }
    }
}

#[derive(Debug)]
enum Category {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Category {
    fn new_from_str(s: char) -> Self {
        match s {
            'x' => Self::Cool,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("invalid category {s}"),
        }
    }
}