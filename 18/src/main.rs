mod colour;
mod dig_plan;
mod trench;

use colour::Colour;
use dig_plan::DigInstruction;
use dig_plan::DigPlan;
use dig_plan::Direction;
use trench::Edge;
use trench::Position;
use trench::Trench;

use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let input = load_input();

    let dig_plan: DigPlan = input.parse().unwrap();
    dig_plan
        .instructions
        .iter()
        .for_each(|instruction| log::debug!("{:?}", instruction));

    let mut trench = Trench::from(&dig_plan);
    log::debug!("{}", trench);
    log::debug! {"{}", trench.edges.len()};

    let trench_volume_cubic_meters = get_total_volume(&mut trench);
    println!("{}", trench_volume_cubic_meters);

    let corrected_dig_plan = extract_correct_instructions(&dig_plan);
    corrected_dig_plan
        .instructions
        .iter()
        .for_each(|instruction| log::debug!("{:?}", instruction));

    let mut trench = Trench::from(&corrected_dig_plan);
    let trench_volume_cubic_meters = get_total_volume(&mut trench);
    println!("{}", trench_volume_cubic_meters);
}

fn extract_correct_instructions(dig_plan: &DigPlan) -> DigPlan {
    let instructions = dig_plan
        .instructions
        .iter()
        .map(|instruction| extract_correct_instruction(instruction))
        .collect();

    DigPlan { instructions }
}

fn extract_correct_instruction(instruction: &DigInstruction) -> DigInstruction {
    let Colour::RGB(red, green, blue) = instruction.colour;
    let hex_int: u64 = ((red as u64) << (2 * 8)) | ((green as u64) << (8)) | blue as u64;

    let hex = format!("{hex_int:#08x}");

    let amount = u64::from_str_radix(&hex[2..7], 16).unwrap();
    let direction_num = u64::from_str_radix(&hex[7..], 16).unwrap();
    let direction = match direction_num {
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Up,
        _ => panic!("Err"),
    };

    return DigInstruction {
        direction,
        amount,
        colour: Colour::RGB(red, green, blue),
    };
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn get_total_volume(trench: &Trench) -> u64 {
    let number_of_boundary_points = trench.edges.iter().map(|edge| edge.length).sum();
    let area = shoelace_formula(&trench.edges);

    let number_of_internal_points =
        calculate_number_of_internal_points(area, number_of_boundary_points);

    number_of_boundary_points + number_of_internal_points
}

fn calculate_number_of_internal_points(area: u64, number_of_boundary_points: u64) -> u64 {
    // Pick's Theorem with help from aoc subreddit
    area + 1 - (number_of_boundary_points / 2)
}

fn shoelace_formula(edges: &[Edge]) -> u64 {
    let anti_clockwise_positions = get_anti_clockwise_positions(edges);

    let (xs, ys): (Vec<i64>, Vec<i64>) = anti_clockwise_positions
        .into_iter()
        .map(|position| (position.x, position.y))
        .unzip();

    let sum_1: i64 = xs
        .iter()
        .zip(ys.iter().cycle().skip(1))
        .map(|(x, y)| x * y)
        .sum();
    let sum_2: i64 = ys
        .iter()
        .zip(xs.iter().cycle().skip(1))
        .map(|(y, x)| x * y)
        .sum();

    (sum_1 - sum_2).abs() as u64 / 2
}

fn get_anti_clockwise_positions(edges: &[Edge]) -> Vec<Position> {
    let mut anti_clockwise_positions = Vec::new();

    let start = edges.first().unwrap().start;

    const ANTI_CLOCKWISE_DIRECTION_ORDER: [Direction; 4] = [
        Direction::Left,
        Direction::Down,
        Direction::Right,
        Direction::Up,
    ];

    let mut current_position = start;

    'outer: loop {
        anti_clockwise_positions.push(current_position);

        for direction in ANTI_CLOCKWISE_DIRECTION_ORDER {
            if let Some(edge) = edges
                .iter()
                .find(|edge| (edge.start == current_position) && edge.direction == direction)
            {
                if !anti_clockwise_positions.contains(&edge.end) {
                    current_position = edge.end;
                    continue 'outer;
                }
            }

            if let Some(edge) = edges.iter().find(|edge| {
                (edge.end == current_position) && reversed_direction(edge.direction) == direction
            }) {
                if !anti_clockwise_positions.contains(&edge.start) {
                    current_position = edge.start;
                    continue 'outer;
                }
            }
        }

        if anti_clockwise_positions.len() >= edges.len() {
            break;
        }

        panic!("positions not connected!");
    }

    anti_clockwise_positions
}

fn reversed_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}
