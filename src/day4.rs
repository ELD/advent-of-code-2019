use std::collections::HashMap;

const RADIX: u32 = 10;

pub fn part1() -> i64 {
    let (start, end) = parse_input();

    let mut valid_inputs = Vec::new();
    (start..end).for_each(|input| {
        let string_pw = format!("{}", input);

        let always_increasing = string_pw
            .chars()
            .zip(string_pw.chars().skip(1))
            .all(|input| i64::from(input.0 as u8) <= i64::from(input.1 as u8));

        let duplicated_digits = string_pw
            .chars()
            .zip(string_pw.chars().skip(1))
            .any(|input| input.0 == input.1);

        if always_increasing && duplicated_digits {
            valid_inputs.push(input);
        }
    });

    valid_inputs.len() as i64
}

// FIXME: Answer 1264
pub fn part2() -> i64 {
    let (start, end) = parse_input();

    let mut increasing_inputs = Vec::new();
    (start..end).for_each(|input| {
        let string_pw = format!("{}", input);

        let always_increasing = string_pw
            .chars()
            .zip(string_pw.chars().skip(1))
            .all(|input| input.0.to_digit(RADIX).unwrap() <= input.1.to_digit(RADIX).unwrap());

        if always_increasing {
            increasing_inputs.push(input);
        }
    });

    let mut valid_inputs = Vec::new();
    for input in increasing_inputs {
        let mut frequency_list = HashMap::new();
        format!("{}", input).chars().for_each(|digit| {
            let digit = digit.to_digit(RADIX).unwrap();

            frequency_list
                .entry(digit)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        });

        let double = frequency_list.iter().any(|(_, value)| *value == 2);
        if double {
            valid_inputs.push(input);
        }
    }

    valid_inputs.len() as i64
}

fn parse_input() -> (i64, i64) {
    let input = include_str!("inputs/day4.txt").trim();

    let input_i64s = input
        .split('-')
        .map(|input| input.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (*input_i64s.first().unwrap(), *input_i64s.last().unwrap())
}
