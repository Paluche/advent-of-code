#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let input = &input[0..input.len() - 1];
    for i in 0.. {
        let a = format!("{input}{i}");
        let hash = md5::compute(&a);

        if hash[0..2].iter().map(|x| *x as usize).sum::<usize>()
            + (hash[2] >> 4) as usize
            == 0
        {
            return i;
        }
    }

    0
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let input = &input[0..input.len() - 1];
    for i in 0.. {
        let a = format!("{input}{i}");
        let hash = md5::compute(&a);

        if hash[0..3].iter().map(|x| *x as usize).sum::<usize>() == 0 {
            return i;
        }
    }

    0
}
