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
    direction: Direction,
) -> Location<'a> {
    let index = nodes.binary_search_by_key(&location, |(l, _)| l).unwrap();

    nodes[index].1[direction as usize]
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (directions, nodes) = load_input(input);

    let mut ret: usize = 0;
    let mut location = "AAA";

    while location != "ZZZ" {
        let next = next_location(&nodes, location, directions[ret % directions.len()]);
        ret += 1;

        assert!(next != location);

        location = next;
    }

    ret
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    0
}
