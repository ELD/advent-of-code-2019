use std::collections::HashMap;

type OrbitList = HashMap<&'static str, &'static str>;

pub fn part1() -> i32 {
    let orbit_list = parse_input();

    count_orbits(&orbit_list)
}

pub fn part2() -> i32 {
    let orbit_list = parse_input();

    minimum_orbits(&orbit_list, "YOU", "SAN")
}

pub(crate) fn parse_input() -> OrbitList {
    include_str!("inputs/day6.txt")
        .trim()
        .split('\n')
        .map(|orbit| {
            let mut split = orbit.split(')');
            let body = split.next().unwrap();
            let satellite = split.next().unwrap();

            (satellite, body)
        })
        .collect::<OrbitList>()
}

pub(crate) fn count_orbits(orbits: &OrbitList) -> i32 {
    orbits.keys().fold(0, |acc, body| {
        let mut orbit_count = 0;
        let mut origin = *body;

        while orbits.contains_key(origin) {
            orbit_count += 1;
            origin = orbits.get(origin).expect("no orbit?");
        }

        acc + orbit_count
    })
}

pub(crate) fn minimum_orbits(orbits: &OrbitList, start: &'static str, end: &'static str) -> i32 {
    let destinations_from_start = travel_from(orbits, start);
    let destinations_from_end = travel_from(orbits, end);

    for (i, p1) in destinations_from_start.iter().enumerate() {
        for (j, p2) in destinations_from_end.iter().enumerate() {
            if p2 == p1 {
                return (i + j) as i32;
            }
        }
    }

    0
}

fn travel_from(orbits: &OrbitList, start: &'static str) -> Vec<&'static str> {
    let mut travel_list = Vec::new();
    let mut current = start;

    while let Some(value) = orbits.get(current) {
        travel_list.push(*value);
        current = value;
    }

    travel_list
}

#[cfg(test)]
mod test {
    use crate::day6::*;

    #[test]
    fn counts_orbits_small() {
        let expected = 42;
        let mut orbit_list = OrbitList::new();
        orbit_list.insert("B", "COM");
        orbit_list.insert("C", "B");
        orbit_list.insert("D", "C");
        orbit_list.insert("E", "D");
        orbit_list.insert("F", "E");
        orbit_list.insert("G", "B");
        orbit_list.insert("H", "G");
        orbit_list.insert("I", "D");
        orbit_list.insert("J", "E");
        orbit_list.insert("K", "J");
        orbit_list.insert("L", "K");

        let actual = count_orbits(&orbit_list);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_shortest_transfer() {
        let expected = 4;
        let mut orbit_list = OrbitList::new();
        orbit_list.insert("SAN", "I");
        orbit_list.insert("YOU", "K");
        orbit_list.insert("L", "K");
        orbit_list.insert("K", "J");
        orbit_list.insert("J", "E");
        orbit_list.insert("I", "D");
        orbit_list.insert("H", "G");
        orbit_list.insert("G", "B");
        orbit_list.insert("F", "E");
        orbit_list.insert("E", "D");
        orbit_list.insert("D", "C");
        orbit_list.insert("C", "B");
        orbit_list.insert("B", "COM");

        let actual = minimum_orbits(&orbit_list, "YOU", "SAN");

        assert_eq!(expected, actual);
    }
}
