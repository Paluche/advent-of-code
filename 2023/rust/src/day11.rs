use pathfinding::matrix::Matrix;

fn expand(exp_points: &[usize], index: usize, expansion: usize) -> usize {
    if index <= exp_points[0] {
        return index;
    }

    for (i, w) in exp_points.windows(2).enumerate() {
        if w[0] <= index && index < w[1] {
            return index + (i + 1) * (expansion - 1);
        }
    }

    index + exp_points.len() * (expansion - 1)
}

fn run(input: &str, expansion: usize) -> usize {
    let matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();

    let rows_to_expand: Vec<usize> = matrix
        .iter()
        .enumerate()
        .filter_map(|(i, r)| {
            if r.iter().all(|x| *x == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let cols_to_expand: Vec<usize> = matrix
        .rotated_cw(1)
        .iter()
        .enumerate()
        .filter_map(|(i, r)| {
            if r.iter().all(|x| *x == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let galaxies: Vec<(usize, usize)> = matrix
        .items()
        .filter_map(|(pos, c)| match c {
            '.' => None,
            '#' => Some(pos),
            _ => panic!(),
        })
        .collect();

    let exp_galaxies: Vec<(usize, usize)> = galaxies
        .iter()
        .map(|(r, c)| {
            (
                expand(&rows_to_expand, *r, expansion),
                expand(&cols_to_expand, *c, expansion),
            )
        })
        .collect();

    let mut distances: Vec<usize> = Vec::new();

    for (i, a) in exp_galaxies.iter().enumerate() {
        distances.append(
            &mut exp_galaxies[i..]
                .iter()
                .filter_map(|b| if a == b { None } else { Some(dist(a, b)) })
                .collect(),
        );
    }

    distances.iter().sum()
}

fn dist(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    run(input, 2)
}
