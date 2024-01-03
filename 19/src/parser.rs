use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::{BooleanStatement, PartRating, PartRatingValue, Statement, System, Var, WorkflowName};

#[derive(Debug)]
pub enum ParseStatementError {
    UnableToParseStatement(String),
    InvalidVar(ParseVarError),
    InvalidRating(ParseIntError),
    UnknownBooleanOperator(String),
}

impl FromStr for Statement {
    type Err = ParseStatementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ACCEPTED_SYMBOL: &str = "A";
        if s == ACCEPTED_SYMBOL {
            return Ok(Statement::Accepted);
        }

        const REJECTED_SYMBOL: &str = "R";
        if s == REJECTED_SYMBOL {
            return Ok(Statement::Rejected);
        }

        const TERNARY_SYMBOL: char = ':';
        const OR_SYMBOL: char = ',';

        let maybe_ternary_symbol_position = s.chars().position(|c| c == TERNARY_SYMBOL);
        let maybe_or_symbol_position = s.chars().position(|c| c == OR_SYMBOL);

        if let (Some(ternary_position), Some(or_symbol_position)) =
            (maybe_ternary_symbol_position, maybe_or_symbol_position)
        {
            let boolean_statement = parse_boolean_statement(&s[0..ternary_position])?;
            let statement_1: Statement = s[ternary_position + 1..or_symbol_position].parse()?;
            let statement_2: Statement = s[or_symbol_position + 1..].parse()?;

            return Ok(Statement::If(
                boolean_statement,
                Box::new(statement_1),
                Box::new(statement_2),
            ));
        }

        if let Ok(workflow_name) = s.parse() {
            return Ok(Statement::Workflow(workflow_name));
        }

        Err(ParseStatementError::UnableToParseStatement(s.to_string()))
    }
}

fn parse_boolean_statement(s: &str) -> Result<BooleanStatement, ParseStatementError> {
    let var = s[0..1].parse().map_err(ParseStatementError::InvalidVar)?;
    let operator_str = &s[1..2];
    let rating_str = &s[2..];
    let rating_value = rating_str
        .parse()
        .map_err(ParseStatementError::InvalidRating)?;

    match operator_str {
        ">" => Ok(BooleanStatement::GreaterThan(var, rating_value)),
        "<" => Ok(BooleanStatement::LessThan(var, rating_value)),
        _ => Err(ParseStatementError::UnknownBooleanOperator(
            operator_str.to_string(),
        )),
    }
}

#[derive(Debug)]
pub enum ParseVarError {
    UnknownVar(String),
}

impl FromStr for Var {
    type Err = ParseVarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Var::X),
            "m" => Ok(Var::M),
            "a" => Ok(Var::A),
            "s" => Ok(Var::S),
            _ => Err(ParseVarError::UnknownVar(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum ParseWorkflowNameError {}

impl FromStr for WorkflowName {
    type Err = ParseWorkflowNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WorkflowName(s.to_string()))
    }
}

#[derive(Debug)]
pub enum ParsePartRatingError {
    InvalidX(ParseIntError),
    InvalidM(ParseIntError),
    InvalidA(ParseIntError),
    InvalidS(ParseIntError),
    InvalidFormat,
}

impl FromStr for PartRating {
    type Err = ParsePartRatingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = &s[1..s.len() - 1];

        let [x_value, m_value, a_value, s_value]: [&str; 4] = values
            .split(',')
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| ParsePartRatingError::InvalidFormat)?;

        fn parse_rating_declaration(s: &str) -> Result<PartRatingValue, ParseIntError> {
            s[2..].parse()
        }

        let x = parse_rating_declaration(x_value).map_err(ParsePartRatingError::InvalidX)?;
        let m = parse_rating_declaration(m_value).map_err(ParsePartRatingError::InvalidM)?;
        let a = parse_rating_declaration(a_value).map_err(ParsePartRatingError::InvalidA)?;
        let s = parse_rating_declaration(s_value).map_err(ParsePartRatingError::InvalidS)?;

        Ok(PartRating { x, m, a, s })
    }
}

#[derive(Debug)]
pub enum ParseSystemError {
    InvalidSystemFormat,
    UnableToFindStatementStart,
    InvalidWorkflowName(ParseWorkflowNameError),
    InvalidStatement(ParseStatementError),
    InvalidRating(ParsePartRatingError),
}

impl FromStr for System {
    type Err = ParseSystemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [workflows_str, part_ratings_str]: [&str; 2] = s
            .split("\n\n")
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| ParseSystemError::InvalidSystemFormat)?;

        let workflows: HashMap<WorkflowName, Statement> = workflows_str
            .split('\n')
            .map(|workflow_str| {
                let statement_start = workflow_str
                    .chars()
                    .position(|c| c == '{')
                    .ok_or(ParseSystemError::UnableToFindStatementStart)?;

                let workflow_name: WorkflowName = workflow_str[..statement_start]
                    .parse()
                    .map_err(ParseSystemError::InvalidWorkflowName)?;
                let statement: Statement = workflow_str
                    [statement_start + 1..workflow_str.len() - 1]
                    .parse()
                    .map_err(ParseSystemError::InvalidStatement)?;

                Ok((workflow_name, statement))
            })
            .collect::<Result<HashMap<WorkflowName, Statement>, ParseSystemError>>()?;

        let part_ratings: Vec<PartRating> = part_ratings_str
            .split('\n')
            .map(|part_rating_str| {
                part_rating_str
                    .parse()
                    .map_err(ParseSystemError::InvalidRating)
            })
            .collect::<Result<Vec<PartRating>, ParseSystemError>>()?;

        Ok(System {
            workflows,
            part_ratings,
        })
    }
}
