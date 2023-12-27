struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    fn new(line: &str) -> Self {
        let mut a = line.split('x');
        Self {
            length: a.next().unwrap().parse::<usize>().unwrap(),
            width: a.next().unwrap().parse::<usize>().unwrap(),
            height: a.next().unwrap().parse::<usize>().unwrap(),
        }
    }

    fn wrapping_paper_dimension(&self) -> usize {
        let a = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];

        a.iter().map(|s| 2 * s).sum::<usize>() + a.iter().min().unwrap()
    }

    fn ribbon_dimension(&self) -> usize {
        let a = [
            self.length + self.width,
            self.width + self.height,
            self.height + self.length,
        ];

        a.iter().min().unwrap() * 2 + self.length * self.width * self.height
    }
}

fn run<F>(input: &str, f: F) -> usize
where
    F: Fn(Present) -> usize,
{
    input.lines().map(|l| f(Present::new(l))).sum()
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    run(input, |w| w.wrapping_paper_dimension())
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    run(input, |w| w.ribbon_dimension())
}
