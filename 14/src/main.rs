mod platform;

use platform::Platform;
use platform::Position;
use platform::Space;
use platform::TiltDirection;
use platform::TiltResult;

use std::env;
use std::fs;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let mut platform: Platform = input.parse().unwrap();
    log::debug!("{}", platform);

    while platform.tilt(TiltDirection::North) == TiltResult::RocksMoved {}

    log::debug!("{}", platform);

    let total_load = calculate_total_load(&platform);
    println!("{}", total_load);
}

fn calculate_total_load(platform: &Platform) -> u32 {
    let mut total_load = 0;

    let platform_height = platform.get_height();
    for y in 0..platform.get_height() {
        for x in 0..platform.get_width() {
            if platform.get(Position { x, y }) == Some(Space::RoundedRock) {
                let rows_to_south_wall = platform_height - y;
                total_load += rows_to_south_wall as u32;
            }
        }
    }

    total_load
}
