use crate::utils;

struct MapRule {
    pub dest: usize,
    pub source: usize,
    pub range: usize,
}

type Range = (usize, usize); // source, range.

impl MapRule {
    fn max() -> Self {
        Self {
            dest: usize::MAX,
            source: usize::MAX,
            range: 0,
        }
    }
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

    /// not converted (source, range) => converted [(dest, range)] + not yet converted (source, range)
    fn consume(&self, (source, range): Range) -> (Vec<Range>, Option<Range>) {
        let mut ret: Vec<Range> = Vec::new();
        let mut source = source;
        let mut range = range;

        if source < self.source {
            // Considering the map rules are ordered by origin. Everything which is before our
            // source if ranges that have no conversion to do on it.
            let handled_range = range.min(self.source - source);

            assert!(handled_range != 0);

            ret.push((source, handled_range));

            if range == handled_range {
                // Nothing else to handled
                return (ret, None);
            }

            // Update the values. To convert the remaining.
            source = self.source;
            range -= handled_range;
        }

        if !(self.source <= source && source < self.source + self.range) {
            // This map rule does not handled this range.
            return (ret, Some((source, range)));
        }

        let dest = self.convert(source).unwrap();

        let mut handled_range = self.range - (source - self.source);

        if range < handled_range {
            handled_range = range;
        }

        ret.push((dest, handled_range));

        if range == handled_range {
            // Nothing else to handled
            return (ret, None);
        }

        // Update the values. To convert the remaining.
        (ret, Some((self.source + self.range, range - handled_range)))
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

fn parse_map(lines: &mut std::str::Lines) -> Vec<MapRule> {
    let mut map_rules: Vec<MapRule> = Vec::new();

    for line in lines.skip(1) {
        if line.is_empty() {
            break;
        }

        map_rules.push(MapRule::from_line(line))
    }

    map_rules.push(MapRule::max());

    map_rules.sort_by_key(|m| m.source);

    map_rules
}

fn load_input(input: &str) -> (Vec<usize>, Maps) {
    let mut lines = input.lines();
    (
        parse_seeds(&mut lines),
        [
            parse_map(&mut lines),
            parse_map(&mut lines),
            parse_map(&mut lines),
            parse_map(&mut lines),
            parse_map(&mut lines),
            parse_map(&mut lines),
            parse_map(&mut lines),
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

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    let (seeds, maps) = load_input(input);
    let mut ranges: Vec<Range> =
        seeds.chunks_exact(2).map(|x| (x[0], x[1])).collect();

    for map_rules in maps.iter() {
        ranges = ranges
            .iter()
            .flat_map(|r| convert_range(r, map_rules))
            .collect();
    }

    ranges.iter().map(|(s, _)| *s).min().unwrap()
}

fn convert_range(range: &Range, map_rules: &MapRules) -> Vec<Range> {
    let mut ret: Vec<Range> = Vec::new();

    let mut range = Some(*range);

    // For each map, sort map rules by origin!
    for map_rule in map_rules.iter() {
        if range.is_none() {
            // Nothing else to handle.
            break;
        }

        let (mut ranges, next_range) = map_rule.consume(range.unwrap());
        ret.append(&mut ranges);
        range = next_range;
    }

    ret
}
