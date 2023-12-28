enum Register {
    A,
    B,
}

impl Register {
    fn new(s: &str) -> Self {
        match s {
            "a" => Self::A,
            "b" => Self::B,
            _ => panic!(),
        }
    }
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i8),
    JumpIfEven(Register, i8),
    JumpIfOne(Register, i8),
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let (instruction, rest) = line.split_once(' ').unwrap();

        match instruction {
            "hlf" => Self::Half(Register::new(rest)),
            "tpl" => Self::Triple(Register::new(rest)),
            "inc" => Self::Increment(Register::new(rest)),
            "jmp" => Self::Jump(rest.parse().expect("Bad format")),
            "jie" => {
                let (reg, inc) = rest.split_once(", ").unwrap();
                Self::JumpIfEven(Register::new(reg), inc.parse().expect(""))
            }
            "jio" => {
                let (reg, inc) = rest.split_once(", ").unwrap();
                Self::JumpIfOne(Register::new(reg), inc.parse().expect(""))
            }
            _ => panic!(),
        }
    }

    fn run(&self, reg_a: &mut isize, reg_b: &mut isize, pc: &mut isize) {
        match self {
            Self::Half(reg) => {
                match reg {
                    Register::A => *reg_a /= 2,
                    Register::B => *reg_b /= 2,
                }
                *pc += 1
            }
            Self::Triple(reg) => {
                match reg {
                    Register::A => *reg_a *= 3,
                    Register::B => *reg_b *= 3,
                }
                *pc += 1
            }
            Self::Increment(reg) => {
                match reg {
                    Register::A => *reg_a += 1,
                    Register::B => *reg_b += 1,
                }
                *pc += 1
            }
            Self::Jump(offset) => *pc += *offset as isize,
            Self::JumpIfEven(reg, offset) => {
                if match reg {
                    Register::A => *reg_a % 2 == 0,
                    Register::B => *reg_b % 2 == 0,
                } {
                    *pc += *offset as isize
                } else {
                    *pc += 1
                }
            }
            Self::JumpIfOne(reg, offset) => {
                if match reg {
                    Register::A => *reg_a == 1,
                    Register::B => *reg_b == 1,
                } {
                    *pc += *offset as isize
                } else {
                    *pc += 1
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from_line).collect()
}

fn run(input: &str, reg_a: isize) -> isize {
    let instructions = parse_input(input);
    let mut reg_a = reg_a;
    let mut reg_b = 0;
    let mut pc = 0;

    while (0..instructions.len()).contains(&(pc as usize)) {
        instructions[pc as usize].run(&mut reg_a, &mut reg_b, &mut pc);
    }

    reg_b
}

#[aoc(day23, part1)]
fn part1(input: &str) -> isize {
    run(input, 0)
}

#[aoc(day23, part2)]
fn part2(input: &str) -> isize {
    run(input, 1)
}
