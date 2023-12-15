fn hash_algorithm(c:char, current_value: &mut usize) {
    let mut ret:usize = *current_value as usize;

    if c == '\n' {
        return;
    }

    ret += c as usize;
    ret *= 17;
    *current_value = ret % 256;
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    input.split(',').map(|s| {
        let mut ret: usize = 0;
        for c in s.chars() {
            hash_algorithm(c, &mut ret);
        }
        ret
    }).sum::<usize>()
}
