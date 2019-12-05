pub fn part1() {
    let tape = parse_input();

    run_tape(tape, 1);
}

pub fn part2() {
    let tape = parse_input();

    run_tape(tape, 5);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Opcode {
    Add,
    Mul,
    Save,
    Output,
    JIT,
    JIF,
    LT,
    EQ,
    Halt,
}

impl From<i64> for Opcode {
    fn from(input: i64) -> Self {
        match input % 10 {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Save,
            4 => Opcode::Output,
            5 => Opcode::JIT,
            6 => Opcode::JIF,
            7 => Opcode::LT,
            8 => Opcode::EQ,
            _ => Opcode::Halt,
        }
    }
}

impl Into<usize> for Opcode {
    fn into(self) -> usize {
        match self {
            Opcode::Add => 4,
            Opcode::Mul => 4,
            Opcode::Save => 2,
            Opcode::Output => 2,
            Opcode::JIT => 3,
            Opcode::JIF => 3,
            Opcode::LT => 4,
            Opcode::EQ => 4,
            Opcode::Halt => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl From<i64> for ParameterMode {
    fn from(input: i64) -> Self {
        match input % 10 {
            1 => ParameterMode::Immediate,
            _ => ParameterMode::Position,
        }
    }
}

pub(crate) fn run_tape(mut tape: Vec<i64>, input: i64) -> Vec<i64> {
    let mut pc = 0 as usize;
    loop {
        let (opcode, param1_mode, param2_mode, output_mode) = parse_opcode(tape[pc]);
        if opcode == Opcode::Halt {
            break;
        }

        match opcode {
            Opcode::Add => {
                let param1 = match param1_mode {
                    ParameterMode::Immediate => tape[pc + 1],
                    ParameterMode::Position => tape[tape[pc + 1] as usize],
                };

                let param2 = match param2_mode {
                    ParameterMode::Immediate => tape[pc + 2],
                    ParameterMode::Position => tape[tape[pc + 2] as usize],
                };

                match output_mode {
                    ParameterMode::Immediate => tape[pc + 3] = param1 + param2,
                    ParameterMode::Position => {
                        let location: usize = tape[pc + 3] as usize;
                        tape[location] = param1 + param2
                    }
                }
            }
            Opcode::Mul => {
                let param1 = match param1_mode {
                    ParameterMode::Immediate => tape[pc + 1],
                    ParameterMode::Position => tape[tape[pc + 1] as usize],
                };

                let param2 = match param2_mode {
                    ParameterMode::Immediate => tape[pc + 2],
                    ParameterMode::Position => tape[tape[pc + 2] as usize],
                };

                match output_mode {
                    ParameterMode::Immediate => tape[pc + 3] = param1 * param2,
                    ParameterMode::Position => {
                        let location: usize = tape[pc + 3] as usize;
                        tape[location] = param1 * param2
                    }
                }
            }
            Opcode::Output => {
                let param1 = tape[pc + 1] as usize;

                println!("{}", tape[param1]);
            }
            Opcode::Save => {
                let param1 = tape[pc + 1] as usize;

                tape[param1] = input;
            }
            Opcode::JIT => {
                let param1 = match param1_mode {
                    ParameterMode::Immediate => tape[pc + 1],
                    ParameterMode::Position => tape[tape[pc + 1] as usize],
                };

                if param1 != 0 {
                    pc = match param2_mode {
                        ParameterMode::Immediate => tape[pc + 2],
                        ParameterMode::Position => tape[tape[pc + 2] as usize],
                    } as usize;

                    continue;
                }
            }
            Opcode::JIF => {
                let param1 = match param1_mode {
                    ParameterMode::Immediate => tape[pc + 1],
                    ParameterMode::Position => tape[tape[pc + 1] as usize],
                };

                if param1 == 0 {
                    pc = match param2_mode {
                        ParameterMode::Immediate => tape[pc + 2],
                        ParameterMode::Position => tape[tape[pc + 2] as usize],
                    } as usize;

                    continue;
                }
            }
            Opcode::LT => {
                let param1 = match param1_mode {
                    ParameterMode::Immediate => tape[pc + 1],
                    ParameterMode::Position => tape[tape[pc + 1] as usize],
                };

                let param2 = match param2_mode {
                    ParameterMode::Immediate => tape[pc + 2],
                    ParameterMode::Position => tape[tape[pc + 2] as usize],
                };

                let output = if param1 < param2 { 1 } else { 0 };

                let location = tape[pc + 3] as usize;
                tape[location] = output;
            }
            Opcode::EQ => {
                let param1 = match param1_mode {
                    ParameterMode::Immediate => tape[pc + 1],
                    ParameterMode::Position => tape[tape[pc + 1] as usize],
                };

                let param2 = match param2_mode {
                    ParameterMode::Immediate => tape[pc + 2],
                    ParameterMode::Position => tape[tape[pc + 2] as usize],
                };

                let output = if param1 == param2 { 1 } else { 0 };

                let location = tape[pc + 3] as usize;
                tape[location] = output;
            }
            Opcode::Halt => break,
        }

        pc += Into::<usize>::into(opcode);
    }

    tape
}

pub(crate) fn parse_opcode(input: i64) -> (Opcode, ParameterMode, ParameterMode, ParameterMode) {
    (
        Opcode::from(input),
        ParameterMode::from(input / 100),
        ParameterMode::from(input / 1000),
        ParameterMode::from(input / 10000),
    )
}

pub(crate) fn parse_input() -> Vec<i64> {
    include_str!("inputs/day5.txt")
        .trim()
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day5::*;

    #[test]
    fn test_opcode_decoding() {
        let input = 1002;
        let expected_output = (
            Opcode::Mul,
            ParameterMode::Position,
            ParameterMode::Immediate,
            ParameterMode::Position,
        );

        let actual = parse_opcode(input);

        assert_eq!(expected_output, actual);
    }

    #[test]
    fn test_pc_increment() {
        let input = 1002;
        let expected_output = (
            Opcode::Mul,
            ParameterMode::Position,
            ParameterMode::Immediate,
            ParameterMode::Position,
        );

        let actual = parse_opcode(input);

        assert_eq!(4 as usize, actual.0.into());
    }
}
