use num::integer::lcm;
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

#[derive(Clone, Debug)]
enum Module<'a> {
    FlipFlop {
        destinations: Vec<&'a str>,
        state: bool,
    },
    Conjuction {
        destinations: Vec<&'a str>,
        memory: HashMap<&'a str, Pulse>,
    },
    Broadcaster {
        destinations: Vec<&'a str>,
    },
}

impl<'a> Module<'a> {
    fn from_line(line: &'a str) -> (&'a str, Self) {
        let (id, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations.split(", ").collect();

        if id == "broadcaster" {
            (id, Self::Broadcaster { destinations })
        } else {
            let module_type = &id[0..1];

            (
                &id[1..],
                match module_type {
                    "%" => Self::FlipFlop {
                        destinations,
                        state: false,
                    },
                    "&" => Self::Conjuction {
                        destinations,
                        memory: HashMap::new(),
                    },
                    _ => panic!(),
                },
            )
        }
    }

    fn destinations(&self) -> &Vec<&'a str> {
        match self {
            Self::FlipFlop {
                destinations,
                state: _,
            } => destinations,
            Self::Broadcaster { destinations } => destinations,
            Self::Conjuction {
                destinations,
                memory: _,
            } => destinations,
        }
    }

    fn reset(&mut self) {
        match self {
            Self::FlipFlop {
                destinations: _,
                state,
            } => *state = false,
            Self::Broadcaster { destinations: _ } => (),
            Self::Conjuction {
                destinations: _,
                memory,
            } => memory.iter_mut().for_each(|(_, v)| *v = Pulse::Low),
        }
    }

    fn init_module_source(&mut self, id: &'a str) {
        if let Self::Conjuction {
            destinations: _,
            memory,
        } = self
        {
            memory.insert(id, Pulse::Low);
        }
    }

    fn run(
        &mut self,
        from: &'a str,
        input: Pulse,
    ) -> Option<(Pulse, &Vec<&'a str>)> {
        match self {
            Self::FlipFlop {
                destinations,
                state,
            } => {
                match input {
                    Pulse::High => return None,
                    Pulse::Low => *state = !*state,
                }

                Some((
                    if *state { Pulse::High } else { Pulse::Low },
                    destinations,
                ))
            }
            Self::Broadcaster { destinations } => Some((input, destinations)),
            Self::Conjuction {
                destinations,
                memory,
            } => {
                *memory.get_mut(from).unwrap() = input;

                Some((
                    if memory.iter().all(|(_, val)| val.is_high()) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    },
                    destinations,
                ))
            }
        }
    }
}

type Modules<'a> = HashMap<&'a str, Module<'a>>;

fn parse_input(input: &str) -> Modules {
    let mut modules: Modules = HashMap::new();

    input.lines().for_each(|line| {
        let (id, module) = Module::from_line(line);
        modules.insert(id, module);
    });

    for (id, module) in modules.clone().iter() {
        module
            .destinations()
            .iter()
            .for_each(|d| match modules.get_mut(d) {
                None => (),
                Some(c) => c.init_module_source(id),
            });
    }

    modules
}

fn do_step<'a>(
    inputs: Vec<(&'a str, Pulse, &'a str)>,
    modules: &mut Modules<'a>,
) -> Vec<(&'a str, Pulse, &'a str)> {
    inputs
        .iter()
        .filter_map(|(fid, p, nid)| {
            modules
                .get_mut(nid)?
                .run(fid, *p)
                .map(|(next, destinations)| {
                    destinations
                        .iter()
                        .map(|id| (*nid, next, *id))
                        .collect::<Vec<(&str, Pulse, &str)>>()
                })
        })
        .flatten()
        .collect()
}

fn press_button<F>(start_pulse: Pulse, modules: &mut Modules, mut callback: F)
where
    F: FnMut(&Pulse, &str),
{
    let mut inputs = vec![("button", start_pulse, "broadcaster")];

    callback(&start_pulse, "broadcaster");

    while !inputs.is_empty() {
        inputs = do_step(inputs, modules);

        inputs.iter().for_each(|(_, p, nid)| callback(p, nid));
    }
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let mut modules = parse_input(input);

    let mut low_count = 0usize;
    let mut high_count = 0usize;

    for _ in 0..1000 {
        press_button(Pulse::Low, &mut modules, |p, _| match *p {
            Pulse::Low => {
                low_count += 1;
            }
            Pulse::High => {
                high_count += 1;
            }
        });
    }

    low_count * high_count
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let mut modules = parse_input(input);

    // TODO compute the actual output of the cycles must compute to estimate the moment they will
    // be all in Pulse::High state of the memory of the conjuction that controls the rx signal.
    ["sv", "ng", "ft", "jz"]
        .iter()
        .map(|id| {
            let mut press_count = 0;
            let mut id_low_count = 0;

            modules.iter_mut().for_each(|(_, m)| m.reset());

            loop {
                press_count += 1;
                press_button(Pulse::Low, &mut modules, |p, nid| {
                    if let Pulse::Low = *p {
                        if nid == *id {
                            // This is, according to me, a bug but it is
                            // the actual solution. I cannot comprehend
                            // why!?
                            id_low_count += 1;
                        }
                    }
                });

                if id_low_count == 1 {
                    break press_count;
                }
            }
        })
        .fold(1, lcm)
}
