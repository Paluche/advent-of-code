pub fn parse_numbers<T: std::str::FromStr>(s: &str) -> Vec<T> {
    let mut ret: Vec<T> = Vec::new();

    for number in s.split(' ') {
        if let Ok(number) = number.parse::<T>() {
            ret.push(number)
        }
    }

    ret
}
