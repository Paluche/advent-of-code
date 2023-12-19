use pathfinding::matrix::directions::{E, N, S, W};

type Direction = (isize, isize);
type Position = (isize, isize);

fn parse_input_1(input: &str) -> Vec<(Direction, isize)> {
    input
        .lines()
        .map(|line| {
            let mut a = line.split_whitespace();

            let direction = match a.next().unwrap() {
                "R" => E,
                "L" => W,
                "U" => N,
                "D" => S,
                _ => panic!(),
            };

            let count = a.next().unwrap().parse::<isize>().unwrap();

            (direction, count)
        })
        .collect()
}

fn parse_input_2(input: &str) -> Vec<(Direction, isize)> {
    let ret: Vec<(Direction, isize)> = input
        .lines()
        .map(|line| {
            let (_, a) = line.split_once('#').unwrap();

            let count = isize::from_str_radix(&a[..5], 16).unwrap();

            let direction = match &a[5..6] {
                "0" => E,
                "1" => S,
                "2" => W,
                "3" => N,
                _ => panic!(),
            };

            (direction, count)
        })
        .collect();

    ret
}

fn get_corners(instructions: &[(Direction, isize)]) -> (Vec<Position>, isize) {
    let mut ret: Vec<Position> = vec![(0, 0)];
    // The coordonates points to the middle of the edge trench.
    // Half of the trench is outside the edge.
    // The Shoelace formula will compute the area inside the middle, we need to
    // add only half of the width taken by the edge trench. The other half is
    // included in the shoelace formula result.
    // But actually it is a little more than the half that we must add. On each
    // corner, the area computed by the shoelace formula is actually one or
    // three quarters of the space dug. So they mostly compensate each other
    // excepted for 4 corners which actually makes the edge being a loop, on
    // those four corners 3 quarters are not included in the area. 2 quarters
    // are included by half of the perimeter, rest 1 quarters time four makes
    // 1 more. As it is later divided by 2, the perimeter variable is
    // initialized to 2.
    let mut perimeter: isize = 2;

    for (i, (direction, count)) in instructions.iter().enumerate() {
        let position = (
            ret[i].0 + *count * direction.0,
            ret[i].1 + *count * direction.1,
        );

        perimeter += *count;

        ret.push(position);
    }

    // Delay the division by 2, to the last moment. If we do it at each count,
    // we will loose data, count is not necessary an even number. By the total
    // will be as the shape in a loop and count is not a float.
    (ret, perimeter / 2)
}

fn run(instructions: &[(Direction, isize)]) -> isize {
    // Shoelace formula. Compute the area of a surface based on the coordinates
    // of the points forming it.
    // X = columns
    // Y = rows
    // For each line (AB), take the coordinates of the two dots delimiting the
    // line. Do Ax * By = Bx * Ay. Sum all the results and divide by two.
    //
    // https://en.wikipedia.org/wiki/Shoelace_formula
    //
    // The edge is "luckily" oriented counter-clock-wise so the result of the
    // shoelace formula will be positive.
    let (corners, perimeter) = get_corners(instructions);

    let area = corners
        .windows(2)
        .map(|w| w[0].1 * w[1].0 - w[0].0 * w[1].1)
        .sum::<isize>()
        / 2;

    area + perimeter
}

#[aoc(day18, part1)]
fn part1(input: &str) -> isize {
    run(&parse_input_1(input))
}

#[aoc(day18, part2)]
fn part2(input: &str) -> isize {
    run(&parse_input_2(input))
}
