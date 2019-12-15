use std::cmp::Ordering;
use tape_computer::Amp;

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(input: i64) -> Self {
        match input {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{:?}", part2());
}

fn part1() -> i64 {
    let mut computer = Amp::new(parse_input());

    let mut block_count = 0;
    while let (Some(_), Some(_), Some(tile)) = (
        computer.run(None, 2),
        computer.run(None, 2),
        computer.run(None, 2),
    ) {
        if Tile::from(tile) == Tile::Block {
            block_count += 1;
        }
    }

    block_count
}

fn part2() -> i64 {
    let mut memory = parse_input();
    memory[0] = 2;
    let mut computer = Amp::new(memory);

    let mut score = 0;
    let mut joystick_input = None;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    while let (Some(x), Some(y), Some(tile)) = (
        computer.run(joystick_input, 2),
        computer.run(joystick_input, 2),
        computer.run(joystick_input, 2),
    ) {
        if x == -1 && y == 0 {
            score = tile;
            continue;
        }

        match Tile::from(tile) {
            Tile::Ball => ball_x = x,
            Tile::HorizontalPaddle => paddle_x = x,
            _ => {}
        };

        joystick_input = match ball_x.cmp(&paddle_x) {
            Ordering::Greater => Some(1),
            Ordering::Equal => Some(0),
            Ordering::Less => Some(-1),
        };
    }

    score
}

fn parse_input() -> Vec<i64> {
    let mut intcode = include_str!("day13.txt")
        .trim()
        .split(',')
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    (0..4096).for_each(|_| {
        intcode.push(0);
    });

    intcode
}
