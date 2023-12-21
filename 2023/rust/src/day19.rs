use std::{cmp::Ordering, collections::HashMap};

fn in_range(i: usize, min: usize, max: usize) -> Ordering {
    if i < min {
        Ordering::Less
    } else if i > max {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

struct Rule<'a> {
    category: &'a str,
    order: Ordering,
    limit: usize,
    destination: &'a str,
}

impl<'a> Rule<'a> {
    fn new(s: &'a str) -> Rule {
        let (condition, destination) = match s.split_once(':') {
            Some(x) => x,
            None => {
                return Self {
                    category: "default",
                    order: Ordering::Less,
                    limit: 0,
                    destination: s,
                }
            }
        };
        let category = &condition[0..1];
        let order = match &condition[1..2] {
            ">" => Ordering::Greater,
            "<" => Ordering::Less,
            c => panic!("{c}"),
        };

        let limit = condition[2..].parse::<usize>().unwrap();

        Self {
            category,
            order,
            limit,
            destination,
        }
    }

    fn process(&self, rating: &Rating) -> Option<&str> {
        if self.category == "default" {
            return Some(self.destination);
        }

        rating.iter().find_map(|(cat, count)| {
            if *cat == self.category && count.cmp(&self.limit) == self.order {
                Some(self.destination)
            } else {
                None
            }
        })
    }

    fn process_range(
        &self,
        category: &str,
        (min, max): (usize, usize),
    ) -> Option<(usize, usize, &'a str, Option<(usize, usize)>)> {
        if self.category == "default" {
            Some((min, max, self.destination, None))
        } else if self.category == category {
            match self.order {
                Ordering::Less => match in_range(self.limit, min, max) {
                    Ordering::Less => {
                        // The range is greater than the limit. The condition
                        // is not meant.
                        None
                    }
                    Ordering::Equal => {
                        // The limit is within the range. The condition is
                        // meant for part of the range.
                        Some((
                            min,
                            self.limit - 1,
                            self.destination,
                            Some((self.limit, max)),
                        ))
                    }
                    Ordering::Greater => {
                        // The range is lesser than the limit. The condition
                        // is meant for the whole range.
                        Some((min, max, self.destination, None))
                    }
                },
                Ordering::Greater => match in_range(self.limit, min, max) {
                    Ordering::Less => {
                        // The range is grater than the limit. The condition
                        // is meant for the whole range.
                        Some((min, max, self.destination, None))
                    }
                    Ordering::Equal => {
                        // The limit is within the range. The condition is
                        // meant for part of the range.
                        Some((
                            self.limit + 1,
                            max,
                            self.destination,
                            Some((min, self.limit)),
                        ))
                    }
                    Ordering::Greater => {
                        // The range is lesser than the limit. The condition
                        // is not meant.
                        None
                    }
                },
                _ => panic!(),
            }
        } else {
            None
        }
    }
}

type Rules<'a> = Vec<Rule<'a>>;
type Workflows<'a> = HashMap<&'a str, Rules<'a>>;

fn new_workflow(line: &str) -> (&str, Rules) {
    let (name, rules) = line.split_once('{').unwrap();

    (
        name,
        rules
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(Rule::new)
            .collect(),
    )
}

type Rating<'a> = Vec<(&'a str, usize)>;
type Ratings<'a> = Vec<Rating<'a>>;

fn new_rating(line: &str) -> Rating {
    // Remove {}
    let line = &line[1..line.len() - 1];

    line.split(',')
        .map(|s| {
            let (cat, count) = s.split_once('=').unwrap();
            (cat, count.parse::<usize>().unwrap())
        })
        .collect()
}

fn parse_input(input: &str) -> (Workflows, Ratings) {
    let (workflows_input, ratings_input) = input.split_once("\n\n").unwrap();

    let mut workflows: Workflows = HashMap::new();

    workflows_input.lines().for_each(|l| {
        let (k, v) = new_workflow(l);
        workflows.insert(k, v);
    });

    let ratings: Ratings = ratings_input.lines().map(new_rating).collect();

    (workflows, ratings)
}

fn process(workflows: &Workflows, rating: &Rating, id: &str) -> bool {
    let rules: &Rules = workflows.get(id).unwrap();

    rules
        .iter()
        .find_map(|rule| match rule.process(rating) {
            Some("A") => Some(true),
            Some("R") => Some(false),
            Some(next) => Some(process(workflows, rating, next)),
            None => None,
        })
        .unwrap()
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (workflows, ratings) = parse_input(input);

    ratings
        .iter()
        .filter_map(|rating| {
            if process(&workflows, rating, "in") {
                Some(rating.iter().map(|(_, c)| c).sum::<usize>())
            } else {
                None
            }
        })
        .sum()
}

type Combination<'a> = HashMap<&'a str, (usize, usize)>;

fn process_combination<'a>(
    workflows: &'a Workflows<'a>,
    combination: &Combination<'a>,
    destination: &'a str,
) -> Vec<(Combination<'a>, &'a str)> {
    let rules: &Rules = workflows.get(destination).unwrap();
    let mut combination: Combination = combination.clone();
    let mut ret: Vec<(Combination<'a>, &'a str)> = Vec::new();

    rules.iter().for_each(|rule| {
        if rule.category == "default" {
            ret.push((combination.clone(), rule.destination));
        } else {
            let cat_range: (usize, usize) =
                *combination.get(rule.category).unwrap();

            if let Some((next_min, next_max, next_dest, rest)) =
                rule.process_range(rule.category, cat_range)
            {
                let mut next_combination: Combination = HashMap::new();

                combination.iter().for_each(|(cat, range)| {
                    if cat == &rule.category {
                        next_combination.insert(cat, (next_min, next_max));
                    } else {
                        next_combination.insert(cat, *range);
                    }
                });

                ret.push((next_combination, next_dest));

                if let Some(rest) = rest {
                    *combination.get_mut(rule.category).unwrap() = rest;
                }
            }
        }
    });

    ret
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let (workflows, _) = parse_input(input);

    let mut init: Combination = HashMap::new();

    ["x", "m", "a", "s"].iter().for_each(|c| {
        init.insert(c, (1, 4000));
    });

    let mut combinations: Vec<(Combination, &str)> = vec![(init, "in")];
    let mut accepted: Vec<Combination> = Vec::new();

    loop {
        if combinations.is_empty() {
            break;
        }

        combinations = combinations
            .iter()
            .flat_map(|(combination, destination)| {
                process_combination(&workflows, combination, destination)
            })
            .collect();

        combinations.retain(|(combination, destination)| match *destination {
            "A" => {
                accepted.push(combination.clone());
                false
            }
            "R" => false,
            _ => true,
        })
    }

    accepted
        .iter()
        .map(|c| {
            c.iter()
                .map(|(_, (min, max))| max + 1 - min)
                .product::<usize>()
        })
        .sum()
}
