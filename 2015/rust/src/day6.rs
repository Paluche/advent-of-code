use pathfinding::matrix::Matrix;

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

fn parse_coordinates(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(',').unwrap();

    (
        x.parse::<usize>().ok().unwrap(),
        y.parse::<usize>().ok().unwrap(),
    )
}

fn parse_line(line: &str) -> (Action, (usize, usize), (usize, usize)) {
    let mut action: Option<Action> = None;
    let mut line = line;

    if let Some(new_line) = line.strip_prefix("toggle ") {
        action = Some(Action::Toggle);
        line = new_line;
    }

    if let Some(new_line) = line.strip_prefix("turn on ") {
        action = Some(Action::TurnOn);
        line = new_line;
    }

    if let Some(new_line) = line.strip_prefix("turn off ") {
        action = Some(Action::TurnOff);
        line = new_line;
    }

    let (from, to) = line.split_once(" through ").unwrap();

    (
        action.unwrap(),
        parse_coordinates(from),
        parse_coordinates(to),
    )
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let mut grid = Matrix::from_fn(1000, 1000, |_| false);

    input.lines().for_each(|l| {
        let (action, (fx, fy), (tx, ty)) = parse_line(l);

        for x in fx..=tx {
            for y in fy..=ty {
                let light = grid.get_mut((x, y)).unwrap();
                match action {
                    Action::TurnOn => *light = true,
                    Action::TurnOff => *light = false,
                    Action::Toggle => *light = !*light,
                }
            }
        }
    });

    grid.iter().map(|r| r.iter().filter(|x| **x).count()).sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let mut grid = Matrix::from_fn(1000, 1000, |_| 0_usize);

    input.lines().for_each(|l| {
        let (action, (fx, fy), (tx, ty)) = parse_line(l);

        for x in fx..=tx {
            for y in fy..=ty {
                let light = grid.get_mut((x, y)).unwrap();
                match action {
                    Action::TurnOn => *light += 1,
                    Action::TurnOff => {
                        if *light > 0 {
                            *light -= 1
                        }
                    }
                    Action::Toggle => *light += 2,
                }
            }
        }
    });

    grid.iter().map(|r| r.iter().sum::<usize>()).sum()
}
