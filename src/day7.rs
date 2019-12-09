use crate::tape_computer::Amp;
use itertools::Itertools;

pub fn part1() -> i32 {
    let tape = parse_input();

    (0..=4)
        .permutations(5)
        .map(|perm| {
            let mut signal = 0;
            for p in perm {
                let mut amp = Amp::new(tape.clone());

                signal = amp.run(Some(p), signal).unwrap();
            }

            signal
        })
        .max()
        .expect("no maximum found")
}

pub fn part2() -> i32 {
    let tape = parse_input();

    (5..=9)
        .permutations(5)
        .map(|perm| {
            let mut signal = 0;
            let mut amps = vec![Amp::new(tape.clone()); 5];
            let mut input_iter = perm.iter().cloned();

            for amp_id in (0..amps.len()).cycle() {
                if let Some(s) = amps[amp_id].run(input_iter.next(), signal) {
                    signal = s;
                } else {
                    break;
                }
            }

            signal
        })
        .max()
        .expect("no max found")
}

fn parse_input() -> Vec<i32> {
    include_str!("inputs/day7.txt")
        .trim()
        .split(',')
        .map(|inst| inst.parse().unwrap())
        .collect()
}
