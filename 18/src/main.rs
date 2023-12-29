mod colour;
mod dig_plan;
mod trench;

use dig_plan::DigPlan;
use trench::Trench;

use std::env;
use std::fs;

use crate::trench::Position;
use crate::trench::TrenchPart;

fn main() {
    env_logger::init();

    let input = load_input();

    let dig_plan: DigPlan = input.parse().unwrap();
    dig_plan
        .instructions
        .iter()
        .for_each(|instruction| log::debug!("{:?}", instruction));

    let mut trench = Trench::from(dig_plan);
    log::debug!("{}", trench);
    log::debug! {"{}", trench.dug_out_positions.len()};

    dig_out_interior(&mut trench);

    let trench_volume_cubic_meters = trench.dug_out_positions.len();
    println!("{}", trench_volume_cubic_meters);
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn dig_out_interior(trench: &mut Trench) {
    const START: Position = Position { y: 1, x: 1 };

    let mut stack = Vec::new();
    stack.push(START);

    while let Some(position) = stack.pop() {
        if trench.dug_out_positions.get(&position).is_some() {
            continue;
        } else {
            trench
                .dug_out_positions
                .insert(position, TrenchPart::Interior);
        }

        let new_positions = [
            Position {
                y: position.y - 1,
                x: position.x,
            },
            Position {
                y: position.y + 1,
                x: position.x,
            },
            Position {
                y: position.y,
                x: position.x - 1,
            },
            Position {
                y: position.y,
                x: position.x + 1,
            },
        ];

        new_positions.into_iter().for_each(|p| stack.push(p));
    }
}
