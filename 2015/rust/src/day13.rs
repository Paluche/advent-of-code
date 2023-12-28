use std::collections::{HashMap, HashSet};

use pathfinding::directed::dijkstra::dijkstra;

type Graph<'a> = HashMap<[&'a str; 2], isize>;
type People<'a> = HashSet<&'a str>;

fn parse_input(input: &str) -> Graph {
    let mut ret: HashMap<[&str; 2], isize> = HashMap::new();
    input.lines().for_each(|l| {
        let (a, rest) = l.split_once(" would ").unwrap();
        let (weight, b) = rest
            .split_once(" happiness units by sitting next to ")
            .unwrap();

        let weight = if let Some(weight) = weight.strip_prefix("gain ") {
            -weight.parse::<isize>().unwrap()
        } else if let Some(weight) = weight.strip_prefix("lose ") {
            weight.parse::<isize>().unwrap()
        } else {
            panic!()
        };

        let b = b.strip_suffix('.').unwrap();

        let mut key = [a, b];
        key.sort();

        if let Some(cur_weight) = ret.get_mut(&key) {
            *cur_weight += weight;
        } else {
            ret.insert(key, weight);
        }
    });

    ret
}

fn list_people<'a>(graph: &Graph<'a>) -> People<'a> {
    let mut ret: HashSet<&str> = HashSet::new();

    graph.keys().for_each(|k| {
        ret.insert(k[0]);
        ret.insert(k[1]);
    });

    ret
}

fn run(graph: &Graph, people: &People) -> isize {
    people
        .iter()
        .map(|start| {
            let visited: Vec<&str> = vec![start];
            let (_, happiness) = dijkstra(
                &(visited),
                |visited| {
                    if visited.len() == people.len() {
                        let mut key = [start, *visited.last().unwrap()];
                        let mut visited = visited.clone();

                        key.sort();
                        visited.push(start);

                        if let Some(weight) = graph.get(&key) {
                            vec![(visited, *weight)]
                        } else {
                            Vec::new()
                        }
                    } else {
                        let ret = graph
                            .iter()
                            .filter_map(|(k, weight)| {
                                let mut visited = visited.clone();
                                let a = k[0];
                                let b = k[1];
                                let x = *visited.last().unwrap();

                                if x == a {
                                    if visited.contains(&b) {
                                        None
                                    } else {
                                        visited.push(b);
                                        Some((visited, *weight))
                                    }
                                } else if x == b {
                                    if visited.contains(&a) {
                                        None
                                    } else {
                                        visited.push(a);
                                        Some((visited, *weight))
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<(Vec<&str>, isize)>>();

                        ret
                    }
                },
                |visited| visited.len() == people.len() + 1,
            )
            .unwrap();
            -happiness
        })
        .max()
        .unwrap()
}

#[aoc(day13, part1)]
fn part1(input: &str) -> isize {
    let graph = parse_input(input);
    let people = list_people(&graph);

    run(&graph, &people)
}

#[aoc(day13, part2)]
fn part2(input: &str) -> isize {
    let mut graph = parse_input(input);
    let mut people = list_people(&graph);
    let me = "me";

    people.insert(me);

    people.iter().for_each(|p| {
        let mut key = [me, p];
        key.sort();
        graph.insert(key, 0);
    });

    run(&graph, &people)
}
