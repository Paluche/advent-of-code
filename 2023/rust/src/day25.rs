use pathfinding::directed::bfs::{bfs, bfs_reach};
use std::collections::{HashMap, HashSet};

type Components<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse_input(input: &str) -> Components {
    let mut components: Components =
        HashMap::with_capacity(input.lines().count());

    input.lines().for_each(|l| {
        let (a, rest) = l.split_once(": ").unwrap();
        rest.split_whitespace().for_each(|b| {
            components.entry(a).or_default().insert(b);
            components.entry(b).or_default().insert(a);
        });
    });

    components
}

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
    let components = parse_input(input);
    let keys: Vec<&str> = components.keys().copied().collect();

    // For reach point, start -> end 3 times. Removing the path.
    let start = &keys[0];

    'main: for end in keys.iter().skip(1) {
        let mut components = components.clone();

        for i in 0..=3 {
            let path = bfs(
                start,
                |x| components.get(*x).unwrap().clone(),
                |x| x == end,
            );

            if i == 3 {
                if path.is_some() {
                    continue 'main;
                }
            } else if let Some(path) = path {
                // Remove all wires from the path in the graph we obtained.
                for w in path.windows(2) {
                    components.get_mut(&w[0]).unwrap().remove(&w[1]);
                    components.get_mut(&w[1]).unwrap().remove(&w[0]);
                }
            }
        }

        return [*start, *end]
            .iter()
            .map(|pos| bfs_reach(pos, |x| components.get(*x).unwrap()).count())
            .product();
    }

    0
}
