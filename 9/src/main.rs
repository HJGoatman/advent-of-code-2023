use std::env;
use std::fs;
use std::num::ParseIntError;

use factorial::Factorial;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let lines: Vec<String> = input.split('\n').map(|line| line.to_string()).collect();
    log::debug!("{:?}", lines);

    let sequences = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse())
                .collect::<Result<Vec<i128>, ParseIntError>>()
        })
        .collect::<Result<Vec<Vec<i128>>, ParseIntError>>()
        .unwrap();

    log::debug!("{:?}", sequences);

    let sum_of_next_values: i128 = sequences
        .iter()
        .map(|sequence| find_next_value(sequence))
        .inspect(|v| log::debug!("{}", v))
        .sum();

    println!("{}", sum_of_next_values);

    let reverse_sequences = sequences
        .iter()
        .cloned()
        .map(|seq| seq.into_iter().rev().collect::<Vec<i128>>())
        .collect::<Vec<Vec<i128>>>();

    let sum_of_previous_values: i128 = reverse_sequences
        .iter()
        .map(|sequence| find_next_value(sequence))
        .inspect(|v| log::debug!("{}", v))
        .sum();

    println!("{}", sum_of_previous_values)
}

fn find_next_value(sequence: &[i128]) -> i128 {
    let mut differences: Vec<Vec<i128>> = Vec::new();

    let mut most_recent_sequence: Vec<i128> = sequence.to_vec();
    while !most_recent_sequence.iter().all(|v| *v == 0) {
        let mut difference_col = Vec::new();

        for i in 0..most_recent_sequence.len() - 1 {
            let difference = most_recent_sequence[i + 1] - most_recent_sequence[i];

            difference_col.push(difference);
        }

        differences.push(most_recent_sequence);
        most_recent_sequence = difference_col;
    }

    log::debug!("{:?}", differences);

    let d_1: Vec<i128> = differences
        .iter()
        .map(|seq| seq.first().unwrap())
        .cloned()
        .collect();

    log::debug!("D: {:?}", d_1);

    let polnominal_degree = d_1.len();
    log::debug!("k: {}", polnominal_degree);

    let f = |n: i128| -> i128 {
        (0..(polnominal_degree))
            .inspect(|i| log::trace!("\ti: {}", i))
            .map(|i: usize| -> i128 {
                d_1[i] * (0..i).map(|a| n - a as i128).product::<i128>() / i.factorial() as i128
            })
            .inspect(|v| log::trace!("\t{}", v))
            .sum::<i128>()
    };

    f(sequence.len() as i128)
}
