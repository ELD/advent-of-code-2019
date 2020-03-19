use pathfinding::directed::dijkstra::{dijkstra, dijkstra_all};
use std::collections::HashMap;
use tape_computer::Amp;

type Grid = HashMap<(i64, i64), Status>;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn reverse(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl From<i64> for Direction {
    fn from(direction: i64) -> Self {
        match direction {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => unreachable!(),
        }
    }
}

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Status {
    Wall,
    Clear,
    OxygenSystem,
}

impl From<i64> for Status {
    fn from(status: i64) -> Self {
        match status {
            0 => Status::Wall,
            1 => Status::Clear,
            2 => Status::OxygenSystem,
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> usize {
    let grid = populate_grid();

    let result = dijkstra(
        &(0, 0),
        |point| {
            let successors = vec![
                (point.0 - 1, point.1),
                (point.0 + 1, point.1),
                (point.0, point.1 + 1),
                (point.0, point.1 - 1),
            ];

            successors
                .iter()
                .filter(|&point| grid.get(point) != Some(&Status::Wall))
                .map(|&point| (point, 1))
                .collect::<Vec<_>>()
        },
        |point| grid.get(point) == Some(&Status::OxygenSystem),
    );

    result.expect("no result").1
}

fn part2() -> i64 {
    let grid = populate_grid();

    let &oxygen_pos = grid
        .iter()
        .filter(|&(_, value)| value == &Status::OxygenSystem)
        .nth(0)
        .expect("no oxygen system found")
        .0;

    let result = dijkstra_all(&oxygen_pos, |point| {
        let successors = vec![
            (point.0 - 1, point.1),
            (point.0 + 1, point.1),
            (point.0, point.1 + 1),
            (point.0, point.1 - 1),
        ];

        successors
            .iter()
            .filter(|&point| grid.get(point) != Some(&Status::Wall))
            .map(|&point| (point, 1))
            .collect::<Vec<_>>()
    });

    result
        .values()
        .max_by_key(|(_, cost)| cost)
        .expect("no max by key")
        .1
}

fn populate_grid() -> Grid {
    let mut droid = Amp::new(parse_input());
    let mut grid = Grid::new();
    grid.insert((0, 0), Status::Clear);

    visit(&mut droid, &mut grid, (0, 0));

    grid
}

fn visit(droid: &mut Amp, grid: &mut Grid, point: (i64, i64)) {
    [
        Direction::North,
        Direction::East,
        Direction::West,
        Direction::South,
    ]
    .iter()
    .for_each(|&dir| {
        let next_pos = match dir {
            Direction::North => (point.0, point.1 + 1),
            Direction::East => (point.0 + 1, point.1),
            Direction::South => (point.0, point.1 - 1),
            Direction::West => (point.0 - 1, point.1),
        };

        if !grid.contains_key(&next_pos) {
            let status = Status::from(droid.run(None, dir.into()).expect("no status"));

            grid.insert(next_pos, status);

            if status != Status::Wall {
                visit(droid, grid, next_pos);

                droid.run(None, dir.reverse().into());
            }
        }
    });
}

fn parse_input() -> Vec<i64> {
    let mut instructions = include_str!("day15.txt")
        .trim()
        .split(',')
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<_>>();

    (0..4096).for_each(|_| instructions.push(0));

    instructions
}
