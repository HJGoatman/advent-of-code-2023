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
    let lines: Vec<String> = input.split('\n').map(|line| line.to_string()).collect();

    let first_values = lines
        .iter()
        .map(|line| line.chars().find(|char| char.is_numeric()));
    let last_values = lines
        .iter()
        .map(|line| line.chars().rev().find(|char| char.is_numeric()));

    let calibration_value_pairs: Vec<String> = first_values
        .zip(last_values)
        .map(
            |(maybe_first, maybe_last)| match (maybe_first, maybe_last) {
                (Some(first), Some(last)) => Some([first, last].iter().collect::<String>()),
                (_, _) => None,
            },
        )
        .flat_map(|e| e)
        .collect();

    log::debug!("Value Pairs: {:?}", calibration_value_pairs);

    let calibration_values: Vec<u16> = calibration_value_pairs
        .iter()
        .filter_map(|unparsed| unparsed.parse().ok())
        .collect();

    log::debug!("Calibrations values: {:?}", calibration_values);

    let calibration_values_sum: u16 = calibration_values.iter().sum();

    println!("{}", calibration_values_sum);
}
