pub enum Opcode {
    Add,
    Mul,
}

impl From<i64> for Opcode {
    fn from(input: i64) -> Self {
        match input {
            1 => Self::Add,
            2 => Self::Mul,
            _ => unimplemented!(),
        }
    }
}

pub fn part1() -> i64 {
    let mut input = parse_input();

    input[1] = 12;
    input[2] = 2;

    let mut index = 0 as usize;
    loop {
        if input[index] == 99 {
            break;
        }
        let opcode = Opcode::from(input[index]);
        let (operand_pos1, operand_pos2) = (input[index + 1] as usize, input[index + 2] as usize);
        let output_pos = input[index + 3] as usize;

        match opcode {
            Opcode::Add => input[output_pos] = input[operand_pos1] + input[operand_pos2],
            Opcode::Mul => input[output_pos] = input[operand_pos1] * input[operand_pos2],
        };

        index += 4;
    }

    input[0]
}

pub fn part2() -> i64 {
    const OUTPUT_VALUE: i64 = 19_690_720;

    let input = parse_input();

    for noun in 1..99 {
        for verb in 1..99 {
            let mut tape = input.clone();
            tape[1] = noun;
            tape[2] = verb;
            if run_tape(tape)[0] == OUTPUT_VALUE {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

fn parse_input() -> Vec<i64> {
    include_str!("inputs/day2.txt")
        .trim()
        .split(',')
        .map(|input| input.parse::<i64>().expect("unable to parse"))
        .collect::<Vec<i64>>()
}

fn run_tape(mut tape: Vec<i64>) -> Vec<i64> {
    let mut index = 0 as usize;
    loop {
        if tape[index] == 99 {
            break;
        }
        let opcode = Opcode::from(tape[index]);
        let (operand_pos1, operand_pos2) = (tape[index + 1] as usize, tape[index + 2] as usize);
        let output_pos = tape[index + 3] as usize;

        match opcode {
            Opcode::Add => tape[output_pos] = tape[operand_pos1] + tape[operand_pos2],
            Opcode::Mul => tape[output_pos] = tape[operand_pos1] * tape[operand_pos2],
        };

        index += 4;
    }

    tape
}
