mod parser;
mod set;

use std::collections::HashMap;
use std::env;
use std::fs;

use crate::set::Range;
use crate::set::Set;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Statement {
    Accepted,
    Rejected,
    If(BooleanExpression, Box<Statement>, Box<Statement>),
    Workflow(WorkflowName),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Var {
    X,
    M,
    A,
    S,
}

type PartRatingValue = u64;

#[derive(Debug, PartialEq, Eq, Clone)]
enum BooleanExpression {
    GreaterThan(Var, PartRatingValue),
    LessThan(Var, PartRatingValue),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct WorkflowName(String);

#[derive(Debug, Clone, Copy)]
struct PartRating {
    x: PartRatingValue,
    m: PartRatingValue,
    a: PartRatingValue,
    s: PartRatingValue,
}

#[derive(Debug)]
struct System {
    workflows: HashMap<WorkflowName, Statement>,
    part_ratings: Vec<PartRating>,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let system: System = input.parse().unwrap();
    log::debug!("{:#?}", system);

    const STARTING_WORKFLOW_NAME: &str = "in";
    let starting_workflow_name = WorkflowName(STARTING_WORKFLOW_NAME.to_string());
    let starting_statement = system.workflows.get(&starting_workflow_name).unwrap();

    let sum_of_accepted_rating_numbers = system
        .part_ratings
        .iter()
        .filter(|part_rating| {
            evaluate(&system.workflows, starting_statement, **part_rating) == Statement::Accepted
        })
        .map(|part_rating| part_rating.x + part_rating.m + part_rating.a + part_rating.s)
        .sum::<PartRatingValue>();
    println!("{}", sum_of_accepted_rating_numbers);

    const MIN: PartRatingValue = 1;
    const MAX: PartRatingValue = 4000 + 1;

    let starting_set = Set(vec![Range { min: MIN, max: MAX }]);
    let sets = [
        starting_set.clone(),
        starting_set.clone(),
        starting_set.clone(),
        starting_set,
    ];

    let total_combinations: PartRatingValue =
        calculate_total_combinations(&system.workflows, starting_statement, sets);
    println!("{}", total_combinations);
}

fn calculate_total_combinations(
    workflows: &HashMap<WorkflowName, Statement>,
    statement: &Statement,
    sets: [Set; 4],
) -> PartRatingValue {
    match statement {
        Statement::Accepted => sets.iter().map(|set| set.cardinality()).product(),
        Statement::Rejected => 0,
        Statement::If(boolean_expression, stmt_1, stmt_2) => {
            let mut set_1 = sets.clone();
            let mut set_2 = sets.clone();

            apply_boolean_constraint(boolean_expression, &mut set_1);
            apply_boolean_constraint(&inverse(boolean_expression), &mut set_2);

            let combinations_if_true = calculate_total_combinations(workflows, stmt_1, set_1);
            let combinations_if_false = calculate_total_combinations(workflows, stmt_2, set_2);

            combinations_if_true + combinations_if_false
        }
        Statement::Workflow(workflow_name) => {
            let new_statement = workflows.get(workflow_name).unwrap();
            calculate_total_combinations(workflows, new_statement, sets)
        }
    }
}

fn inverse(boolean_expression: &BooleanExpression) -> BooleanExpression {
    match boolean_expression {
        BooleanExpression::GreaterThan(var, value) => BooleanExpression::LessThan(*var, *value + 1),
        BooleanExpression::LessThan(var, value) => BooleanExpression::GreaterThan(*var, *value - 1),
    }
}

fn apply_boolean_constraint<'a>(
    boolean_expression: &'a BooleanExpression,
    sets: &'a mut [Set; 4],
) -> &'a mut [Set; 4] {
    match boolean_expression {
        BooleanExpression::GreaterThan(var, value) => {
            let min = *value + 1;
            let max = PartRatingValue::MAX;

            let range = Range { min, max };

            insert_range(range, sets, var)
        }
        BooleanExpression::LessThan(var, value) => {
            let min = PartRatingValue::MIN;
            let max = *value;

            let range = Range { min, max };

            insert_range(range, sets, var)
        }
    }
}

fn insert_range<'a>(range: Range, sets: &'a mut [Set; 4], var: &'a Var) -> &'a mut [Set; 4] {
    let mut set = Set(vec![range]);

    let set_index = match var {
        Var::X => 0,
        Var::M => 1,
        Var::A => 2,
        Var::S => 3,
    };

    sets[set_index].intersection(&mut set);

    sets
}

fn evaluate(
    workflows: &HashMap<WorkflowName, Statement>,
    statement: &Statement,
    part_rating: PartRating,
) -> Statement {
    match statement {
        Statement::Accepted => Statement::Accepted,
        Statement::Rejected => Statement::Rejected,
        Statement::If(boolean_statement, if_true_statement, if_false_statement) => {
            match evaluate_boolean(boolean_statement, part_rating) {
                true => evaluate(workflows, if_true_statement, part_rating),
                false => evaluate(workflows, if_false_statement, part_rating),
            }
        }
        Statement::Workflow(workflow_name) => {
            let statement = workflows.get(workflow_name).unwrap();
            evaluate(workflows, statement, part_rating)
        }
    }
}

fn evaluate_boolean(boolean_statement: &BooleanExpression, part_rating: PartRating) -> bool {
    match boolean_statement {
        BooleanExpression::GreaterThan(var, value) => get_var(part_rating, var) > *value,
        BooleanExpression::LessThan(var, value) => get_var(part_rating, var) < *value,
    }
}

fn get_var(part_rating: PartRating, var: &Var) -> PartRatingValue {
    match var {
        Var::X => part_rating.x,
        Var::M => part_rating.m,
        Var::A => part_rating.a,
        Var::S => part_rating.s,
    }
}
