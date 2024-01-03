use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<(Vec<&str>, usize, &str)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('[').unwrap();
            let checksum = b.strip_suffix(']').unwrap();
            let (name, sector_id) = a.rsplit_once('-').unwrap();
            let sector_id = sector_id.parse::<usize>().expect("");
            (name.split('-').collect(), sector_id, checksum)
        })
        .collect()
}

fn is_room_real(ids: &[&str], checksum: &str) -> bool {
    let mut expected_checksum: Vec<(char, usize)> = Vec::new();

    for id in ids {
        for c in id.chars() {
            if let Some(i) = expected_checksum.iter().position(|(b, _)| *b == c)
            {
                expected_checksum.get_mut(i).unwrap().1 += 1;
            } else {
                expected_checksum.push((c, 1));
            }
        }
    }

    expected_checksum.sort_by(|(a, x), (b, y)| match y.cmp(x) {
        Ordering::Equal => a.cmp(b),
        r => r,
    });

    let expected_checksum = expected_checksum.iter().fold(
        String::with_capacity(5),
        |mut ret, (c, _)| {
            if ret.len() < 5 {
                ret.push(*c);
            }
            ret
        },
    );

    expected_checksum == checksum
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|(id, sector_id, checksum)| {
            if is_room_real(id, checksum) {
                *sector_id
            } else {
                0
            }
        })
        .sum()
}

fn decrypt_name(ids: &[&str], sector_id: usize) -> String {
    let mut ret = String::new();
    const A: u32 = 'a' as u32;
    let sector_id = sector_id as u32;

    for id in ids {
        for c in id.chars() {
            let c = c as u32;
            ret.push((A + ((c - A + sector_id) % 26)) as u8 as char);
        }
        ret.push(' ')
    }

    ret
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    for (id, sector_id, checksum) in parse_input(input) {
        if !is_room_real(&id, checksum) {
            continue;
        }

        if decrypt_name(&id, sector_id) != "northpole object storage " {
            continue;
        }

        return sector_id;
    }
    panic!()
}
