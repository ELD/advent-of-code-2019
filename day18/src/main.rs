use std::collections::HashMap;

type Point = (usize, usize);
type Grid = HashMap<Point, Space>;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Space {
    Wall,
    Empty,
    Key(char),
    Door(char),
    Start,
}

impl From<char> for Space {
    fn from(space: char) -> Self {
        match space {
            c if c == '#' => Space::Wall,
            c if c == '.' => Space::Empty,
            c if c.is_alphabetic() && c.is_lowercase() => Space::Key(c),
            c if c.is_alphabetic() && c.is_uppercase() => Space::Door(c),
            c if c == '@' => Space::Start,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() {}

fn part2() {}

fn parse_input(input: &'static str) -> (Grid, Point) {
    let mut grid = Grid::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            grid.insert((col, row), Space::from(c));
        })
    });

    let &start = grid
        .iter()
        .find(|(_, &space)| space == Space::Start)
        .unwrap()
        .0;

    (grid, start)
}

fn collect_keys(start: Point, grid: &Grid) -> i32 {
    eprintln!("{:?}", start);
    eprintln!("{:?}", grid);
    0
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn small_example_132_steps() {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        let (grid, start) = parse_input(input);
        let expected = 132;

        let actual = collect_keys(start, &grid);

        assert_eq!(actual, expected);
    }
}
