// for each line. Find the numbers and start and stop coordinates. Find the
// symbols and their coordinates.

struct Number {
    value: usize,
    line: usize,
    started: bool,
    start: usize,
    end: usize,
}

impl Number {
    fn new(line: usize, start: usize) -> Self {
        Number {
            value: 0,
            line,
            started: false,
            start,
            end: start,
        }
    }

    fn next_digit(&mut self, digit: char) {
        self.value = self.value * 10 + digit.to_digit(10).unwrap() as usize;
        if self.started {
            self.end += 1;
        } else {
            self.started = true;
        }
    }

    fn has_adjacent_symbol(
        &self,
        symbols: &[Symbol],
        line_numbers: usize,
        column_numbers: usize,
    ) -> Option<usize> {
        let min_line = self.line.checked_sub(1).unwrap_or(self.line);
        let max_line = std::cmp::min(self.line + 1, line_numbers);
        let min_column = self.start.checked_sub(1).unwrap_or(self.start);
        let max_column = std::cmp::min(self.end + 1, column_numbers);

        for symbol in symbols {
            if !(min_line <= symbol.line && symbol.line <= max_line) {
                continue;
            }

            if !(min_column <= symbol.column && symbol.column <= max_column) {
                continue;
            }

            return Some(self.value);
        }

        None
    }
}

struct Symbol {
    pub line: usize,
    pub column: usize,
}

fn parse_lines(input: &str) -> (Vec<Symbol>, Vec<Number>, usize, usize) {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let line_numbers: usize = input.lines().count();
    let mut column_numbers: Option<usize> = None;

    let mut current_number: Option<Number> = None;

    for (line_number, line) in input.lines().enumerate() {
        assert!(column_numbers.get_or_insert(line.len()) == &line.len());

        for (column_number, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let cur = current_number.get_or_insert(Number::new(line_number, column_number));
                cur.next_digit(c)
            } else {
                if let Some(cur) = current_number.take() {
                    numbers.push(cur);
                }

                if c != '.' {
                    symbols.push(Symbol {
                        line: line_number,
                        column: column_number,
                    });
                }
            }
        }
    }

    (symbols, numbers, line_numbers, column_numbers.unwrap())
}
#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let (symbols, numbers, line_numbers, column_numbers) = parse_lines(input);

    numbers
        .iter()
        .filter_map(|n: &Number| n.has_adjacent_symbol(&symbols, line_numbers, column_numbers))
        .sum()
}
