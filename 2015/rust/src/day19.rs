use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (instructions, input) = input.split_once("\n\n").unwrap();

    let mut instructions: Vec<(&str, &str)> = instructions
        .lines()
        .map(|l| l.split_once(" => ").unwrap())
        .collect();

    instructions.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));

    (instructions, &input[..input.len() - 1])
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (instructions, input) = parse_input(input);

    let mut res: HashSet<String> = HashSet::new();

    instructions.iter().for_each(|(from, into)| {
        (0..input.len()).for_each(|i| {
            if input[i..].starts_with(from) {
                res.insert(format!(
                    "{}{}",
                    &input[..i],
                    input[i..].replacen(from, into, 1)
                ));
            }
        })
    });

    res.len()
}

// Each molecule is formed of several "words" each starts with an upper case.
// If it has more than one letter into it, then they are in lower case.
fn split(s: &str) -> Vec<&str> {
    let mut ret: Vec<&str> = Vec::new();
    let mut start: Option<usize> = None;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if let Some(start) = start {
                ret.push(&s[start..i]);
            }
            start = Some(i);
        }
    }

    if let Some(start) = start {
        ret.push(&s[start..]);
    }

    ret
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let (_, molecule) = parse_input(input);

    // Thanks reddit for the solution.
    //
    // All of the rules are of one of the following forms:
    // α => βγ
    // α => βRnγAr
    // α => βRnγYδAr
    // α => βRnγYδYεAr
    //
    // So each mutation, adds one or two, if Y is present, element in the
    // molecule + some optional extras that will not mutate.
    //
    // As Rn, Ar, and Y are only result of replacement, they never mutate into
    // something else.
    //
    // Y is always paired with something else that could mutate.
    //
    // The numbers of symbols indicates the numbers of transitions done to get
    // the result if we:
    //
    // Subtract #Rn and #Ar because they are just extras, remove them.
    // Subtract two times #Y because we get rid of the Ys and the extra
    // elements following them.
    // Subtract one because we start with "e".

    let molecule = split(molecule);

    let ar_count = molecule.iter().filter(|&element| *element == "Ar").count();
    let rn_count = molecule.iter().filter(|&element| *element == "Rn").count();
    let y_count = molecule.iter().filter(|&element| *element == "Y").count();

    molecule.len() - rn_count - ar_count - 2 * y_count - 1
}
