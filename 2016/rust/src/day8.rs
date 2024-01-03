use pathfinding::matrix::Matrix;

enum Rotation {
    Row,
    Column,
}

type Dimensions = (usize, usize);
type Instruction = (Rotation, usize, usize);
type Instructions = Vec<Instruction>;
type Input = Vec<(Dimensions, Instructions)>;

fn to_usize(s: &str) -> usize {
    s.parse().expect("Bad format")
}

fn parse_instruction(s: &str) -> (usize, usize) {
    let (a, b) = s.split_once(" by ").unwrap();
    (to_usize(a), to_usize(b))
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines().peekable();
    let mut ret = Input::with_capacity(input.lines().count() / 2);

    while let Some(line) = lines.next() {
        let dimensions = {
            let line = line.strip_prefix("rect ").unwrap();
            let (columns, rows) = line.split_once('x').unwrap();

            (to_usize(rows), to_usize(columns))
        };

        let mut instructions: Instructions = Vec::new();

        while let Some(line) = lines.next_if(|l| l.starts_with("rotate ")) {
            let (rotation, line) = if let Some(line) =
                line.strip_prefix("rotate row y=")
            {
                (Rotation::Row, line)
            } else if let Some(line) = line.strip_prefix("rotate column x=") {
                (Rotation::Column, line)
            } else {
                panic!();
            };

            let (x, c) = parse_instruction(line);
            instructions.push((rotation, x, c));
        }

        ret.push((dimensions, instructions));
    }

    ret
}

fn light_rectangle(grid: &mut Matrix<bool>, (rows, columns): (usize, usize)) {
    for row in 0..rows {
        for column in 0..columns {
            grid[(row, column)] = true;
        }
    }
}

fn rotate_column(grid: &mut Matrix<bool>, column: usize, count: usize) {
    let prev: Vec<bool> = grid.iter().map(|row| row[column]).collect();

    for row in 0..grid.rows {
        let prev_row = (grid.rows as isize + row as isize - count as isize)
            % grid.rows as isize;
        grid[(row, column)] = prev[prev_row as usize];
    }
}

fn rotate_row(grid: &mut Matrix<bool>, row: usize, count: usize) {
    let prev: Vec<bool> = (0..grid.columns).map(|c| grid[(row, c)]).collect();

    for column in 0..grid.columns {
        let prev_column = (grid.columns as isize + column as isize
            - count as isize)
            % grid.columns as isize;
        grid[(row, column)] = prev[prev_column as usize];
    }
}

fn run(input: &str) -> Matrix<bool> {
    let input = parse_input(input);
    let mut grid = Matrix::from_fn(6, 50, |_| false);

    for (rect, instructions) in input {
        light_rectangle(&mut grid, rect);

        for (rotation, index, count) in instructions {
            match rotation {
                Rotation::Row => rotate_row(&mut grid, index, count),
                Rotation::Column => rotate_column(&mut grid, index, count),
            }
        }
    }

    grid
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    run(input)
        .iter()
        .map(|row| row.iter().filter(|x| **x).count())
        .sum()
}

pub fn print_matrix(matrix: &Matrix<bool>) {
    println!("{}x{}", matrix.rows, matrix.columns);

    for row in matrix {
        for c in row {
            print!("{}", if *c { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

#[aoc(day8, part2)]
fn part2(input: &str) -> &'static str {
    let grid = run(input);

    print_matrix(&grid);

    // Solved visually
    "EFEYKFRFIJ"
}
