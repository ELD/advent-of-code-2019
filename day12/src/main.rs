use lazy_static::lazy_static;
use num_integer::lcm;
use regex::Regex;
use std::cmp::Ordering;

lazy_static! {
    static ref PLANET_REGEX: Regex = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Moon {
    position: Point,
    velocity: Point,
}

impl Moon {
    fn calculate_gravity(rhs: &Moon, lhs: &Moon) -> (i32, i32, i32) {
        (
            match rhs.position.x.cmp(&lhs.position.x) {
                Ordering::Greater => -1,
                Ordering::Equal => 0,
                Ordering::Less => 1,
            },
            match rhs.position.y.cmp(&lhs.position.y) {
                Ordering::Greater => -1,
                Ordering::Equal => 0,
                Ordering::Less => 1,
            },
            match rhs.position.z.cmp(&lhs.position.z) {
                Ordering::Greater => -1,
                Ordering::Equal => 0,
                Ordering::Less => 1,
            },
        )
    }

    fn adjust_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn calculate_energy(&self) -> i32 {
        let potential_energy =
            self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        let kinetic_energy = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();

        potential_energy * kinetic_energy
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0, z: 0 }
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i32 {
    let mut moons = parse_input();

    (0..1000).for_each(|_| {
        step_time(&mut moons);
    });

    moons.iter().map(|moon| moon.calculate_energy()).sum()
}

fn part2() -> i64 {
    let moons = parse_input();

    let x_init = moons.iter().map(|moon| moon.position.x).collect::<Vec<_>>();
    let y_init = moons.iter().map(|moon| moon.position.y).collect::<Vec<_>>();
    let z_init = moons.iter().map(|moon| moon.position.z).collect::<Vec<_>>();

    let p_x = find_period(&x_init);
    let p_y = find_period(&y_init);
    let p_z = find_period(&z_init);

    lcm(p_x, lcm(p_y, p_z))
}

fn parse_input() -> Vec<Moon> {
    include_str!("day12.txt")
        .trim()
        .lines()
        .map(|pos| PLANET_REGEX.captures(pos).unwrap())
        .map(|regex_captures_iter| {
            let x = regex_captures_iter
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let y = regex_captures_iter
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let z = regex_captures_iter
                .get(3)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();

            Moon {
                position: Point { x, y, z },
                velocity: Point { x: 0, y: 0, z: 0 },
            }
        })
        .collect()
}

fn step_time(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in (i + 1)..moons.len() {
            let (dx, dy, dz) = Moon::calculate_gravity(&moons[i], &moons[j]);
            moons[i].velocity.x += dx;
            moons[j].velocity.x -= dx;
            moons[i].velocity.y += dy;
            moons[j].velocity.y -= dy;
            moons[i].velocity.z += dz;
            moons[j].velocity.z -= dz;
        }
    }

    moons.iter_mut().for_each(|moon| {
        moon.adjust_position();
    })
}

fn find_period(axis_init: &[i32]) -> i64 {
    let init_state = axis_init.iter().map(|&pos| (pos, 0)).collect::<Vec<_>>();
    let mut moons = init_state.clone();

    let mut count = 0;
    loop {
        for i in 0..init_state.len() {
            for j in (i + 1)..init_state.len() {
                let (d, _, _) = Moon::calculate_gravity(
                    &Moon {
                        position: Point {
                            x: moons[i].0,
                            y: 0,
                            z: 0,
                        },
                        velocity: Point::default(),
                    },
                    &Moon {
                        position: Point {
                            x: moons[j].0,
                            y: 0,
                            z: 0,
                        },
                        velocity: Point::default(),
                    },
                );

                moons[i].1 += d;
                moons[j].1 -= d;
            }
            moons[i].0 += moons[i].1;
        }

        if moons == init_state {
            break;
        }
        count += 1;
    }

    count + 1
}

#[cfg(test)]
mod test {
    use crate::*;

    fn setup_input() -> Vec<Moon> {
        let init_velocity = Point { x: 0, y: 0, z: 0 };
        vec![
            Moon {
                position: Point { x: -1, y: 0, z: 2 },
                velocity: init_velocity,
            },
            Moon {
                position: Point {
                    x: 2,
                    y: -10,
                    z: -7,
                },
                velocity: init_velocity,
            },
            Moon {
                position: Point { x: 4, y: -8, z: 8 },
                velocity: init_velocity,
            },
            Moon {
                position: Point { x: 3, y: 5, z: -1 },
                velocity: init_velocity,
            },
        ]
    }

    #[test]
    fn calculates_velocity_for_one_time_step() {
        let mut moons = setup_input();
        let expected = vec![
            Point { x: 2, y: -1, z: 1 },
            Point { x: 3, y: -7, z: -4 },
            Point { x: 1, y: -7, z: 5 },
            Point { x: 2, y: 2, z: 0 },
        ];

        step_time(&mut moons);

        for (&expected_pos, moon) in expected.iter().zip(moons) {
            assert_eq!(moon.position, expected_pos);
        }
    }

    #[test]
    fn calculate_total_energy() {
        //        After 100 steps:
        //        pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>
        //        pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>
        //        pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>
        //        pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>

        let moons = vec![
            Moon {
                position: Point {
                    x: 8,
                    y: -12,
                    z: -9,
                },
                velocity: Point { x: -7, y: 3, z: 0 },
            },
            Moon {
                position: Point {
                    x: 13,
                    y: 16,
                    z: -3,
                },
                velocity: Point {
                    x: 3,
                    y: -11,
                    z: -5,
                },
            },
            Moon {
                position: Point {
                    x: -29,
                    y: -11,
                    z: -1,
                },
                velocity: Point { x: -3, y: 7, z: 4 },
            },
            Moon {
                position: Point {
                    x: 16,
                    y: -13,
                    z: 23,
                },
                velocity: Point { x: 7, y: 1, z: 1 },
            },
        ];
        let expected = 1940;

        let actual: i32 = moons.iter().map(|moon| moon.calculate_energy()).sum();

        assert_eq!(actual, expected);
    }
}
