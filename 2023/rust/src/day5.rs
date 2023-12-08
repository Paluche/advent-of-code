use crate::utils;

struct MapRule {
    dest: usize,
    source: usize,
    range: usize,
}

impl MapRule {
    fn from_line(line: &str) -> Self {
        let mut numbers_s = line.split(' ');

        Self {
            dest: numbers_s.next().unwrap().parse::<usize>().unwrap(),
            source: numbers_s.next().unwrap().parse::<usize>().unwrap(),
            range: numbers_s.next().unwrap().parse::<usize>().unwrap(),
        }
    }

    fn convert(&self, value: usize) -> Option<usize> {
        let i = value.checked_sub(self.source)?;

        if i > self.range {
            return None;
        }

        Some(self.dest + i)
    }
}

type MapRules = Vec<MapRule>;
type Maps = [MapRules; 7];

fn convert(map_rules: &MapRules, value: usize) -> usize {
    for map_rule in map_rules.iter() {
        if let Some(ret) = map_rule.convert(value) {
            return ret;
        }
    }

    // Any source numbers that aren't mapped correspond to the same destination
    // number.
    value
}

fn parse_seeds(lines: &mut std::str::Lines) -> Vec<usize> {
    let expected_prefix = "seeds: ";
    let line = lines.next().unwrap();
    if !line.starts_with(expected_prefix) {
        panic!();
    }

    // Skip next empty line.
    lines.next();

    utils::parse_numbers::<usize>(&line[expected_prefix.len()..])
}

fn parse_map(expected: &str, lines: &mut std::str::Lines) -> Vec<MapRule> {
    let line = lines.next().unwrap();
    let (name, _) = line.rsplit_once(' ').unwrap();

    assert!(name.starts_with(expected));

    let mut map_rules: Vec<MapRule> = Vec::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        map_rules.push(MapRule::from_line(line))
    }

    map_rules
}

fn load_input(input: &str) -> (Vec<usize>, Maps) {
    let mut lines = input.lines();
    (
        parse_seeds(&mut lines),
        [
            parse_map("seed-to-soil", &mut lines),
            parse_map("soil-to-fertilizer", &mut lines),
            parse_map("fertilizer-to-water", &mut lines),
            parse_map("water-to-light", &mut lines),
            parse_map("light-to-temperature", &mut lines),
            parse_map("temperature-to-humidity", &mut lines),
            parse_map("humidity-to-location", &mut lines),
        ],
    )
}

fn seed_to_location(seed: usize, maps: &Maps) -> usize {
    let mut ret = seed;
    for map_rules in maps.iter() {
        ret = convert(map_rules, ret);
    }

    ret
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let (seeds, maps) = load_input(input);

    seeds
        .iter()
        .map(|s| seed_to_location(*s, &maps))
        .min()
        .unwrap()
}
