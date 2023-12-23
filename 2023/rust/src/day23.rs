use pathfinding::matrix::{
    directions::{E, N, S, W},
    Matrix,
};

type Position = (usize, usize);
type Direction = (isize, isize);

#[derive(Copy, Clone)]
enum Terrain {
    Path,
    Forest,
    Slope(Direction),
}

impl Terrain {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(N),
            '>' => Self::Slope(E),
            '<' => Self::Slope(W),
            'v' => Self::Slope(S),
            _ => panic!(),
        }
    }

    fn is_path(&self) -> bool {
        matches!(self, Self::Path)
    }
}

fn next(
    map: &Matrix<Terrain>,
    start: Position,
    direction: Direction,
) -> Option<Position> {
    let position = map.move_in_direction(start, direction)?;

    match map[position] {
        Terrain::Path | Terrain::Slope(_) => Some(position),
        Terrain::Forest => None,
    }
}

fn successors(
    map: &Matrix<Terrain>,
    start: Position,
    part2: bool,
) -> Vec<Position> {
    let terrain = map[start];
    let directions = if part2 || terrain.is_path() {
        vec![N, E, S, W]
    } else if let Terrain::Slope(d) = terrain {
        vec![d]
    } else {
        panic!()
    };

    directions
        .iter()
        .filter_map(|&d| next(map, start, d))
        .collect()
}

fn parse_input(input: &str) -> (Matrix<Terrain>, Position, Position) {
    let map: Matrix<Terrain> = Matrix::from_rows(
        input
            .lines()
            .take_while(|x| !x.is_empty())
            .map(|l| l.chars().map(Terrain::from_char)),
    )
    .unwrap();

    let start: Position = (0..map.columns)
        .find_map(|c| match map[(0, c)] {
            Terrain::Path => Some((0, c)),
            _ => None,
        })
        .unwrap();

    let end: Position = (0..map.columns)
        .find_map(|c| match map[(map.rows - 1, c)] {
            Terrain::Path => Some((map.rows - 1, c)),
            _ => None,
        })
        .unwrap();

    (map, start, end)
}

#[derive(Clone)]
struct Path {
    position: Position,
    previous: Option<Box<Self>>,
}

impl Path {
    fn root(position: Position) -> Self {
        Self {
            previous: None,
            position,
        }
    }

    fn new(previous: &Self, position: Position) -> Self {
        Self {
            previous: Some(Box::new(previous.clone())),
            position,
        }
    }

    fn contains(&self, position: Position) -> bool {
        let mut current = Some(self);

        while current.is_some() {
            if current.unwrap().position == position {
                return true;
            }

            current = self.previous.as_deref();
        }

        false
    }

    fn weight(&self) -> usize {
        let mut current = Some(self);
        let mut ret = 0;

        while current.is_some() {
            ret += 1;
            current = self.previous.as_deref();
        }

        ret
    }
}

fn run(input: &str, part2: bool) -> usize {
    let (map, start, end) = parse_input(input);

    let mut ways: Vec<Path> = vec![Path::root(start)];
    let mut ok_ways: Vec<usize> = Vec::new();

    loop {
        if ways.is_empty() {
            break;
        }

        let next_ways: Vec<Path> = ways
            .iter()
            .filter_map(|path| {
                let next: Vec<Position> =
                    successors(&map, path.position, part2)
                        .iter()
                        .filter_map(|&pos| {
                            if path.contains(pos) {
                                None
                            } else {
                                Some(pos)
                            }
                        })
                        .collect();

                if next.is_empty() {
                    return None;
                }

                Some(
                    next.iter()
                        .filter_map(|&position| {
                            let next_path = Path::new(path, position);

                            if position == end {
                                ok_ways.push(path.weight());
                                None
                            } else {
                                Some(next_path)
                            }
                        })
                        .collect::<Vec<Path>>(),
                )
            })
            .flatten()
            .collect();
        ways = next_ways;
    }

    *ok_ways.iter().max().unwrap()
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    run(input, false)
}

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    0 // run(input, true)
}
