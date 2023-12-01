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

    let first_values: Vec<Option<char>> = lines
        .iter()
        .map(|line| line.chars().find(|char| char.is_numeric()))
        .collect();
    let last_values: Vec<Option<char>> = lines
        .iter()
        .map(|line| line.chars().rev().find(|char| char.is_numeric()))
        .collect();

    let calibration_values_sum: u16 = get_calibration_values_sum(&first_values, &last_values);
    println!("{}", calibration_values_sum);

    let first_values: Vec<Option<char>> = lines.iter().map(|line| get_value(&line, true)).collect();
    let last_values: Vec<Option<char>> = lines.iter().map(|line| get_value(&line, false)).collect();

    let part_2_calibrations_sum: u16 = get_calibration_values_sum(&first_values, &last_values);
    println!("{}", part_2_calibrations_sum);
}

fn get_calibration_values_sum(first_values: &[Option<char>], last_values: &[Option<char>]) -> u16 {
    let calibration_value_pairs: Vec<String> = first_values
        .into_iter()
        .zip(last_values)
        .map(
            |(maybe_first, maybe_last)| match (maybe_first, maybe_last) {
                (Some(first), Some(last)) => Some([first, last].into_iter().collect::<String>()),
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

    calibration_values.iter().sum()
}

fn get_value(line: &str, get_first: bool) -> Option<char> {
    const SPELLED_OUT_DIGITS: [(&str, char); 10] = [
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let chars: Vec<char> = match get_first {
        true => line.chars().collect(),
        false => line.chars().rev().collect(),
    };

    for i in 0..chars.len() {
        let maybe_digit = chars.get(i).unwrap();

        if maybe_digit.is_numeric() {
            return Some(*maybe_digit);
        }

        for (spelled_out_digit, numeric_digit) in SPELLED_OUT_DIGITS.iter() {
            let spelled_out_digit_len = spelled_out_digit.len();

            if i + spelled_out_digit_len > chars.len() {
                continue;
            }

            let possible_spelled_out_digit: &[char] = &chars[i..i + spelled_out_digit_len];
            let spelled_out_digit: Vec<char> = match get_first {
                true => spelled_out_digit.chars().collect(),
                false => spelled_out_digit.chars().rev().collect(),
            };

            if spelled_out_digit == possible_spelled_out_digit {
                return Some(*numeric_digit);
            }
        }
    }

    None
}
