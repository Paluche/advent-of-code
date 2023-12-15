fn hash_algorithm(s: &str) -> usize {
    let mut ret: usize = 0;

    for c in s.chars() {
        if c == '\n' {
            continue;
        }

        ret += c as usize;
        ret *= 17;
        ret %= 256;
    }
    ret
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    input.split(',').map(hash_algorithm).sum()
}

fn parse_entry<'a>(s: &'a str, boxes: &mut Vec<Vec<(&'a str, usize)>>) {
    let s = s.trim_end();

    if s == "" {
        return;
    }

    if s.chars().last().unwrap() == '-' {
        let label = &s[..s.len() - 1];
        let id = hash_algorithm(label);

        if let Some(position) = boxes[id].iter().position(|(l, _)| *l == label)
        {
            boxes[id].remove(position);
        }
    } else {
        let (label, focal) = s.split_once('=').unwrap();
        let id = hash_algorithm(label);
        let focal = focal.parse::<usize>().unwrap();

        if let Some(position) = boxes[id].iter().position(|(l, _)| *l == label)
        {
            boxes[id].remove(position);
            boxes[id].insert(position, (label, focal));
        } else {
            boxes[id].push((label, focal));
        }
    }
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    input.split(',').for_each(|s| parse_entry(s, &mut boxes));

    boxes
        .iter()
        .enumerate()
        .map(|(n, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, (_, f))| (n + 1) * (i + 1) * f)
                .sum::<usize>()
        })
        .sum()
}
