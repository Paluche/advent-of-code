use json::JsonValue;

fn count(value: &JsonValue, part2: bool) -> isize {
    match value {
        JsonValue::Null => panic!(),
        JsonValue::Short(_) => 0,
        JsonValue::String(_) => panic!(),
        JsonValue::Number(n) => isize::try_from(*n).ok().unwrap(),
        JsonValue::Boolean(_) => 0,
        JsonValue::Object(obj) => {
            if part2 && obj.iter().any(|(_, v)| v == "red") {
                0
            } else {
                obj.iter().map(|(_, v)| count(v, part2)).sum()
            }
        }
        JsonValue::Array(arr) => arr.iter().map(|v| count(v, part2)).sum(),
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> isize {
    count(&json::parse(input).unwrap(), false)
}

#[aoc(day12, part2)]
fn part2(input: &str) -> isize {
    count(&json::parse(input).unwrap(), true)
}
