const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Debug)]
struct Layer {
    data: Vec<i32>,
}

fn main() {
    println!("{:?}", part1());
    part2();
}

fn part1() -> i32 {
    let layers = parse_input();

    let result = layers
        .iter()
        .enumerate()
        .map(|(idx, layer)| (idx, layer.data.iter().filter(|&&v| v == 0).count()))
        .min_by(|&(_, item1), &(_, item2)| item1.cmp(&item2))
        .unwrap()
        .0;

    let num_ones = layers[result].data.iter().filter(|&&v| v == 1).count();
    let num_twos = layers[result].data.iter().filter(|&&v| v == 2).count();

    (num_ones * num_twos) as i32
}

fn part2() {
    let layers = parse_input();

    let mut final_image = vec![2; WIDTH * HEIGHT];

    for layer in layers {
        for (idx, &color) in layer.data.iter().enumerate() {
            if final_image[idx] != 2 {
                continue;
            }

            final_image[idx] = color;
        }
    }

    let final_image = final_image
        .chunks(WIDTH)
        .map(|chunk| {
            chunk
                .iter()
                .map(|&digit| if digit == 1 { "X" } else { " " })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    for row in final_image {
        println!("{}", row);
    }
}

fn parse_input() -> Vec<Layer> {
    let mut layers = Vec::new();
    let digits: Vec<i32> = include_str!("day8.txt")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    for chunk in digits.chunks(WIDTH * HEIGHT) {
        layers.push(Layer { data: chunk.into() })
    }

    layers
}
