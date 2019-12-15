use tape_computer::Amp;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i64 {
    let memory = parse_input();

    let mut computer = Amp::new(memory);
    computer.run(None, 1).expect("no output")
}
fn part2() -> i64 {
    let memory = parse_input();

    let mut computer = Amp::new(memory);
    computer.run(None, 2).expect("no output")
}

fn parse_input() -> Vec<i64> {
    let mut memory: Vec<i64> = include_str!("day9.txt")
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    (0..=4096).for_each(|_| {
        memory.push(0);
    });

    memory
}

#[cfg(test)]
mod test {
    use tape_computer::Amp;

    #[test]
    fn outputs_large_number() {
        let instructions = vec![104, 1125899906842624, 99];
        let expected: i64 = 1125899906842624;

        let mut amp = Amp::new(instructions);
        let actual = amp.run(None, 0).unwrap();

        assert_eq!(actual, expected);
    }
}
