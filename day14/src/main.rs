use nom::{
    bytes::complete::tag,
    character::{complete::alpha1, complete::digit1, complete::space0, complete::space1},
    combinator::{map, map_res},
    multi::separated_list,
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use num_integer::Integer;
use std::cmp::Ordering;
use std::{collections::HashMap, str::FromStr};

type Quantity = i64;
type Material = String;
type Reactions = HashMap<String, Reaction>;
type SurplusQuantities = HashMap<Material, Quantity>;

#[derive(Debug)]
struct Reaction {
    product: Material,
    quantity: i64,
    deps: Vec<Reactant>,
}

#[derive(Clone, Debug)]
struct Reactant {
    material: Material,
    quantity: i64,
}

fn main() {
    parse_input();
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i64 {
    let reactions = parse_input();

    let mut surplus = SurplusQuantities::new();
    count_reactants(&reactions, 1, "FUEL".to_string(), &mut surplus)
}

fn part2() -> i64 {
    let reactions = parse_input();
    let mut surplus = SurplusQuantities::new();
    let max_ore = 1_000_000_000_000;

    let mut min_fuel = 1;
    let mut max_fuel = count_reactants(&reactions, min_fuel, "FUEL".to_string(), &mut surplus);

    while min_fuel <= max_fuel {
        let mid_fuel = min_fuel + (max_fuel - min_fuel) / 2;
        let ore_count = count_reactants(&reactions, mid_fuel, "FUEL".to_string(), &mut surplus);

        match ore_count.cmp(&max_ore) {
            Ordering::Equal => {
                max_fuel = mid_fuel;
                break;
            }
            Ordering::Greater => max_fuel = mid_fuel - 1,
            Ordering::Less => min_fuel = mid_fuel + 1,
        }

        surplus.clear();
    }

    max_fuel
}

fn count_reactants(
    reactions: &Reactions,
    mut quantity: i64,
    product: String,
    surplus_quantities: &mut SurplusQuantities,
) -> i64 {
    if product == "ORE" {
        return quantity;
    }

    let surplus = surplus_quantities.entry(product.clone()).or_default();
    if *surplus >= quantity {
        *surplus -= quantity;
        return 0;
    } else {
        quantity -= *surplus;
        *surplus = 0;
    }

    let reaction = reactions.get(&product).unwrap();
    let num_reactions_required = quantity.div_ceil(&reaction.quantity);
    let surplus_produced = reaction.quantity * num_reactions_required - quantity;

    *surplus_quantities.entry(product).or_default() += surplus_produced;

    reaction
        .deps
        .iter()
        .map(|dep| {
            count_reactants(
                reactions,
                dep.quantity * num_reactions_required,
                dep.material.clone(),
                surplus_quantities,
            )
        })
        .sum()
}

fn parse_input() -> Reactions {
    let input = include_str!("day14.txt").trim();

    input
        .lines()
        .filter_map(|line| reaction(line).ok())
        .map(|reaction| reaction.1)
        .collect()
}

fn reaction(input: &'static str) -> IResult<&'static str, (String, Reaction)> {
    map(
        separated_pair(
            reactant_list,
            delimited(space0, tag("=>"), space0),
            reactant,
        ),
        |(reactants, output)| {
            (
                output.material.clone(),
                Reaction {
                    product: output.material,
                    quantity: output.quantity,
                    deps: reactants,
                },
            )
        },
    )(input)
}

fn reactant(input: &str) -> IResult<&str, Reactant> {
    map(
        separated_pair(map_res(digit1, i64::from_str), space1, alpha1),
        |(quantity, material)| Reactant {
            material: material.to_string(),
            quantity,
        },
    )(input)
}

fn reactant_list(input: &str) -> IResult<&str, Vec<Reactant>> {
    separated_list(pair(tag(","), space0), reactant)(input)
}
