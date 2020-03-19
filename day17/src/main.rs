use std::collections::{HashMap, VecDeque};
use tape_computer::Amp;

type Point = (i32, i32);
type Grid = HashMap<Point, char>;

fn main() {
    println!("{}", part1());
    println!("{:?}", part2());
}

fn part1() -> i32 {
    let mut droid = Amp::new(parse_input());

    let (grid, grid_image) = populate_grid(&mut droid);
    let intersections = find_intersections(&grid);

    print_grid(&grid_image);

    intersections.iter().map(|(x, y)| x * y).sum()
}

// HACK: I calculated the path by hand and then factored the functions
// I should have done this via some sort of compression algorithm and
// a graph discovery, I think?
fn part2() -> i64 {
    let mut memory = parse_input();
    memory[0] = 2;
    let mut droid = Amp::new(memory);

    let instructions: VecDeque<i64> =
        "A,B,A,B,A,C,B,C,A,C\nL,10,L,12,R,6\nR,10,L,4,L,4,L,12\nL,10,R,10,R,6,L,4\nn\ny\n"
            .chars()
            .map(|c| c as u8 as i64)
            .collect();

    droid.input_buffer = instructions;

    while let Some(output) = droid.run(None, 0) {
        if !(output as u8 as char).is_ascii() {
            return output;
        }
    }

    0
}

fn populate_grid(droid: &mut Amp) -> (Grid, Vec<String>) {
    let mut grid = Grid::new();
    let mut buffer = Vec::new();
    let mut line = String::new();

    let mut row = 0;
    let mut col = 0;
    while let Some(output) = droid.run(None, 0) {
        if output == 10 {
            row += 1;
            col = 0;
            buffer.push(line.clone());
            line.clear();
            continue;
        }

        let c = output as u8 as char;
        line.push(c);
        grid.insert((row, col), c);

        col += 1;
    }

    (grid, buffer)
}

fn find_intersections(grid: &Grid) -> Vec<Point> {
    grid.keys()
        .filter(|&key| {
            let possible_points = vec![
                (key.0, key.1 + 1),
                (key.0 + 1, key.1),
                (key.0, key.1 - 1),
                (key.0 - 1, key.1),
            ];

            if *grid.get(key).unwrap() != '#' {
                return false;
            }

            possible_points.iter().all(|point| {
                let grid_space = grid.get(point);
                grid_space.is_some() && *grid_space.unwrap() == '#'
            })
        })
        .map(|&key| key)
        .collect()
}

fn print_grid(grid_image: &Vec<String>) {
    for line in grid_image {
        println!("{}", line);
    }
}

fn parse_input() -> Vec<i64> {
    let mut intcode = include_str!("day17.txt")
        .trim()
        .split(",")
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<_>>();

    (0..4096).for_each(|_| {
        intcode.push(0);
    });

    intcode
}
