use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
use std::usize;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct ConditionRecord {
    format_1: Vec<Condition>,
    format_2: Vec<usize>,
}

#[derive(Debug)]
enum ParseConditionRecordError {
    UnknownConditionType,
    ParseIntError(ParseIntError),
}

impl FromStr for ConditionRecord {
    type Err = ParseConditionRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let format_1 = split.next().unwrap();
        let format_2 = split.next().unwrap();

        let format_1_parse_result: Result<Vec<Condition>, ParseConditionRecordError> = format_1
            .chars()
            .map(|c| match c {
                '.' => Ok(Condition::Operational),
                '#' => Ok(Condition::Damaged),
                '?' => Ok(Condition::Unknown),
                _ => Err(ParseConditionRecordError::UnknownConditionType),
            })
            .collect();

        let format_1 = format_1_parse_result?;

        let format_2: Result<Vec<usize>, ParseConditionRecordError> = format_2
            .split(',')
            .map(|s| s.parse().map_err(ParseConditionRecordError::ParseIntError))
            .collect();
        let format_2 = format_2?;

        Ok(ConditionRecord { format_1, format_2 })
    }
}

impl Display for ConditionRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format_1: String = self
            .format_1
            .iter()
            .map(|condition| match condition {
                Condition::Operational => '.',
                Condition::Damaged => '#',
                Condition::Unknown => '?',
            })
            .collect();

        let format_2 = self
            .format_2
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",");

        f.write_fmt(format_args!("{} {}", format_1, format_2))?;
        Ok(())
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let condition_records: Vec<ConditionRecord> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.parse())
        .collect::<Result<Vec<ConditionRecord>, ParseConditionRecordError>>()
        .unwrap();

    let mut cache = HashMap::new();

    let arrangements: Vec<usize> = condition_records
        .iter()
        .map(|record| find_possible_arrangements(&mut cache, &record.format_1, &record.format_2))
        .inspect(|arrangements| log::debug!("Arrangements: {}", arrangements))
        .collect();

    let arrangements_sum: usize = arrangements.iter().sum();
    println!("{}", arrangements_sum);

    let unfolded_condition_records: Vec<ConditionRecord> = condition_records
        .iter()
        .map(unfold_condition_record)
        .collect();

    let arrangements: Vec<usize> = unfolded_condition_records
        .iter()
        .map(|record| find_possible_arrangements(&mut cache, &record.format_1, &record.format_2))
        .inspect(|arrangements| log::debug!("Arrangements: {}", arrangements))
        .collect();

    let arrangements_sum: usize = arrangements.iter().sum();
    println!("{}", arrangements_sum);
}

fn unfold_condition_record(condition_record: &ConditionRecord) -> ConditionRecord {
    let mut format_1 = Vec::new();
    let mut format_2 = Vec::new();

    for i in 0..5 {
        format_1.append(&mut condition_record.format_1.to_vec());
        format_2.append(&mut condition_record.format_2.to_vec());

        if i < 4 {
            format_1.push(Condition::Unknown);
        }
    }

    ConditionRecord { format_1, format_2 }
}

fn find_possible_arrangements(
    cache: &mut HashMap<(Vec<Condition>, Vec<usize>), usize>,
    record: &[Condition],
    criteria: &[usize],
) -> usize {
    log::trace!(
        "{}",
        ConditionRecord {
            format_1: record.to_vec(),
            format_2: criteria.to_vec()
        }
    );

    if let Some(result) = cache.get(&(record.to_vec(), criteria.to_vec())) {
        log::debug!("Found Cached result!");
        return *result;
    }

    let compressed_record = compress_record(record);
    if compressed_record.len() != record.len() {
        log::trace!("Reducing!");

        return find_and_cache(cache, &compressed_record, criteria);
    }

    let maybe_current_criteria = calculate_current_criteria(record);

    log::trace!("{:?}", maybe_current_criteria);

    match maybe_current_criteria {
        CalculateCriteriaResult::Full(current_criteria) => {
            log::trace!("{:?}", current_criteria);

            if &current_criteria == criteria {
                return 1;
            } else {
                return 0;
            }
        }
        CalculateCriteriaResult::PartialIncomplete(current_criteria) => {
            if !current_criteria.is_empty()
                && !criteria.is_empty()
                && (current_criteria.len() <= criteria.len())
            {
                let last_index = current_criteria.len() - 1;
                if current_criteria
                    .iter()
                    .zip(criteria[..last_index].iter())
                    .any(|(a, b)| a != b)
                {
                    return 0;
                }

                if current_criteria[last_index] > criteria[last_index] {
                    return 0;
                }
            }
        }
        CalculateCriteriaResult::PartialComplete(current_criteria, splitting_index) => {
            if !current_criteria.is_empty() {
                if current_criteria.len() > criteria.len() {
                    return 0;
                }

                if current_criteria == criteria[..current_criteria.len()] {
                    return find_and_cache(
                        cache,
                        &record[splitting_index..],
                        &criteria[current_criteria.len()..],
                    );
                } else {
                    return 0;
                }
            }
        }
    };

    let unknown_index = record
        .iter()
        .position(|condition| *condition == Condition::Unknown)
        .unwrap();

    let mut damaged_branch = record.to_vec();
    damaged_branch[unknown_index] = Condition::Damaged;

    let mut operational_branch = record.to_vec();
    operational_branch[unknown_index] = Condition::Operational;

    return find_and_cache(cache, &damaged_branch, criteria)
        + find_and_cache(cache, &operational_branch, criteria);
}

fn find_and_cache(
    cache: &mut HashMap<(Vec<Condition>, Vec<usize>), usize>,
    record: &[Condition],
    criteria: &[usize],
) -> usize {
    let result = find_possible_arrangements(cache, record, criteria);
    cache.insert((record.to_vec(), criteria.to_vec()), result);
    result
}

#[derive(Debug)]
enum CalculateCriteriaResult {
    Full(Vec<usize>),
    PartialComplete(Vec<usize>, usize),
    PartialIncomplete(Vec<usize>),
}

fn calculate_current_criteria(format_1: &[Condition]) -> CalculateCriteriaResult {
    let mut criteria = Vec::new();
    let mut num_contingous_damaged_springs = 0;

    for (i, condition) in format_1.iter().enumerate() {
        match condition {
            Condition::Unknown => {
                if num_contingous_damaged_springs > 0 {
                    criteria.push(num_contingous_damaged_springs);
                    return CalculateCriteriaResult::PartialIncomplete(criteria);
                }
                return CalculateCriteriaResult::PartialComplete(criteria, i);
            }
            Condition::Damaged => {
                num_contingous_damaged_springs += 1;
            }
            Condition::Operational => {
                if num_contingous_damaged_springs > 0 {
                    criteria.push(num_contingous_damaged_springs);
                    num_contingous_damaged_springs = 0;
                }
            }
        }
    }

    if num_contingous_damaged_springs > 0 {
        criteria.push(num_contingous_damaged_springs);
    }

    CalculateCriteriaResult::Full(criteria)
}

fn compress_record(record: &[Condition]) -> Vec<Condition> {
    let mut reduced: Vec<Condition> = record.iter().fold(vec![], |mut current, next| {
        if *next == Condition::Operational {
            if let Some(last) = current.last() {
                if *last == Condition::Operational {
                    return current;
                }
            }
        }

        current.push(*next);
        current
    });

    reduced = reduced
        .into_iter()
        .rev()
        .skip_while(|v| *v == Condition::Operational)
        .collect::<Vec<Condition>>()
        .into_iter()
        .rev()
        .collect();

    reduced
}
