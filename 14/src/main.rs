mod platform;

use platform::Platform;
use platform::Position;
use platform::Space;
use platform::TiltDirection;
use platform::TiltResult;

use std::collections::HashMap;
use std::env;
use std::fs;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let platform: Platform = input.parse().unwrap();
    let mut part_1_platform = platform.clone();
    log::debug!("{}", platform);

    while part_1_platform.tilt(TiltDirection::North) == TiltResult::RocksMoved {}

    log::debug!("{}", part_1_platform);

    let total_load = calculate_total_load(&part_1_platform);
    println!("{}", total_load);

    let mut part_2_platform = platform.clone();

    const NUMBER_OF_CYCLES: usize = 1000000000;

    spin_platform(&mut part_2_platform, NUMBER_OF_CYCLES);

    let total_load = calculate_total_load(&part_2_platform);
    println!("{}", total_load);
}

fn spin_platform(platform: &mut Platform, number_of_cycles: usize) {
    let mut cache = HashMap::new();

    let mut i = 0;
    while i < number_of_cycles {
        platform.spin_cycle();

        if let Some(old_i) = cache.get(platform) {
            log::trace!("i: {}, old_i: {}", i, old_i);
            let i_difference = i - old_i;

            let remaining_cycles = number_of_cycles - i;

            let new_i = number_of_cycles - (remaining_cycles % i_difference);
            i = new_i
        } else {
            cache.insert(platform.clone(), i);
        }

        i += 1;
    }
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
