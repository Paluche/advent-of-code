fn hash_algorithm(s: &str) -> usize {
    s.chars().fold(0usize, |a, c| ((a + c as usize) * 17) % 256)
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    input.split(',').map(|s| hash_algorithm(s.trim_end())).sum()
}

fn load(s:&str) -> (&str, usize, Option<usize>) {
    let (label, focal) = if s.chars().last().unwrap() == '-' {
        (&s[..s.len() - 1], None)
    } else {
        let (label, focal) = s.split_once('=').unwrap();
        (label, Some(focal.parse::<usize>().unwrap()))
    };

    (label, hash_algorithm(label), focal)
}

fn parse_entry<'a>(s: &'a str, boxes: &mut Vec<Vec<(&'a str, usize)>>) {
    let (label, id, focal) = load(s.trim_end());

    if let Some(position) = boxes[id].iter().position(|(l, _)| *l == label)
    {
        boxes[id].remove(position);
        if let Some(focal) = focal {
            boxes[id].insert(position, (label, focal));
        }
    } else if let Some(focal) = focal {
        boxes[id].push((label, focal));
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
