enum Spring {
    Damaged,
    Operational,
    Unknown,
}

impl Spring {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Vec<Spring>, Vec<u8>)> {
    input
        .lines()
        .map(|l| {
            let (springs, d_springs) = l.split_once(' ').unwrap();

            (
                springs.chars().map(|c| Spring::from(c)).collect(),
                d_springs
                    .split(',')
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn compute_arrangements(
    (springs, d_springs): &(Vec<Spring>, Vec<u8>),
) -> usize {
    evaluate_row(springs, d_springs, None)
}

fn evaluate_row(
    springs: &[Spring],
    d_springs: &[u8],
    remaining_d: Option<u8>,
) -> usize {
    let mut i: usize = 0;
    let mut remaining_d: Option<u8> = remaining_d;
    // None => Spring can be either damaged or operational
    // Some(0) => The next spring must be operational
    // Some(x > 0) => The next spring must be damaged

    for (j, spring) in springs.iter().enumerate() {
        match spring {
            Spring::Operational => {
                if let Some(x) = remaining_d {
                    if x > 0 {
                        // Spring should have been a damaged one, no solution
                        // in this configuration.
                        return 0;
                    }
                    // End of continuous damaged group.
                    remaining_d = None
                } else {
                    // Spring can be operational or damaged continue.
                }
            }
            Spring::Damaged => {
                if let Some(x) = remaining_d {
                    if x == 0 {
                        // Spring should have been an operational one, no
                        // solution in this configuration.
                        return 0;
                    }

                    // One damage spring less for that group.
                    remaining_d = Some(x - 1);
                } else {
                    // Starting new damaged group.
                    if i == d_springs.len() {
                        // No damaged group available next, no solution in this
                        // configuration.
                        return 0;
                    }

                    remaining_d = Some(d_springs[i] - 1);
                    i += 1;
                }
            }
            Spring::Unknown => {
                if let Some(x) = remaining_d {
                    if x == 0 {
                        // Spring must be an operational one.
                        remaining_d = None;
                    } else {
                        // Spring must be a damaged one.
                        remaining_d = Some(x - 1);
                    }
                } else {
                    // Spring can be operational or damaged.
                    // Run both case
                    //
                    // Operational spring
                    let op_res = evaluate_row(
                        &springs[j + 1..],
                        &d_springs[i..],
                        None
                    );

                    // Damaged spring
                    if i == d_springs.len() {
                        // No damaged group available next, no solution in this
                        // configuration.
                        return op_res;
                    }

                    remaining_d = Some(d_springs[i] - 1);
                    i += 1;

                    // consider the unknown state spring is operational
                    let damages_res = evaluate_row(
                        &springs[j + 1..],
                        &d_springs[i..],
                        remaining_d,
                    );

                    return op_res + damages_res;
                }
            }
        }
    }

    // Check all the damage groups has been consumed.
    if (remaining_d == None || remaining_d == Some(0)) && i == d_springs.len()
    {
        return 1;
    }

    0
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|r| compute_arrangements(r))
        .sum()
}
