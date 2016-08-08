#[cfg(test)]

use nom::IResult::*;

use error;
use parser;
use parser::Statement;

const ERROR_PROMPT: &'static str = "[0m[01;31m$[0m "; // our prompt is red
const NORMAL_PROMPT: &'static str = "$ ";

#[test]
fn test_prompt_on_successful_exit() {
    assert_eq!(NORMAL_PROMPT, super::get_prompt(&Ok(())));
}

#[test]
fn test_prompt_on_unsuccessful_exit() {
    assert_eq!(ERROR_PROMPT,
               super::get_prompt(&Err(error::ShellError::False)));
}

#[test]
fn test_exit_message() {
    assert_eq!("Goodbye!", super::exit_message());
}

// parser tests
#[test]
fn test_multiple_statement_parser() {
    assert_eq!(parser::statement_list("true && true ; true || true"),
    Done("", vec![Statement::And(
            Box::new(Statement::Simple("true", vec![])),
            Box::new(Statement::Simple("true", vec![]))
            ),
            Statement::Or(
                Box::new(Statement::Simple("true", vec![])),
                Box::new(Statement::Simple("true", vec![]))),
    ])
    );
}
