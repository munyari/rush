// Rush - A simple Unix shell written in Rust
//     Copyright (C) 2016  Panashe M. Fundira

//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.

//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <http://www.gnu.org/licenses/>.
extern crate ansi_term;
#[macro_use]
extern crate nom;
extern crate readline;

use ansi_term::Colour::Red;
use nom::*;

// TODO: Why am I using IO result???
use std::io::Result;
use std::process::Command;
use readline::Error::*;

mod builtin;

fn main() {
    print_disclaimer();

    start_shell();
}

fn start_shell() {

    let mut return_status: Result<()> = Ok(());

    'repl: loop {
        let user_input = match readline::readline(&get_prompt(&return_status)) {
            Err(InvalidUtf8) => {
                println!("Input is not valid UTF8");
                continue;
            }
            Err(EndOfFile)   => break,
            Err(e)           => panic!("Error {}", e),
            Ok(s)            => s,
        };

        match statement_list(&user_input) {
            IResult::Done(_, list) => {
                for statement in list {
                    return_status = run_statement(&statement);
                    if let Err(ref e) = return_status {
                        println!("Invalid command: {}", e);
                    }
                }
            }
            IResult::Incomplete(_) => {}
            IResult::Error(e) => panic!("Fatal parse error: {}", e)
        }
    }
}

fn get_prompt(return_status: &Result<()>) -> String {
    // The error prompt is red
    let error_prompt: String = Red.paint("$ ").to_string();
    let normal_prompt: String = String::from("$ ");

    match *return_status {
        Ok(()) => normal_prompt,
        _ => error_prompt,
    }
}

fn exit_message() -> &'static str {
    "Goodbye!"
}

// Named fields in Struct
fn run_statement(statement: &Statement) -> Result<()> {
    match *statement {
        Statement::And(ref s1, ref s2) => {
            match run_statement(s1) {
                Ok(_) => run_statement(s2),
                Err(e) => Err(e),
            }
        }
        Statement::Or(ref s1, ref s2) => {
            let return1 = run_statement(s1);
            let return2 = run_statement(s2);
            if !return1.is_ok() {
                return2
            } else {
                return1
            }
        }

        Statement::Simple(command, ref arguments) => {
            let out = Command::new(command)
                .args(&arguments[..])
                .spawn();

            if let Err(e) = out {
                return Err(e);
            }
                "true" => Ok(()),
                "false" => Err(Error::new(std::io::ErrorKind::Other, "false")),

            try!(out.unwrap().wait());

            Ok(())
        }
    }
}

fn print_disclaimer() -> () {
    let disclaimer = "Rush -  Copyright (C) 2016  Panashe M. Fundira \
    \nThis program comes with ABSOLUTELY NO WARRANTY; for details type `show w'. \
    \nThis is free software, and you are welcome to redistribute it \
    \nunder certain conditions; type `show c' for details.";
    println!("{}", disclaimer);
}

// helper parsers

fn alphanum_or_underscore_or_dash(c: char) -> bool {
    c == '_' || c == '-' || c.is_alphanum()
}
named!(whitespace<&str, &str>, is_a_s!(" \t"));
named!(command_terminator<&str, &str>, tag_s!(";"));
named!(executable<&str, &str>, take_while1_s!(alphanum_or_underscore_or_dash));
named!(argument<&str, &str>, take_while1_s!(alphanum_or_underscore_or_dash));
named!(arguments<&str, std::vec::Vec<&str> >, many0!(argument));
named!(and<&str, &str>, tag_s!("&&"));
named!(or<&str, &str>, tag_s!("||"));
named!(simple_statement<&str, Statement>,
       chain!(
           whitespace? ~
           ex: executable ~
           whitespace? ~
           args: arguments ~
           whitespace?,
           || { Statement::Simple(ex, args) }
           )
      );

#[derive(Debug)]
#[derive(PartialEq)]
enum Statement<'a> {
    And(Box<Statement<'a>>, Box<Statement<'a>>),
    Or(Box<Statement<'a>>, Box<Statement<'a>>),
    Simple(&'a str, Vec<&'a str>),
}

named!(statement<&str, Statement>, chain!(
        // TODO: This isn't the correct order!
        // s: alt!(simple_statement | compound_statement),
        s: alt_complete!(compound_statement | simple_statement),
        || { s }
        )
      );
named!(and_statement<&str, Statement>, chain!(
        s1: simple_statement ~
        and ~
        s2: statement,
        || { Statement::And(Box::new(s1), Box::new(s2)) }
        )
      );
named!(or_statement<&str, Statement>, chain!(
        s1: simple_statement ~
        or ~
        s2: statement,
        || { Statement::Or(Box::new(s1), Box::new(s2)) }
        )
      );
named!(compound_statement<&str, Statement>, alt!(and_statement | or_statement));
// statements are delimited here
named!(statement_list<&str, Vec<Statement> >, separated_list!(command_terminator, statement));

#[cfg(test)]
mod tests {
    use std::io;

    use nom::IResult::*;
    use nom::Err::Position;
    use nom::ErrorKind::*;
    use nom::Needed;
    use super::Statement;

    const ERROR_PROMPT: &'static str = "[0m[01;31m$[0m "; // our prompt is red
    const NORMAL_PROMPT: &'static str = "$ ";

    #[test]
    fn test_prompt_on_successful_exit() {
        assert_eq!(NORMAL_PROMPT, super::get_prompt(&Ok(())));
    }

    #[test]
    fn test_prompt_on_unsuccessful_exit() {
        assert_eq!(ERROR_PROMPT,
                   super::get_prompt(
                       &Err(
                           io::Error::new(io::ErrorKind::Other, "oh no!")
                           )
                       )
                  );
    }

    #[test]
    fn test_exit_message() {
        assert_eq!("Goodbye!", super::exit_message());
    }

    // parser tests
    #[test]
    fn test_command_terminator_parser() {
        assert_eq!(super::command_terminator(";"), Done("", ";"));
        assert_eq!(super::command_terminator(" "),
        Error(Position(TagStr, " ")));
    }

    #[test]
    fn test_end_of_statement_parser() {
        assert_eq!(super::end_of_statement(";"), Done("", ";"));
        assert_eq!(super::end_of_statement("\n"), Done("", "\n"));
        assert_eq!(super::command_terminator(" "),
        Error(Position(TagStr, " ")));
    }

    #[test]
    fn test_executable_parser() {
        assert_eq!(super::executable("nvim"), Done("", "nvim"));
        assert_eq!(super::executable("_this_is_perfectly_valid_"),
        Done("", "_this_is_perfectly_valid_"));
        assert_eq!(super::executable(" "),
        Error(Position(TakeWhile1Str, " ")));
    }

    #[test]
    fn test_argument() {
        assert_eq!(super::argument("--color"), Done("", "--color"));
        assert_eq!(super::argument("-a"), Done("", "-a"));
        assert_eq!(super::argument("a"), Done("", "a"));
        assert_eq!(super::argument("This_is_a_valid_argument"),
        Done("", "This_is_a_valid_argument"));
        assert_eq!(super::argument("Only the first word is an argument"),
        Done(" the first word is an argument", "Only"));
    }

    #[test]
    fn test_connective_parser() {
        assert_eq!(super::connective("&&"), Done("", "&&"));
        assert_eq!(super::connective("||"), Done("", "||"));
        assert_eq!(super::connective(""), Incomplete(Needed::Size(2)));
        assert_eq!(super::connective("I'm not valid!"),
        Error(Position(Alt, "I'm not valid!")));

    }

    #[test]
    fn test_empty_parser() {
        assert_eq!(super::empty("\n"), Done("", ""));
        assert_eq!(super::empty("  \n"), Done("", ""));
        assert_eq!(super::empty("\t\n"), Done("", ""));
        assert_eq!(super::empty(";"), Done("", ""));
        assert_eq!(super::empty("  ;"), Done("", ""));
        assert_eq!(super::empty("\t;"), Done("", ""));
        assert_eq!(super::empty("I'm definitely not empty"),
        Error(Position(Alt, "I'm definitely not empty")));
    }

    #[test]
    fn test_simple_statement_parser() {
        // assert_eq!(super::simple_statement("\n"),
        //    Done("\n", super::Statement::Simple("", vec![])));
        assert_eq!(super::simple_statement("ls --color"),
        Done("", super::Statement::Simple("ls", vec!["--color"])));
        assert_eq!(super::simple_statement("ls -a"),
        Done("", super::Statement::Simple("ls", vec!["-a"])));
    }

    #[test]
    fn test_compound_statement_parser() {
        assert_eq!(super::compound_statement("ls && echo hello"),
        Done("", super::Statement::And(
                (Box::new(Statement::Simple("ls", vec![]))),
                (Box::new(
                        Statement::Simple("echo", vec![ "hello"]))
                )
                )
            )
        );

        assert_eq!(super::compound_statement("true || false"),
        Done("", Statement::Or(
                (Box::new(Statement::Simple("true", vec![]))),
                (Box::new(Statement::Simple("false", vec![]))),
                )
            )
        );

        assert_eq!(super::compound_statement("true && true || true"),
        Done("", Statement::And(
                (Box::new(Statement::Simple("true", vec![]))),
                Box::new(Statement::Or(
                        Box::new(Statement::Simple("true", vec![])),
                        Box::new(Statement::Simple("true", vec![]))
                        )
                        )
                )
            )
        );
    }

    #[test]
    #[ignore]
    fn test_or_statement_parser() {
        panic!()
    }

    #[test]
    #[ignore]
    fn test_and_statement_parser() {
        panic!()
    }

    #[test]
    fn test_multiple_statement_parser() {
        assert_eq!(super::statement_list("true && true ; true || true"),
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

}
