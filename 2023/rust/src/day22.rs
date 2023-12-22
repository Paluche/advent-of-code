use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinates {
    fn new(line: &str) -> Self {
        let mut s = line.split(',');
        Self {
            x: s.next().unwrap().parse::<usize>().unwrap(),
            y: s.next().unwrap().parse::<usize>().unwrap(),
            z: s.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

fn intersects(min_1: usize, max_1: usize, min_2: usize, max_2: usize) -> bool {
    (min_1 <= min_2 && min_2 <= max_1)
        || (min_1 <= max_2 && max_2 <= max_1)
        || (min_2 <= min_1 && min_1 <= max_2)
        || (min_2 <= max_1 && max_1 <= max_2)
}

#[derive(Clone, Copy)]
struct Brick {
    start: Coordinates,
    end: Coordinates,
}

impl Brick {
    fn new(line: &str) -> Self {
        let (start, end) = line.split_once('~').unwrap();
        Self {
            start: Coordinates::new(start),
            end: Coordinates::new(end),
        }
    }

    fn ground() -> Self {
        Self {
            start: Coordinates { x: 0, y: 0, z: 0 },
            end: Coordinates {
                x: usize::MAX,
                y: usize::MAX,
                z: 0,
            },
        }
    }

    fn has_below(&self, other: &Self) -> bool {
        intersects(self.start.x, self.end.x, other.start.x, other.end.x)
            && intersects(self.start.y, self.end.y, other.start.y, other.end.y)
    }

    fn settled_on(&mut self, other: &Self) -> bool {
        self.start.z == other.end.z + 1
    }

    fn settle_on(&mut self, other: &Self) {
        let z = other.end.z + 1;
        let dist = self.end.z - self.start.z;
        self.start.z = z;
        self.end.z = z + dist;
    }

    fn cmp(&self, other: &Self) -> Ordering {
        // Compare on the z axis.
        self.start.z.cmp(&other.start.z)
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = input.lines().map(Brick::new).collect();

    bricks.sort_by(|a, b| a.cmp(b));
    bricks.insert(0, Brick::ground());

    bricks
}

fn fall(
    bricks: &mut [Brick],
    mut supports: Option<&mut Vec<Vec<usize>>>,
    mut supported_by: Option<&mut Vec<Vec<usize>>>,
) -> usize {
    let mut ret: usize = 0;

    for i in 1..bricks.len() {
        let bricks_below: Vec<(usize, usize)> = (0..i)
            .rev()
            .filter_map(|j| {
                if bricks[i].has_below(&bricks[j]) {
                    Some((j, bricks[j].end.z))
                } else {
                    None
                }
            })
            .collect();

        let z_max = bricks_below.iter().max_by_key(|(_, z)| *z).unwrap().1;

        for (j, z) in bricks_below {
            if z == z_max {
                let bricks_below = bricks[j];

                if let Some(s) = &mut supports {
                    s[j].push(i);
                }

                if let Some(s) = &mut supported_by {
                    s[i].push(j);
                }

                if !bricks[i].settled_on(&bricks_below) {
                    bricks[i].settle_on(&bricks_below);
                    ret += 1;
                }
            }
        }
    }

    ret
}

#[aoc(day22, part1)]
fn part1(input: &str) -> usize {
    let mut bricks: Vec<Brick> = parse_input(input);
    let len = bricks.len();
    let mut supports: Vec<Vec<usize>> = Vec::with_capacity(len);
    let mut supported_by: Vec<Vec<usize>> = Vec::with_capacity(len);

    (0..bricks.len()).for_each(|_| {
        supports.push(Vec::new());
        supported_by.push(Vec::new());
    });

    // print_bricks(&bricks);
    // Bricks are sorted. Make then fall.
    fall(&mut bricks, Some(&mut supports), Some(&mut supported_by));

    // Does one brick is the only support for another one? If not count it in.
    (0..bricks.len())
        .filter(|i| !supported_by.iter().any(|v| v.len() == 1 && v.contains(i)))
        .count()
}

#[aoc(day22, part2)]
fn part2(input: &str) -> usize {
    let mut bricks: Vec<Brick> = parse_input(input);

    fall(&mut bricks, None, None);

    (1..bricks.len())
        .map(|i| {
            let mut bricks = bricks.clone();

            bricks.remove(i);
            fall(&mut bricks, None, None)
        })
        .sum()
}
