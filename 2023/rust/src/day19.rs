use std::{cmp::Ordering, collections::HashMap};

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
            None => return Self {
                category: "default",
                order: Ordering::Less,
                limit: 0,
                destination: s
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
    let rules:&Rules = workflows.get(id).unwrap();

    rules.iter().find_map(|rule| {
        match rule.process(rating) {
            Some("A") => Some(true),
            Some("R") => Some(false),
            Some(next) => Some(process(workflows, rating, next)),
            None => None
        }
    }).unwrap()
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (workflows, ratings) = parse_input(input);

    ratings.iter().filter_map(|rating| {
        if process(&workflows, rating, "in") {
            Some(rating.iter().map(|(_, c)| c).sum::<usize>())
        } else {
            None
        }
    }).sum()
}
