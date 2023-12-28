type Ingredients = Vec<[isize; 5]>;

fn parse_input(input: &str) -> Ingredients {
    input
        .lines()
        .map(|l| {
            let (_, rest) = l.split_once(": capacity ").unwrap();
            let (capacity, rest) = rest.split_once(", durability ").unwrap();
            let (durability, rest) = rest.split_once(", flavor ").unwrap();
            let (flavor, rest) = rest.split_once(", texture ").unwrap();
            let (texture, calories) = rest.split_once(", calories ").unwrap();

            [
                capacity.parse::<isize>().unwrap(),
                durability.parse::<isize>().unwrap(),
                flavor.parse::<isize>().unwrap(),
                texture.parse::<isize>().unwrap(),
                calories.parse::<isize>().unwrap(),
            ]
        })
        .collect()
}

fn result(
    mult: [usize; 4],
    ingredients: &Ingredients,
    calories: Option<usize>,
) -> usize {
    let res: Vec<usize> = ingredients
        .iter()
        .enumerate()
        .map(|(i, ingredient)| {
            ingredient.iter().map(|v| v * mult[i] as isize).collect()
        })
        .fold([0_isize; 5], |mut sum, ingredient: Vec<isize>| {
            for (i, v) in sum.iter_mut().enumerate() {
                *v += ingredient[i];
            }
            sum
        })
        .iter()
        .map(|&x| if x < 0 { 0 } else { x as usize })
        .collect();

    if let Some(calories) = calories {
        if res[4] != calories {
            return 0;
        }
    }

    res[0..4].iter().product()
}

fn run(input: &str, calories: Option<usize>) -> usize {
    let ingredients = parse_input(input);
    let mut max: usize = usize::MIN;

    for a in 0..100 {
        for b in 0..(100 - a) {
            for c in 0..(100 - (a + b)) {
                let d = 100 - (a + b + c);
                assert_eq!(a + b + c + d, 100);
                max = max.max(result([a, b, c, d], &ingredients, calories));
            }
        }
    }

    max
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    run(input, None)
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    run(input, Some(500))
}
