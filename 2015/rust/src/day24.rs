use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().expect("")).collect()
}

fn push_next(group: Vec<u64>, packages: &[u64], sum: u64) -> Vec<Vec<u64>> {
    assert!(group.iter().sum::<u64>() < sum);

    packages
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            if group.contains(p) {
                Vec::new()
            } else {
                let mut group = group.clone();
                group.push(*p);
                //println!("{} {} {} {}", packages.len(), i, group.iter().sum::<u64>(), sum);
                match group.iter().sum::<u64>().cmp(&sum) {
                    Ordering::Less => {
                        push_next(group, &packages[(i + 1)..], sum)
                    }
                    Ordering::Equal => vec![group],
                    Ordering::Greater => Vec::new(),
                }
            }
        })
        .collect()
}

fn run(input: &str, groups_nb: u64) -> u64 {
    let packages = parse_input(input);
    let sum = packages.iter().sum::<u64>() / groups_nb;
    let groups = push_next(Vec::new(), &packages, sum);

    let min = groups.iter().map(|g| g.len()).min().unwrap();

    groups
        .iter()
        .filter_map(|g| {
            if g.len() == min {
                Some(g.iter().product::<u64>())
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[aoc(day24, part1)]
fn part1(input: &str) -> u64 {
    run(input, 3)
}

#[aoc(day24, part2)]
fn part2(input: &str) -> u64 {
    run(input, 4)
}
