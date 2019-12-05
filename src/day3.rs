use std::collections::HashSet;

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

type WireCoords = HashSet<(i64, i64)>;

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c.to_ascii_lowercase() {
            'l' => Self::Left,
            'u' => Self::Up,
            'r' => Self::Right,
            'd' => Self::Down,
            _ => unimplemented!(),
        }
    }
}

pub fn part1() -> i64 {
    let (wire_one, wire_two) = parse_input();

    let wire_one_coords = compute_coords(wire_one);
    let wire_two_coords = compute_coords(wire_two);

    let intersection = wire_one_coords.intersection(&wire_two_coords);

    intersection.map(|(x, y)| x.abs() + y.abs()).min().unwrap()
}

pub fn part2() -> i64 {
    let (wire_one, wire_two) = parse_input();

    let wire_one_coords = compute_coords(wire_one.clone());
    let wire_two_coords = compute_coords(wire_two.clone());

    let intersections = wire_one_coords.intersection(&wire_two_coords);

    intersections
        .map(|intersection| {
            let one_dist = compute_distance_to(*intersection, &wire_one);
            let two_dist = compute_distance_to(*intersection, &wire_two);

            one_dist + two_dist
        })
        .min()
        .unwrap()
}

fn parse_input() -> (Vec<&'static str>, Vec<&'static str>) {
    let directions = include_str!("inputs/day3.txt")
        .trim()
        .split('\n')
        .map(|directions| directions.split(',').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    (directions[0].clone(), directions[1].clone())
}

fn compute_coords(directions: Vec<&str>) -> WireCoords {
    let mut coords = WireCoords::new();
    let mut last_coord = (0, 0);
    for dir in directions.iter() {
        let direction = Direction::from(dir.chars().nth(0).unwrap());
        let distance = dir[1..].parse::<i64>().unwrap();

        last_coord = insert_coords(&mut coords, last_coord, direction, distance);
    }

    coords
}

fn insert_coords(
    coord_list: &mut WireCoords,
    mut last_coord: (i64, i64),
    direction: Direction,
    distance: i64,
) -> (i64, i64) {
    (1..=distance).for_each(|_| {
        match direction {
            Direction::Left => last_coord.0 -= 1,
            Direction::Up => last_coord.1 += 1,
            Direction::Right => last_coord.0 += 1,
            Direction::Down => last_coord.1 -= 1,
        }

        coord_list.insert(last_coord);
    });

    last_coord
}

fn compute_distance_to(point: (i64, i64), dir_list: &[&str]) -> i64 {
    let mut cursor = (0, 0);
    let mut distance = 0;

    for dir in dir_list {
        if cursor == point {
            break;
        }

        let direction = Direction::from(dir.chars().nth(0).unwrap());
        let dist = dir[1..].parse::<i64>().unwrap();

        (1..=dist).for_each(|_| {
            if cursor == point {
                return;
            }
            distance += 1;
            match direction {
                Direction::Left => cursor.0 -= 1,
                Direction::Up => cursor.1 += 1,
                Direction::Right => cursor.0 += 1,
                Direction::Down => cursor.1 -= 1,
            }
        });
    }

    distance
}
