use pathfinding::prelude::bfs;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum Molecule {
    Hydrogen,
    Lithium,
    Plutonium,
    Strontium,
    Thulium,
    Ruthenium,
    Curium,
}

impl Molecule {
    fn new(s: &str) -> Self {
        match s {
            "hydrogen" => Self::Hydrogen,
            "lithium" => Self::Lithium,
            "plutonium" => Self::Plutonium,
            "strontium" => Self::Strontium,
            "thulium" => Self::Thulium,
            "ruthenium" => Self::Ruthenium,
            "curium" => Self::Curium,
            _ => panic!(),
        }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum Item {
    Generator(Molecule),
    Microchip(Molecule),
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
struct Floor {
    generators: Vec<Molecule>,
    microchips: Vec<Molecule>,
}

impl Floor {
    fn is_empty(&self) -> bool {
        self.generators.is_empty() && self.microchips.is_empty()
    }

    fn items(&self) -> Vec<Item> {
        self.generators
            .iter()
            .map(|x| Item::Generator(*x))
            .chain(self.microchips.iter().map(|x| Item::Microchip(*x)))
            .collect()
    }

    fn move_out(&mut self, item: &Item) {
        match item {
            Item::Generator(x) => self.generators.retain(|g| g != x),
            Item::Microchip(x) => self.microchips.retain(|g| g != x),
        }
    }

    fn move_in(&mut self, item: &Item) {
        match *item {
            Item::Generator(x) => self.generators.push(x),
            Item::Microchip(x) => self.microchips.push(x),
        }
    }

    fn can_move_in(&self, item: &Item) -> bool {
        match item {
            Item::Generator(_) => {
                // Operation can be done if there is no microchip or if the
                // microchips present has their own generators also present.
                self.microchips.is_empty()
                    || self
                        .microchips
                        .iter()
                        .all(|x| self.generators.contains(x))
            }
            Item::Microchip(x) => {
                // Operation can be done if there is no generators or if the
                // generator associated with the microchip is there.
                self.generators.is_empty() || self.generators.contains(x)
            }
        }
    }
}

type Floors = [Floor; 4];

fn parse_line(line: &str) -> Floor {
    let line = line.strip_prefix(" floor contains ").unwrap();
    let mut floor = Floor {
        generators: Vec::new(),
        microchips: Vec::new(),
    };

    let line = line.replace(" and", ",");
    let line = line.strip_suffix('.').unwrap();

    for item in line.split(", ") {
        let item = if let Some(a) = item.strip_prefix("a ") {
            a
        } else if let Some(b) = item.strip_prefix("and a ") {
            b
        } else {
            panic!()
        };

        if let Some(name) = item.strip_suffix(" generator") {
            floor.generators.push(Molecule::new(name));
        } else if let Some(name) = item.strip_suffix("-compatible microchip") {
            floor.microchips.push(Molecule::new(name));
        } else {
            panic!("{}", item);
        }
    }

    floor
}

fn parse_input(input: &str) -> Floors {
    let mut lines = input.lines();

    let first =
        parse_line(lines.next().unwrap().strip_prefix("The first").unwrap());
    let second =
        parse_line(lines.next().unwrap().strip_prefix("The second").unwrap());
    let third =
        parse_line(lines.next().unwrap().strip_prefix("The third").unwrap());
    let fourth = Floor::default();

    [first, second, third, fourth]
}

fn eval_next_floor(
    floors: &Floors,
    current_floor: usize,
    next_floor: usize,
    keep_going: bool,
) -> Vec<Floors> {
    // determinate which objects can be moved.
    let movable_items: Vec<Item> = floors[current_floor]
        .items()
        .iter()
        .filter_map(|x| {
            if floors[next_floor].can_move_in(x) {
                Some(*x)
            } else {
                None
            }
        })
        .collect();

    // Create a path for each of these objects.
    let mut paths: Vec<Floors> = movable_items
        .iter()
        .map(|item| {
            let mut floors = floors.clone();
            floors[current_floor].move_out(item);
            floors[next_floor].move_in(item);

            floors
        })
        .collect();

    if keep_going {
        paths = paths
            .iter()
            .flat_map(|floors| {
                eval_next_floor(floors, current_floor, next_floor, false)
            })
            .collect();
    }

    paths
}

fn successors(
    (current_floor, floors): &(usize, Floors),
) -> Vec<(usize, Floors)> {
    let mut ret: Vec<(usize, Floors)> = Vec::new();

    // - for each floor up / down reachable:
    //     - for each of these paths:
    //          - determinate which objects can be moved.
    //          - create another path for each of these objects.
    //
    //     - add path where nothing moved.
    // - return the paths.

    // Go up or down
    for i in [-1, 1] {
        let next_floor = *current_floor as isize + i;

        if !(0..=3).contains(&next_floor) {
            continue;
        }

        let current_floor = *current_floor;
        let next_floor = next_floor as usize;

        for path in eval_next_floor(floors, current_floor, next_floor, true) {
            ret.push((next_floor, path));
        }
    }

    println!("{current_floor} {ret:?}");
    ret
}

fn success((current_floor, floors): &(usize, Floors)) -> bool {
    *current_floor == 3 && floors[0..3].iter().all(|floor| floor.is_empty())
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let floors = parse_input(input);

    for (i, floor) in floors.iter().enumerate() {
        println!("floor {}: {:?}", i + 1, floor);
    }

    bfs(&(0, floors), successors, success).unwrap().len()
}
