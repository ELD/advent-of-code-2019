pub fn part1() -> i64 {
    parse_input().fold(0, |acc, x| acc + x / 3 - 2)
}

pub fn part2() -> i64 {
    parse_input().map(calc_fuel).sum()
}

fn calc_fuel(mass: i64) -> i64 {
    if mass == 0 {
        return mass;
    }

    let fuel = std::cmp::max(mass / 3 - 2, 0);

    fuel + calc_fuel(fuel)
}

fn parse_input() -> impl Iterator<Item = i64> {
    include_str!("inputs/day1.txt")
        .trim()
        .split('\n')
        .map(|input| input.parse::<i64>().expect("unable to parse"))
}
