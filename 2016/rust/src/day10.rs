use std::collections::HashMap;

fn to_usize(s: &str) -> usize {
    s.parse().expect("bad usize format")
}

#[derive(Clone, Copy)]
enum Destination {
    Invalid,
    Output(usize),
    Bot(usize),
}

impl Destination {
    fn new(s: &str) -> Self {
        if let Some(bot_id) = s.strip_prefix("bot ") {
            Self::Bot(to_usize(bot_id))
        } else if let Some(output_id) = s.strip_prefix("output ") {
            Self::Output(to_usize(output_id))
        } else {
            panic!()
        }
    }

    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid)
    }
}

trait Receiver {
    fn gets_value(&mut self, value: usize);

    fn values_full(&self) -> bool;
}

#[derive(Clone, Copy)]
struct Bot {
    values: [Option<usize>; 2],
    give_low: Destination,
    give_high: Destination,
    done_giving: bool,
}

impl Bot {
    fn set_outputs(&mut self, give_low: Destination, give_high: Destination) {
        assert!(
            !give_low.is_invalid()
                && !give_high.is_invalid()
                && self.give_low.is_invalid()
                && self.give_high.is_invalid()
        );

        self.give_low = give_low;
        self.give_high = give_high;
    }

    fn get_values(&self) -> (usize, usize) {
        let mut values: Vec<usize> =
            self.values.iter().map(|x| x.unwrap()).collect();

        values.sort();

        (values[0], values[1])
    }
}

impl Receiver for Bot {
    fn gets_value(&mut self, value: usize) {
        for i in 0..2 {
            if self.values[i].is_none() {
                self.values[i] = Some(value);
                return;
            }
        }
    }

    fn values_full(&self) -> bool {
        self.values.iter().all(|x| x.is_some())
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            values: [None, None],
            give_low: Destination::Invalid,
            give_high: Destination::Invalid,
            done_giving: false
        }
    }
}

#[derive(Default)]
struct Output {
    value: Option<usize>
}

impl Receiver for Output {
    fn gets_value(&mut self, value: usize) {
        if let Some(cur_value) = self.value {
            assert_eq!(cur_value, value);
        } else {
            self.value = Some(value);
        }
    }

    fn values_full(&self) -> bool {
        self.value.is_some()
    }
}

type Bots = HashMap<usize, Bot>;
type Outputs = HashMap<usize, Output>;

fn parse_input(input: &str) -> Bots {
    let mut ret = Bots::new();

    for line in input.lines() {
        if let Some(line) = line.strip_prefix("value ") {
            let (value, bot_id) = line.split_once(" goes to bot ").unwrap();
            let value = to_usize(value);
            let bot_id = to_usize(bot_id);

            ret.entry(bot_id).or_default().gets_value(value);
        } else if let Some(line) = line.strip_prefix("bot ") {
            let (bot_id, rest) = line.split_once(" gives low to ").unwrap();
            let (give_low, give_high) =
                rest.split_once(" and high to ").unwrap();
            let bot_id = to_usize(bot_id);
            let give_low = Destination::new(give_low);
            let give_high = Destination::new(give_high);

            ret.entry(bot_id)
                .or_default()
                .set_outputs(give_low, give_high);
        } else {
            panic!("Bad input {line}");
        }
    }

    ret
}

fn give(
    value: usize,
    destination: Destination,
    bots: &mut Bots,
    outputs: &mut Outputs,
) {
    match destination {
        Destination::Bot(x) => bots.get_mut(&x).unwrap().gets_value(value),
        Destination::Output(x) => {
            outputs.entry(x).or_default().gets_value(value)
        }
        Destination::Invalid => panic!(),
    }
}

fn run(bots: &mut Bots) -> Outputs {
    let mut outputs = Outputs::new();

    loop {
        if bots.iter().all(|(_, b)| b.values_full()) {
            break;
        }

        for (id, bot) in bots.clone().iter() {
            if bot.values_full() && !bot.done_giving {
                let (low_value, high_value) = bot.get_values();
                give(low_value, bot.give_low, bots, &mut outputs);
                give(high_value, bot.give_high, bots, &mut outputs);
                bots.get_mut(id).unwrap().done_giving = true;
            }
        }
    }

    outputs
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let mut bots = parse_input(input);

    run(&mut bots);

    for (id, bot) in bots {
        if bot.get_values() == (17, 61) {
            return id;
        }
    }

    panic!()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut bots = parse_input(input);

    let outputs = run(&mut bots);

    (0..=2).map(|i| outputs[&i].value.unwrap()).product()
}
