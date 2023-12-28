use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(&str, &str, usize)> {
    input
        .lines()
        .map(|l| {
            let (loc, length) = l.split_once(" = ").unwrap();
            let length = length.parse::<usize>().unwrap();
            let (from, to) = loc.split_once(" to ").unwrap();

            (from, to, length)
        })
        .collect()
}

fn get_cities<'a>(graph: &[(&'a str, &'a str, usize)]) -> HashSet<&'a str> {
    let mut ret: HashSet<&str> = HashSet::new();

    for (from, to, _) in graph {
        ret.insert(from);
        ret.insert(to);
    }

    ret
}

fn run(input: &str, part1: bool) -> usize {
    let graph = parse_input(input);
    let cities = get_cities(&graph);
    let mut ret = isize::MAX;

    for a in cities.iter() {
        let visited: Vec<&str> = vec![a];

        let (_, length) = dijkstra(
            &(visited),
            |path| {
                graph
                    .iter()
                    .filter_map(|(from, to, length)| {
                        let mut path = path.clone();
                        let length = if part1 {
                            *length as isize
                        } else {
                            -(*length as isize)
                        };

                        let x = path.last().unwrap();

                        if x == from {
                            if path.contains(to) {
                                None
                            } else {
                                path.push(to);
                                Some((path, length))
                            }
                        } else if x == to {
                            if path.contains(from) {
                                None
                            } else {
                                path.push(from);
                                Some((path, length))
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Vec<&str>, isize)>>()
            },
            |path| path.len() == cities.len(),
        )
        .unwrap();

        ret = ret.min(length);
    }

    ret.unsigned_abs()
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    run(input, true)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    run(input, false)
}
