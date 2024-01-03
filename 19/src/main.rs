mod parser;

use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Statement {
    Accepted,
    Rejected,
    If(BooleanStatement, Box<Statement>, Box<Statement>),
    Workflow(WorkflowName),
}

#[derive(Debug, PartialEq, Eq)]
enum Var {
    X,
    M,
    A,
    S,
}

type PartRatingValue = u32;

#[derive(Debug, PartialEq, Eq)]
enum BooleanStatement {
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
            evaluate(&system.workflows, &starting_statement, **part_rating) == Statement::Accepted
        })
        .map(|part_rating| part_rating.x + part_rating.m + part_rating.a + part_rating.s)
        .sum::<PartRatingValue>();

    println!("{}", sum_of_accepted_rating_numbers);
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

fn evaluate_boolean(boolean_statement: &BooleanStatement, part_rating: PartRating) -> bool {
    match boolean_statement {
        BooleanStatement::GreaterThan(var, value) => get_var(part_rating, var) > *value,
        BooleanStatement::LessThan(var, value) => get_var(part_rating, var) < *value,
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
