fn main() {
    part1();
    part2();
}

fn part1() {
    let mut signal = parse_input();

    (0..100).for_each(|_| {
        signal = apply_fft(signal.clone());
    });

    (0..8).for_each(|i| print!("{}", signal[i as usize]));
    println!();
}

fn part2() {
    let mut signal: Vec<i32> = parse_input();
    let size = signal.len();
    signal = signal
        .into_iter()
        .cycle()
        .take(size * 10000)
        .collect::<Vec<_>>();
    let message_offset = signal[0..7].iter().fold(0, |acc, x| acc * 10 + x) as usize;

    assert!(message_offset > signal.len() / 2);
    (0..100).for_each(|_| {
        let mut total = 0;
        (message_offset..signal.len()).rev().for_each(|i| {
            //            eprintln!("signal[{}]: {}", i, signal[i]);
            total += signal[i];
            signal[i] = total % 10;
        });
    });

    (message_offset..message_offset + 8).for_each(|i| print!("{}", signal[i]));
    println!();
}

fn apply_fft(signal: Vec<i32>) -> Vec<i32> {
    signal
        .iter()
        .enumerate()
        .map(|(d, _)| {
            let signal = signal.clone();
            let pattern = generate_pattern(d);
            (signal
                .iter()
                .zip(pattern.iter().cycle().skip(1))
                .map(|(s, m)| (s * m))
                .sum::<i32>())
            .abs()
                % 10
        })
        .collect()
}

fn generate_pattern(digit: usize) -> Vec<i32> {
    let base_pattern = vec![0, 1, 0, -1];
    let mut modified_pattern: Vec<i32> = Vec::new();

    for num in base_pattern {
        (0..=digit).for_each(|_| modified_pattern.push(num));
    }

    modified_pattern.into_iter().collect()
}

fn parse_input() -> Vec<i32> {
    include_str!("day16.txt")
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{apply_fft, generate_pattern};

    #[test]
    fn generates_pattern_for_first_digit() {
        let expected = vec![0, 1, 0, -1];
        let actual = generate_pattern(0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn generates_pattern_for_second_digit() {
        let expected = vec![0, 0, 1, 1, 0, 0, -1, -1];
        let actual = generate_pattern(1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn generates_pattern_for_tenth_digit() {
        let expected = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        ];
        let actual = generate_pattern(9);

        assert_eq!(actual, expected);
    }

    #[test]
    fn fft_accurate_after_one_pass() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![4, 8, 2, 2, 6, 1, 5, 8];

        let actual = apply_fft(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn fft_accurate_after_two_pass() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![3, 4, 0, 4, 0, 4, 3, 8];

        (0..2).for_each(|_| {
            input = apply_fft(input.clone());
        });

        assert_eq!(input, expected);
    }
}
