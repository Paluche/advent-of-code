use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(&str, usize, usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (id, rest) = l.split_once(" can fly ").unwrap();
            let (speed, rest) = rest.split_once(" km/s for ").unwrap();
            let (active_time, rest) = rest
                .split_once(" seconds, but then must rest for ")
                .unwrap();
            let rest_time = rest.strip_suffix(" seconds.").unwrap();

            (
                id,
                speed.parse::<usize>().unwrap(),
                active_time.parse::<usize>().unwrap(),
                rest_time.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

const TIME: usize = 2_503;

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    let reindeers = parse_input(input);

    reindeers
        .iter()
        .map(|(_, speed, active_time, rest_time)| {
            let cycle_time = active_time + rest_time;
            let cycle_dist = active_time * speed;

            let mut dist = (TIME / cycle_time) * cycle_dist;
            let remaining = TIME % cycle_time;

            if remaining >= *active_time {
                dist += cycle_dist;
            } else {
                dist += remaining * speed;
            }

            dist
        })
        .max()
        .unwrap()
}

fn increment_dist(
    time: usize,
    speed: usize,
    active_time: usize,
    rest_time: usize,
) -> usize {
    if time % (active_time + rest_time) >= active_time {
        0
    } else {
        speed
    }
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    let reindeers = parse_input(input);
    let mut positions: HashMap<&str, usize> = HashMap::new();
    let mut points: HashMap<&str, usize> = HashMap::new();

    reindeers.iter().for_each(|&(id, _, _, _)| {
        positions.insert(id, 0);
        points.insert(id, 0);
    });

    for time in 0..TIME {
        // Update positions;
        for (id, speed, active_time, rest_time) in reindeers.iter() {
            *positions.get_mut(id).unwrap() +=
                increment_dist(time, *speed, *active_time, *rest_time);
        }

        let max = positions.values().max().unwrap();
        let leaders: Vec<&str> = positions
            .iter()
            .filter_map(|(id, dist)| if dist == max { Some(*id) } else { None })
            .collect();

        for leader in leaders {
            *points.get_mut(&leader).unwrap() += 1
        }
    }

    *points.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1
}
