use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Part {
    Ash,
    Rock,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Reflection {
    Horizontal,
    Vertical,
}

type Line = Vec<Part>;
type Pattern = Vec<Line>;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).unwrap();
    fs::read_to_string(filepath).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let patterns: Vec<Pattern> = parse_patterns(&input).unwrap();

    let reflections: Vec<(usize, Reflection, usize)> = patterns
        .iter()
        .enumerate()
        .flat_map(|(i, pattern)| {
            find_reflection(&pattern)
                .into_iter()
                .map(move |(reflection, index_start)| (i, reflection, index_start))
        })
        .inspect(|reflection| log::trace!("{:?}", reflection))
        .collect();

    let summary = summarise(&reflections);
    println!("{}", summary)
}

fn summarise(reflections: &[(usize, Reflection, usize)]) -> usize {
    reflections
        .iter()
        .fold(0, |curr, (_, reflection, index_start)| {
            curr + match reflection {
                Reflection::Horizontal => 100 * *index_start,
                Reflection::Vertical => *index_start,
            }
        })
}

fn find_reflection(pattern: &[Line]) -> Vec<(Reflection, usize)> {
    let transposed_pattern = transpose(pattern);

    let mut reflections = scan_for_reflections(&transposed_pattern, Reflection::Vertical);
    let mut horizontal_reflections = scan_for_reflections(pattern, Reflection::Horizontal);

    reflections.append(&mut horizontal_reflections);
    return reflections;
}

fn transpose(pattern: &[Line]) -> Pattern {
    let mut transposed_pattern = Vec::new();

    let width = pattern.first().unwrap().len();
    for column_index in 0..width {
        let column: Line = pattern
            .iter()
            .map(|line| &line[column_index])
            .copied()
            .collect();
        transposed_pattern.push(column);
    }

    transposed_pattern
}

fn scan_for_reflections(pattern: &[Line], reflection_type: Reflection) -> Vec<(Reflection, usize)> {
    let mut reflections = Vec::new();

    for i in 0..pattern.len() - 1 {
        let j = i + 1;

        let is_potential_reflection = pattern[i] == pattern[j];
        if is_potential_reflection {
            let reflection_confirmation = confirm_reflection(pattern, i);

            if reflection_confirmation {
                reflections.push((reflection_type, i + 1));
            }
        }
    }

    reflections
}

fn confirm_reflection(pattern: &[Line], i: usize) -> bool {
    let (mut i, mut j) = (i, i + 1);

    loop {
        let lines_match = pattern[i] == pattern[j];

        if lines_match == false {
            return false;
        }

        if i == 0 || j == pattern.len() - 1 {
            break;
        }

        i -= 1;
        j += 1;
    }

    true
}

#[derive(Debug)]
enum ParsePatternError {
    UnknownPart(char),
}

fn parse_patterns(input: &str) -> Result<Vec<Pattern>, ParsePatternError> {
    input
        .split("\n\n")
        .map(|pattern_str| {
            pattern_str
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Ok(Part::Ash),
                            '#' => Ok(Part::Rock),
                            a => Err(ParsePatternError::UnknownPart(a)),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}
