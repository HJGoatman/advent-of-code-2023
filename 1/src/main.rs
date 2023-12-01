use env_logger;
use log;
use std::env;
use std::fs;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();

    let filtered_lines: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().filter(|char| char.is_numeric()).collect())
        .collect();

    log::debug!("Filtered lines: {:?}", filtered_lines);

    let calibration_values: Vec<u16> = filtered_lines
        .iter()
        .map(|unparsed| {
            let mut iter = unparsed.iter();

            let first_value = iter.nth(0);

            if first_value.is_none() {
                return None;
            }

            let first_value = first_value.unwrap();

            let value: String = [first_value, iter.last().unwrap_or(first_value)]
                .into_iter()
                .collect();

            Some(value)
        })
        .filter_map(|e| e)
        .filter_map(|unparsed| unparsed.parse::<u16>().ok())
        .collect();

    log::debug!("Calibrations values: {:?}", calibration_values);

    let calibration_values_sum: u16 = calibration_values.iter().sum();

    println!("{}", calibration_values_sum);
}
