use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn is_high(&self) -> bool {
        match self {
            Self::Low => false,
            Self::High => true,
        }
    }
}

fn dest_str_to_u16(s: &str) -> u16 {
    let mut chars = s.chars();
    let a = chars.next().unwrap() as u8;
    let b = chars.next().unwrap() as u8;

    assert!(chars.next().is_none());

    ((a as u16) << 8) + b as u16
}

trait Module: std::fmt::Debug {
    fn destinations(&self) -> &Vec<u16>;
    fn run(&mut self, from: u16, input: Pulse) -> Option<Pulse>;
}

#[derive(Clone, Debug)]
struct FlipFlop {
    destinations: Vec<u16>,
    state: bool,
}

impl FlipFlop {
    fn new(destinations: Vec<u16>) -> Self {
        Self {
            destinations,
            state: false,
        }
    }
}

impl Module for FlipFlop {
    fn destinations(&self) -> &Vec<u16> {
        &self.destinations
    }

    fn run(&mut self, _from: u16, input: Pulse) -> Option<Pulse> {
        match input {
            Pulse::High => return None,
            Pulse::Low => self.state = !self.state,
        }

        Some(if self.state { Pulse::High } else { Pulse::Low })
    }
}

#[derive(Clone, Debug)]
struct Conjuction {
    destinations: Vec<u16>,
    memory: HashMap<u16, Pulse>,
}

impl Conjuction {
    fn new(destinations: Vec<u16>) -> Self {
        Self {
            destinations,
            memory: HashMap::new(),
        }
    }

    fn init_source(&mut self, source: u16) {
        self.memory.insert(source, Pulse::Low);
    }
}

impl Module for Conjuction {
    fn destinations(&self) -> &Vec<u16> {
        &self.destinations
    }

    fn run(&mut self, from: u16, pulse: Pulse) -> Option<Pulse> {
        *self.memory.get_mut(&from).unwrap() = pulse;

        Some(if self.memory.iter().all(|(_, val)| val.is_high()) {
            Pulse::Low
        } else {
            Pulse::High
        })
    }
}

#[derive(Clone, Debug)]
struct Broadcaster {
    destinations: Vec<u16>,
}

impl Broadcaster {
    fn new(destinations: Vec<u16>) -> Self {
        Self { destinations }
    }
}

impl Module for Broadcaster {
    fn destinations(&self) -> &Vec<u16> {
        &self.destinations
    }

    fn run(&mut self, _from: u16, input: Pulse) -> Option<Pulse> {
        Some(input)
    }
}

type FlipFlops = HashMap<u16, FlipFlop>;
type Conjuctions = HashMap<u16, Conjuction>;
type Modules<'a> = HashMap<u16, &'a mut dyn Module>;

fn init_conjuctions(
    source: u16,
    destinations: &[u16],
    conjuctions: &mut Conjuctions,
) {
    destinations
        .iter()
        .for_each(|d| match conjuctions.get_mut(d) {
            None => (),
            Some(c) => c.init_source(source),
        });
}

fn parse_input(input: &str) -> (Broadcaster, FlipFlops, Conjuctions) {
    let mut flipflops: FlipFlops = HashMap::new();
    let mut conjuctions: Conjuctions = HashMap::new();
    let mut broadcaster: Option<Broadcaster> = None;

    for line in input.lines() {
        let (name, destinations) = line.split_once(" -> ").unwrap();

        let destinations =
            destinations.split(", ").map(dest_str_to_u16).collect();

        if name == "broadcaster" {
            broadcaster = Some(Broadcaster::new(destinations));
        } else {
            let module_type = &name[0..1];
            let id = dest_str_to_u16(&name[1..]);

            match module_type {
                "%" => {
                    flipflops.insert(id, FlipFlop::new(destinations));
                }
                "&" => {
                    conjuctions.insert(id, Conjuction::new(destinations));
                }
                _ => panic!(),
            };
        };
    }

    (broadcaster.unwrap(), flipflops, conjuctions)
}

fn do_step(
    inputs: Vec<(u16, Pulse, u16)>,
    modules: &mut Modules,
    low_count: &mut usize,
    high_count: &mut usize,
) -> Vec<(u16, Pulse, u16)> {
    let ret: Vec<(u16, Pulse, u16)> = inputs
        .iter()
        .filter_map(|(fid, p, nid)| {
            let module = modules.get_mut(nid)?;
            let next = module.run(*fid, *p);

            next.map(|next| {
                module
                    .destinations()
                    .iter()
                    .map(|id| (*nid, next, *id))
                    .collect::<Vec<(u16, Pulse, u16)>>()
            })
        })
        .flatten()
        .collect();

    ret.iter().for_each(|(_, p, _)| match *p {
        Pulse::Low => *low_count += 1,
        Pulse::High => *high_count += 1,
    });

    ret
}

fn press_button(
    start_pulse: Pulse,
    modules: &mut Modules,
    low_count: &mut usize,
    high_count: &mut usize,
) {
    let mut inputs = vec![(0, start_pulse, 0)];

    *low_count += 1;

    while !inputs.is_empty() {
        inputs = do_step(inputs, modules, low_count, high_count);
    }
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let (mut broadcaster, mut flipflops, mut conjuctions) = parse_input(input);
    let broadcaster_clone = broadcaster.clone();
    let flipflops_clone = flipflops.clone();
    let conjuctions_clone = conjuctions.clone();

    init_conjuctions(0, broadcaster_clone.destinations(), &mut conjuctions);

    for (id, module) in flipflops_clone.iter() {
        init_conjuctions(*id, module.destinations(), &mut conjuctions)
    }

    for (id, module) in conjuctions_clone.iter() {
        init_conjuctions(*id, module.destinations(), &mut conjuctions)
    }

    let mut modules: Modules = HashMap::new();

    modules.insert(0, &mut broadcaster);

    flipflops.iter_mut().for_each(|(n, m)| {
        modules.insert(*n, m);
    });
    conjuctions.iter_mut().for_each(|(n, m)| {
        modules.insert(*n, m);
    });

    let mut low_count = 0usize;
    let mut high_count = 0usize;

    for _ in 0..1000 {
        press_button(Pulse::Low, &mut modules, &mut low_count, &mut high_count);
    }

    low_count * high_count
}
