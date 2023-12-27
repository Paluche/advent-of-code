use std::collections::HashMap;

enum Logic<'a> {
    In(u16),
    True(&'a str),
    Not(&'a str),
    Or(&'a str, &'a str),
    And(&'a str, &'a str),
    And1(&'a str),
    RShift(&'a str, u8),
    LShift(&'a str, u8),
}

impl<'a> Logic<'a> {
    fn new(expr: &'a str) -> Self {
        if let Some(a) = expr.strip_prefix("NOT ") {
            Self::Not(a)
        } else if let Some(a) = expr.strip_prefix("1 AND ") {
            Self::And1(a)
        } else if let Some((a, b)) = expr.split_once(" AND ") {
            Self::And(a, b)
        } else if let Some((a, b)) = expr.split_once(" OR ") {
            Self::Or(a, b)
        } else if let Some((a, b)) = expr.split_once(" RSHIFT ") {
            Self::RShift(a, b.parse::<u8>().ok().unwrap())
        } else if let Some((a, b)) = expr.split_once(" LSHIFT ") {
            Self::LShift(a, b.parse::<u8>().ok().unwrap())
        } else if let Ok(a) = expr.parse::<u16>() {
            Self::In(a)
        } else {
            Self::True(expr)
        }
    }

    fn exec(&self, states: &HashMap<&str, u16>) -> Option<u16> {
        Some(match self {
            Self::In(x) => *x,
            Self::True(a) => *states.get(a)?,
            Self::Not(a) => !*states.get(a)?,
            Self::Or(a, b) => *states.get(a)? | *states.get(b)?,
            Self::And(a, b) => *states.get(a)? & *states.get(b)?,
            Self::And1(a) => 1 & *states.get(a)?,
            Self::RShift(a, x) => *states.get(a)? >> x,
            Self::LShift(a, x) => *states.get(a)? << x,
        })
    }
}
fn parse_input(input: &str) -> HashMap<&str, Logic> {
    let mut ret: HashMap<&str, Logic> = HashMap::new();
    input.lines().for_each(|line| {
        let (expr, dest) = line.split_once(" -> ").unwrap();

        ret.insert(dest, Logic::new(expr));
    });

    ret
}

fn run(wiring: &HashMap<&str, Logic>) -> u16 {
    let mut states: HashMap<&str, u16> = HashMap::new();

    loop {
        if states.contains_key("a") {
            return *states.get("a").unwrap();
        }

        let mut sthg = false;

        for (wire, logic) in wiring.iter() {
            if states.contains_key(wire) {
                continue;
            }

            if let Some(v) = logic.exec(&states) {
                sthg = true;
                states.insert(wire, v);
            }
        }

        if !sthg {
            panic!()
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u16 {
    run(&parse_input(input))
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u16 {
    let mut wiring = parse_input(input);

    let new_b = run(&wiring);

    *wiring.get_mut("b").unwrap() = Logic::In(new_b);

    run(&wiring)
}
