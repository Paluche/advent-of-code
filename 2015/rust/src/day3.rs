use std::collections::HashSet;

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut position: (isize, isize) = (0, 0);

    visited.insert(position);

    input.lines().next().unwrap().chars().for_each(|c| {
        match c {
            '^' => position.0 += 1,
            'v' => position.0 -= 1,
            '>' => position.1 += 1,
            '<' => position.1 -= 1,
            _ => panic!(),
        };
        visited.insert(position);
    });

    visited.len()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut positions: [(isize, isize); 2] = [(0, 0), (0, 0)];

    visited.insert(positions[0]);

    let input: Vec<char> = input.lines().next().unwrap().chars().collect();

    input.iter().enumerate().for_each(|(i, c)| {
        let i = i % 2;
        match c {
            '^' => positions[i].0 += 1,
            'v' => positions[i].0 -= 1,
            '>' => positions[i].1 += 1,
            '<' => positions[i].1 -= 1,
            _ => panic!(),
        };
        visited.insert(positions[i]);
    });

    visited.len()
}
