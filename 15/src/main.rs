use std::env;
use std::fs;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    log::debug!("{}", input);

    let results_sum: u32 = input
        .split(',')
        .map(|step| holiday_ascii_string_helper(step) as u32)
        .sum();

    println!("{}", results_sum);
}

fn holiday_ascii_string_helper(input: &str) -> u16 {
    input.chars().fold(0, |mut current_value, c| {
        let ascii_code = c as u16;
        current_value += ascii_code;
        current_value *= 17;
        current_value %= 256;
        current_value
    })
}
