use std::collections::HashMap;
use tape_computer::Amp;

type Panel = (i32, i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Color {
    Black,
    White,
}

impl From<i64> for Color {
    fn from(color: i64) -> Self {
        match color {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        }
    }
}

impl Into<i64> for Color {
    fn into(self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_direction(&self, direction: i64) -> Self {
        match self {
            Self::North => {
                if direction == 0 {
                    Self::West
                } else {
                    Self::East
                }
            }
            Self::East => {
                if direction == 0 {
                    Self::North
                } else {
                    Self::South
                }
            }
            Self::South => {
                if direction == 0 {
                    Self::East
                } else {
                    Self::West
                }
            }
            Self::West => {
                if direction == 0 {
                    Self::South
                } else {
                    Self::North
                }
            }
        }
    }
}

struct PaintingRobot {
    location: Panel,
    direction: Direction,
    points_painted: HashMap<Panel, Color>,
}

impl PaintingRobot {
    fn paint(&mut self, color: Color) {
        self.points_painted.insert(self.location, color);
    }

    fn next_panel(&mut self, direction: i64) -> Color {
        self.direction = self.direction.next_direction(direction);

        match self.direction {
            Direction::North => self.location = (self.location.0, self.location.1 + 1),
            Direction::East => self.location = (self.location.0 + 1, self.location.1),
            Direction::South => self.location = (self.location.0, self.location.1 - 1),
            Direction::West => self.location = (self.location.0 - 1, self.location.1),
        };

        self.points_painted
            .get(&self.location)
            .cloned()
            .unwrap_or_else(|| Color::Black)
    }
}

fn main() {
    println!("{:?}", part1());
    part2();
}

fn part1() -> usize {
    let robot = paint_panels(Color::Black);

    robot.points_painted.len()
}

fn part2() {
    let mut robot = paint_panels(Color::White);

    let &(_, min_y) = robot
        .points_painted
        .iter()
        .map(|(key, _)| key)
        .min_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .expect("no min");

    robot.points_painted = robot
        .points_painted
        .iter()
        .map(|((x, y), &v)| ((*x, *y + min_y.abs()), v))
        .collect();

    let mut output_buffer = [[Color::Black; 50]; 6];

    for ((x, y), color) in robot.points_painted {
        output_buffer[y as usize][x as usize] = color;
    }

    let output = output_buffer
        .iter()
        .rev()
        .map(|row| {
            row.iter()
                .map(|&cell| if cell == Color::White { "X" } else { " " })
                .collect()
        })
        .collect::<Vec<String>>();

    for row in output {
        println!("{}", row);
    }
}

fn paint_panels(initial_color: Color) -> PaintingRobot {
    let mem = parse_input();

    let mut robot = PaintingRobot {
        location: (0, 0),
        direction: Direction::North,
        points_painted: HashMap::new(),
    };

    let mut brain = Amp::new(mem);

    let mut panel_color: i64 = initial_color.into();
    while let (Some(color), Some(direction)) =
        (brain.run(None, panel_color), brain.run(None, panel_color))
    {
        let color = Color::from(color);

        robot.paint(color.clone());
        panel_color = robot.next_panel(direction).into();
    }

    robot
}

fn parse_input() -> Vec<i64> {
    let mut memory: Vec<i64> = include_str!("day11.txt")
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    (0..=4096).for_each(|_| {
        memory.push(0);
    });

    memory
}
