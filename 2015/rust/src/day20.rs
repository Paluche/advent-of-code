fn run(input: &str, gift_count: usize, limit: usize) -> usize {
    let goal = input
        .lines()
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("bad format")
        / gift_count;
    let mut houses: Vec<usize> = vec![0; goal];
    let mut ret = usize::MAX;

    for i in 1..goal {
        for k in 1..=limit {
            let j = i * k;

            if j >= goal {
                break;
            }

            houses[j] += i;

            if houses[j] >= goal {
                ret = ret.min(j);
            }
        }
    }

    ret
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    run(input, 10, usize::MAX)
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    run(input, 11, 50)
}
