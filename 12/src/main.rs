
use itertools::Itertools;

use std::env;
use std::fs;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;
use std::usize;

use crate::target_contingous_record_iterator::TargetContingousRecordIterator;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

mod target_contingous_record_iterator {
    use itertools::Itertools;
    use std::{ops::Range};

    use crate::Condition;

    pub struct TargetContingousRecordIterator {
        conditions: Vec<Condition>,
        iterator: Box<dyn Iterator<Item = Vec<usize>>>,
        max_conditions: usize,
    }

    impl TargetContingousRecordIterator {
        pub fn new(
            contingous_record: Vec<(Condition, Range<usize>)>,
            max_conditions: usize,
        ) -> TargetContingousRecordIterator {
            let (conditions, ranges): (Vec<Condition>, Vec<Range<usize>>) =
                contingous_record.into_iter().unzip();

            let iterator = Box::new(ranges.into_iter().multi_cartesian_product());

            TargetContingousRecordIterator {
                conditions,
                iterator,
                max_conditions,
            }
        }
    }

    impl Iterator for TargetContingousRecordIterator {
        type Item = Vec<(Condition, usize)>;

        fn next(&mut self) -> Option<Self::Item> {
            self.iterator
                .find(|values| values.iter().sum::<usize>() == self.max_conditions)
                .map(|values| self.conditions.iter().cloned().zip_eq(values).collect())
        }
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

    let arrangements: Vec<usize> = condition_records
        .iter()
        // .take(1)
        .inspect(|_| log::debug!(""))
        .inspect(|record| log::debug!("{:?}", record))
        .map(|record| find_possible_arrangements(&record.format_1, &record.format_2))
        .inspect(|arrangements| log::debug!("Arragments: {}", arrangements))
        .collect();

    let arrangements_sum: usize = arrangements.iter().sum();
    println!("{}", arrangements_sum);
}

fn find_possible_arrangements(damaged_record: &[Condition], criteria: &[usize]) -> usize {
    let contingous_record = convert_to_contingous_record(damaged_record);
    let target_contingous_record =
        calculate_target_contingous_record(criteria, damaged_record.len() + 1);
    log::debug!("Contingous Record: {:?}", contingous_record);
    log::debug!("Target Contingous Record: {:?}", target_contingous_record);

    let target_contingous_record_iterator =
        TargetContingousRecordIterator::new(target_contingous_record, damaged_record.len());

    let mut possible_arrangements = 0;

    for record in target_contingous_record_iterator {
        let unstacked = unstack_record(record);

        let is_match =
            damaged_record
                .iter()
                .zip_eq(unstacked.iter())
                .all(|(damaged, target)| match (damaged, target) {
                    (Condition::Operational, Condition::Operational) => true,
                    (Condition::Damaged, Condition::Damaged) => true,
                    (Condition::Unknown, Condition::Operational | Condition::Damaged) => true,
                    (_, _) => false,
                });

        log::trace!("{:?}", damaged_record);
        log::trace!("{:?}", unstacked);

        if is_match {
            possible_arrangements += 1;
        }
    }

    possible_arrangements
}

fn unstack_record(record: Vec<(Condition, usize)>) -> Vec<Condition> {
    let mut unstacked = Vec::new();

    for (condition, amount) in record {
        unstacked.append(&mut vec![condition; amount])
    }

    unstacked
}

fn calculate_target_contingous_record(
    criteria: &[usize],
    record_length: usize,
) -> Vec<(Condition, Range<usize>)> {
    let total_damaged: usize = criteria.iter().sum();
    let maximum = record_length - total_damaged;

    let mut target_contingous_record = Vec::new();
    target_contingous_record.push((Condition::Operational, 0..maximum));

    for (i, number_damaged) in criteria.iter().cloned().enumerate() {
        target_contingous_record.push((Condition::Damaged, number_damaged..number_damaged + 1));

        if i < criteria.len() - 1 {
            target_contingous_record.push((Condition::Operational, 1..maximum));
        }
    }

    target_contingous_record.push((Condition::Operational, 0..maximum));

    target_contingous_record
}

fn convert_to_contingous_record(record: &[Condition]) -> Vec<(Condition, usize)> {
    let mut contingous_record = Vec::new();

    let mut maybe_prev_condition: Option<Condition> = None;
    let mut condition_count = 0;

    let record_iter = record.iter();
    for condition in record_iter {
        if let Some(prev_condition) = maybe_prev_condition {
            if *condition == prev_condition {
                condition_count += 1;
                continue;
            }

            contingous_record.push((prev_condition, condition_count));
        }
        maybe_prev_condition = Some(*condition);
        condition_count = 1;
    }
    contingous_record.push((maybe_prev_condition.unwrap(), condition_count));
    contingous_record
}
