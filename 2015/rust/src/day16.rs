use json::{parse, JsonValue};

const REFERENCE: [(&str, usize); 10] = [
    ("children", 3),
    ("cats", 7),
    ("samoyeds", 2),
    ("pomeranians", 3),
    ("akitas", 0),
    ("vizslas", 0),
    ("goldfish", 5),
    ("trees", 3),
    ("cars", 2),
    ("perfumes", 1),
];

fn parse_input(input: &str) -> Vec<(usize, JsonValue)> {
    input
        .lines()
        .map(|l| {
            let rest = l.strip_prefix("Sue ").unwrap();
            let (id, rest) = rest.split_once(": ").unwrap();
            let id = id.parse::<usize>().unwrap();

            let mut rest = rest.replace(':', "\":");
            rest = rest.replace(", ", ", \"");

            (id, parse(format!("{{\"{rest}}}").as_str()).ok().unwrap())
        })
        .collect()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let sues = parse_input(input);

    'main: for (id, sue) in sues.iter() {
        for (key, value) in REFERENCE.iter() {
            if let JsonValue::Object(obj) = sue {
                if let Some(val) = obj.get(key) {
                    if val != value {
                        continue 'main;
                    }
                }
            } else {
                panic!();
            }
        }
        return *id;
    }

    0
}

#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
    let sues = parse_input(input);

    'main: for (id, sue) in sues.iter() {
        for &(key, value) in REFERENCE.iter() {
            if let JsonValue::Object(obj) = sue {
                if let Some(JsonValue::Number(val)) = obj.get(key) {
                    let val = usize::try_from(*val).ok().unwrap();

                    match key {
                        "cats" | "trees" => {
                            if val <= value {
                                continue 'main;
                            }
                        }
                        "goldfish" | "pomeranians" => {
                            if val >= value {
                                continue 'main;
                            }
                        }
                        _ => {
                            if val != value {
                                continue 'main;
                            }
                        }
                    }
                }
            } else {
                panic!();
            }
        }
        return *id;
    }

    0
}
