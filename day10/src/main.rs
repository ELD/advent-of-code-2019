use gcd::Gcd;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i32 {
    let asteroids = parse_input();

    analyze(&asteroids)
        .iter()
        .map(|(_, coords)| {
            coords
                .iter()
                .map(|&(_, simple)| simple)
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap() as i32
}

fn part2() -> i32 {
    let asteroids = parse_input();

    let (_, (origin_x, origin_y), mut deltas) = analyze(&asteroids)
        .into_iter()
        .map(|((x, y), plane)| {
            (
                plane
                    .iter()
                    .map(|(_, can)| can)
                    .collect::<HashSet<_>>()
                    .len(),
                (x, y),
                plane
                    .into_iter()
                    .map(|(delta, _)| delta)
                    .collect::<Vec<_>>(),
            )
        })
        .max()
        .unwrap();

    deltas.sort_by(|&(x1, y1), &(x2, y2)| {
        let (x1s, y1s) = simplest_slope(x1, y1);
        let (x2s, y2s) = simplest_slope(x2, y2);
        let first = (
            if x1 == 0 && y1 < 0 {
                -4.0
            } else {
                (-x1s as f32).atan2(y1s as f32)
            },
            x1 * x1 + y1 * y1,
        );
        let second = (
            if x2 == 0 && y2 < 0 {
                -4.0
            } else {
                (-x2s as f32).atan2(y2s as f32)
            },
            x2 * x2 + y2 * y2,
        );

        first.partial_cmp(&second).unwrap()
    });

    let grouped_deltas = deltas
        .into_iter()
        .group_by(|&(x, y)| simplest_slope(x, y))
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    for depth in 0..grouped_deltas
        .iter()
        .map(|group| group.len())
        .max()
        .unwrap()
    {
        for asteroid in &grouped_deltas {
            if asteroid.len() > depth {
                count += 1;
                if count == 200 {
                    return (asteroid[depth].0 + origin_x) * 100 + asteroid[depth].1 + origin_y;
                }
            }
        }
    }

    0
}

fn analyze(coords: &[(i32, i32)]) -> Vec<((i32, i32), Vec<((i32, i32), (i32, i32))>)> {
    coords
        .iter()
        .map(|&(x, y)| {
            (
                (x, y),
                coords
                    .iter()
                    .filter_map(|&(other_x, other_y)| {
                        if other_x == x && other_y == y {
                            None
                        } else {
                            Some((
                                (other_x - x, other_y - y),
                                simplest_slope(other_x - x, other_y - y),
                            ))
                        }
                    })
                    .collect(),
            )
        })
        .collect()
}

fn parse_input() -> Vec<(i32, i32)> {
    include_str!("day10.txt")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(idy, line)| {
            line.chars().enumerate().filter_map(move |(idx, c)| {
                if c == '#' {
                    Some((idx as i32, idy as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn simplest_slope(x: i32, y: i32) -> (i32, i32) {
    if x == 0 {
        (0, y.signum())
    } else if y == 0 {
        (x.signum(), 0)
    } else {
        let gcd = (x.abs() as u32).gcd(y.abs() as u32) as i32;
        (x / gcd, y / gcd)
    }
}
