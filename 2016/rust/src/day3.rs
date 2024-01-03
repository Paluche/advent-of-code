fn get_possible(triangles: &[[usize; 3]]) -> usize {
    triangles
        .iter()
        .filter(|sides| {
            sides[0] + sides[1] > sides[2]
                && sides[1] + sides[2] > sides[0]
                && sides[2] + sides[0] > sides[1]
        })
        .count()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let triangles: Vec<[usize; 3]> = input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            let a = l.next().unwrap().parse::<usize>().expect("");
            let b = l.next().unwrap().parse::<usize>().expect("");
            let c = l.next().unwrap().parse::<usize>().expect("");

            [a, b, c]
        })
        .collect();

    get_possible(&triangles)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let triangles: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<usize>().expect(""))
                .collect()
        })
        .collect();

    let triangles: Vec<[usize; 3]> = triangles
        .chunks_exact(3)
        .flat_map(|w| {
            (0..3)
                .map(|i| [w[0][i], w[1][i], w[2][i]])
                .collect::<Vec<[usize; 3]>>()
        })
        .collect();

    get_possible(&triangles)
}
