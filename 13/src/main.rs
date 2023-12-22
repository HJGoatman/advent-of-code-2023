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

    let reflections: Vec<(Reflection, usize)> = patterns
        .iter()
        .map(|pattern| find_reflection(&pattern, None).unwrap())
        .inspect(|reflection| log::trace!("{:?}", reflection))
        .collect();

    let summary = summarise(&reflections);
    println!("{}", summary);

    let new_reflections: Vec<(Reflection, usize)> = patterns
        .iter()
        .zip(reflections)
        .map(|(pattern, reflection)| find_clean_reflection(pattern, reflection))
        .collect();
    let summary = summarise(&new_reflections);
    println!("{}", summary);
}

fn find_clean_reflection(pattern: &[Line], reflection: (Reflection, usize)) -> (Reflection, usize) {
    for i in 0..pattern.len() {
        let line = &pattern[i];

        for possible_smudgeless_line in get_possible_smudgeless_lines(line) {
            log::trace!("{:?}", possible_smudgeless_line);
            let new_pattern = [
                &pattern[..i],
                &[possible_smudgeless_line],
                &pattern[i + 1..],
            ]
            .concat();

            if let Some(possible_new_reflection) = find_reflection(&new_pattern, Some(reflection)) {
                return possible_new_reflection;
            }
        }
    }

    panic!("No other reflection!");
}

fn get_possible_smudgeless_lines(line: &[Part]) -> Vec<Line> {
    let mut lines = Vec::new();

    for i in 0..line.len() {
        let part = &line[i];

        let new_part = match part {
            Part::Ash => Part::Rock,
            Part::Rock => Part::Ash,
        };

        let new_line = [&line[..i], &[new_part], &line[i + 1..]].concat();
        lines.push(new_line);
    }

    lines
}

fn summarise(reflections: &[(Reflection, usize)]) -> usize {
    reflections
        .iter()
        .fold(0, |curr, (reflection, index_start)| {
            curr + match reflection {
                Reflection::Horizontal => 100 * *index_start,
                Reflection::Vertical => *index_start,
            }
        })
}

fn find_reflection(
    pattern: &[Line],
    skip_reflection: Option<(Reflection, usize)>,
) -> Option<(Reflection, usize)> {
    let transposed_pattern = transpose(pattern);
    let maybe_vertical_reflection =
        scan_for_reflection(&transposed_pattern, Reflection::Vertical, skip_reflection);
    if let Some(vertical_reflection) = maybe_vertical_reflection {
        return Some(vertical_reflection);
    }

    let maybe_horizontal_reflection =
        scan_for_reflection(pattern, Reflection::Horizontal, skip_reflection);
    if let Some(horizontal_reflection) = maybe_horizontal_reflection {
        return Some(horizontal_reflection);
    }

    None
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

fn scan_for_reflection(
    pattern: &[Line],
    reflection_type: Reflection,
    skip_reflection: Option<(Reflection, usize)>,
) -> Option<(Reflection, usize)> {
    for i in 0..pattern.len() - 1 {
        let j = i + 1;

        let is_potential_reflection = pattern[i] == pattern[j];
        if is_potential_reflection {
            let reflection_confirmation = confirm_reflection(pattern, i);

            if reflection_confirmation {
                let found_reflection = (reflection_type, i + 1);

                if let Some(skip_reflection) = skip_reflection {
                    if skip_reflection == found_reflection {
                        continue;
                    }
                }

                return Some(found_reflection);
            }
        }
    }

    None
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
