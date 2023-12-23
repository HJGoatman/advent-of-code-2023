use std::env;
use std::fs;
use std::str::FromStr;

type FocalLength = u8;

#[derive(Debug, Clone, Copy, Default)]
struct Lens {
    focal_length: FocalLength,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Remove,
    Insert(Lens),
}

#[derive(Debug)]
enum ParseOperationError {
    InvalidOperation,
    InvalidFocalLength,
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let operation = chars.next();

        match operation {
            Some('-') => Ok(Operation::Remove),
            Some('=') => {
                let focal_length = chars
                    .collect::<String>()
                    .parse()
                    .map_err(|_| ParseOperationError::InvalidFocalLength)?;

                let lens = Lens { focal_length };

                Ok(Operation::Insert(lens))
            }
            _ => Err(ParseOperationError::InvalidOperation),
        }
    }
}

#[derive(Debug, Clone)]
struct Step {
    label: String,
    operation: Operation,
}

#[derive(Debug)]
enum ParseStepError {
    ParseOperationError(ParseOperationError),
}

impl FromStr for Step {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opeation_index = s.chars().position(|c| c == '-' || c == '=').ok_or(
            ParseStepError::ParseOperationError(ParseOperationError::InvalidOperation),
        )?;

        let label = s[..opeation_index].to_string();
        let operation = s[opeation_index..]
            .parse()
            .map_err(ParseStepError::ParseOperationError)?;

        Ok(Step { label, operation })
    }
}

#[derive(Debug, Clone, Default)]
struct LabelledLens {
    label: String,
    lens: Lens,
}

#[derive(Debug, Clone, Default)]
struct Box {
    lens_slots: Vec<LabelledLens>,
}

impl Box {
    pub fn new() -> Box {
        let lens_slots = Vec::new();
        Box { lens_slots }
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

const NUMBER_OF_BOXES: usize = 256;

fn main() {
    env_logger::init();

    let input = load_input();
    log::debug!("{}", input);

    let results_sum: u32 = input
        .split(',')
        .map(|step| holiday_ascii_string_helper(step) as u32)
        .sum();

    println!("{}", results_sum);

    let instruction_sequence: Vec<Step> = input
        .split(',')
        .map(|step_str| step_str.parse().unwrap())
        .inspect(|step| log::debug!("{:?}", step))
        .collect();

    let mut boxes: [Box; NUMBER_OF_BOXES] = vec![Box::new(); NUMBER_OF_BOXES].try_into().unwrap();

    instruction_sequence.iter().fold(&mut boxes, |boxes, step| {
        holiday_ascii_string_helper_manual_arrangement_procedure(boxes, step.clone())
    });

    let focusing_power = calculate_focusing_power(&boxes);
    println!("{}", focusing_power);
}

fn calculate_focusing_power(boxes: &[Box; NUMBER_OF_BOXES]) -> u32 {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_number, selected_box)| {
            selected_box
                .lens_slots
                .iter()
                .enumerate()
                .map(move |(lens_slot, labelled_lens)| {
                    (1 + box_number as u32)
                        * (1 + lens_slot as u32)
                        * labelled_lens.lens.focal_length as u32
                })
        })
        .sum()
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

fn holiday_ascii_string_helper_manual_arrangement_procedure(
    boxes: &mut [Box; NUMBER_OF_BOXES],
    step: Step,
) -> &mut [Box; NUMBER_OF_BOXES] {
    let label = step.label;
    let operation = step.operation;

    let box_index = holiday_ascii_string_helper(&label) as usize;

    let selected_box = &mut boxes[box_index];

    match operation {
        Operation::Remove => {
            if let Some(index) = selected_box
                .lens_slots
                .iter()
                .position(|labelled_lens| labelled_lens.label == label.clone())
            {
                selected_box.lens_slots.remove(index);
            }

            boxes
        }
        Operation::Insert(lens) => {
            let labelled_lens = LabelledLens {
                label: label.clone(),
                lens,
            };

            if let Some(matched_label) = selected_box
                .lens_slots
                .iter()
                .position(|labelled_lens| labelled_lens.label == label.clone())
            {
                selected_box.lens_slots[matched_label] = labelled_lens;
            } else {
                selected_box.lens_slots.push(labelled_lens);
            }

            boxes
        }
    }
}
