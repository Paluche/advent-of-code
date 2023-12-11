use num::integer::lcm;

type Location<'a> = &'a str;
type Node<'a> = (Location<'a>, [Location<'a>; 2]);
type Nodes<'a> = Vec<Node<'a>>;
type Direction = u8;
type Directions = Vec<Direction>;

fn load_input(input: &str) -> (Directions, Nodes) {
    let mut lines = input.lines();

    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect();

    // Drop empty line.
    lines.next();

    let mut nodes: Nodes = lines.map(|l| (&l[0..3], [&l[7..10], &l[12..15]])).collect();

    nodes.sort_by_key(|(l, _)| *l);

    (directions, nodes)
}

fn next_location<'a>(
    nodes: &Nodes<'a>,
    location: Location<'a>,
    directions: &Directions,
    index: usize,
) -> Location<'a> {
    let node_index = nodes.binary_search_by_key(&location, |(l, _)| l).unwrap();
    let direction = directions[index % directions.len()];

    nodes[node_index].1[direction as usize]
}

fn follow<F>(directions: &Directions, nodes: &Nodes, start: Location, f: F) -> usize
where
    F: Fn(Location) -> bool,
{
    let mut ret: usize = 0;
    let mut location = start;

    while !f(location) {
        let next = next_location(nodes, location, directions, ret);
        ret += 1;

        assert!(next != location);

        location = next;
    }

    ret
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (directions, nodes) = load_input(input);

    follow(&directions, &nodes, "AAA", |l| l == "ZZZ")
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let (directions, nodes) = load_input(input);

    let locations: Vec<Location> = nodes
        .iter()
        .filter_map(|(l, _)| if l.ends_with('A') { Some(*l) } else { None })
        .collect();

    // The brut force too long. I suspect that each location is part of a loop.
    // It takes systematically a certain number of steps for each of the start
    // locations. So compute each loop length, math then search for the lowest
    // common multiplier of all the path length.
    //let mut ret: usize = 0;

    //while !locations.iter().all(|l| l.ends_with('Z')) {
    //    locations = locations
    //        .iter()
    //        .map(|l| next_location(&nodes, l, &directions, ret))
    //        .collect();

    //    ret += 1;

    //    if ret == 2000000 {
    //        break;
    //    }
    //}

    // ret

    let lengths: Vec<usize> = locations
        .iter()
        .map(|l| follow(&directions, &nodes, l, |x| x.ends_with('Z')))
        .collect();

    let mut ret: usize = lengths[0];
    for a in lengths[1..].iter() {
        ret = lcm(ret, *a);
    }

    ret
}
