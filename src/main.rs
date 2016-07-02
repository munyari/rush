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
#[macro_use]
extern crate nom;
extern crate readline;

use nom::*;

use std::io::Result;
use std::process::Command;
use readline::Error::*;

fn main() {
    print_disclaimer();

    start_shell();
}

fn start_shell() {

    let mut return_status: Result<()> = Ok(());

    'repl: loop {
        let user_input = match readline::readline(get_prompt(&return_status)) {
            Err(InvalidUtf8) => {
                println!("Input is not valid UTF8");
                continue;
            }
            Err(EndOfFile)   => break,
            Err(e)           => panic!("Error {}", e),
            Ok(s)            => s,
        };

        let commands = parse_commands(&user_input);
        eval_tree(commands);

        // // TODO: parsing logic
        // for statement in user_input.split(";") {
        //     if statement.trim() == "exit" {
        //         println!("{}", exit_message());
        //         break 'repl;
        //     }
        //     return_status = run_statement(statement);
        //     if let Err(ref e) = return_status {
        //         println!("Invalid command: {}", e);
        //     }
        // }

    }
}

fn parse_commands(input: &str) -> Vec<ShellCommand> {
    vec![ShellCommand::SimpleStatement("",vec![""])]
}

fn eval_tree(commands: Vec<ShellCommand>) -> () {

}


fn get_prompt(return_status: &Result<()>) -> &'static str {
    const ERROR_PROMPT: &'static str =  "[0m[01;31m$[0m "; // our prompt is red
    const NORMAL_PROMPT: &'static str = "$ ";

    match *return_status {
        Ok(()) => NORMAL_PROMPT,
        _ => ERROR_PROMPT,
    }
}

fn exit_message() -> &'static str {
    "Goodbye!"
}

fn run_statement(statement: &str) -> Result<()> {
    let mut command_line = statement.split_whitespace();
    let command = match command_line.next() {
        Some(c) => c,
        None    => return Ok(()),
    };

    // TODO: wrong arguments don't pass error
    let arguments = command_line.collect::<Vec<&str>>();
    let out = Command::new(command)
        .args(&arguments[..])
        .spawn();

    match out {
        Err(e) => return Err(e),
        _ => (),
    };

    // we wait until the command has exited before proceeding
    try!(out.unwrap().wait());

    // command has successfuly executed
    Ok(())
}

fn print_disclaimer() -> () {
    let disclaimer = "Rush -  Copyright (C) 2016  Panashe M. Fundira \
    \nThis program comes with ABSOLUTELY NO WARRANTY; for details type `show w'. \
    \nThis is free software, and you are welcome to redistribute it \
    \nunder certain conditions; type `show c' for details.";
    println!("{}", disclaimer);
}

// helper parsers
named!(end_of_line<&str, &str>, tag_s!("\n"));
fn alpha_or_underscore_or_dash(c: char) -> bool {
    c == '_' || c == '-' || c.is_alphanum()
}
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t'
}
named!(whitespace<&str, &str>, take_while1_s!(is_whitespace));

named!(statement_terminator<&str, &str>, is_a_s!(";"));
named!(end_of_statement<&str, &str>,
           alt!(end_of_line | statement_terminator));

named!(executable<&str, &str>, take_while1_s!(alpha_or_underscore_or_dash));
named!(argument<&str, &str>, take_while1_s!(alpha_or_underscore_or_dash));
named!(arguments<&str, std::vec::Vec<&str> >, many0!(argument));
named!(and<&str, &str>, tag_s!("&&"));
named!(or<&str, &str>, tag_s!("||"));
named!(connective<&str, &str>, alt!(and | or));

// // used space here because multispace recognizes line feeds
named!(empty<&str, &str>, chain!(
        whitespace? ~ end_of_statement,
        || { "" }
        )
      );

named!(simple_statement<&str, ShellCommand>,
    chain!(
        whitespace? ~
        ex: executable? ~
        whitespace? ~
        args: arguments? ~
        whitespace? ~
        end_of_statement,
        || {
            ShellCommand::SimpleStatement(
                (ex.unwrap_or("")), (args.unwrap_or((vec![])))
                )
        }
    )
);

#[derive(Debug)]
#[derive(PartialEq)]
enum ShellCommand<'a> {
    And(Box<ShellCommand<'a>>, Box<ShellCommand<'a>>),
    Or(Box<ShellCommand<'a>>, Box<ShellCommand<'a>>),
    SimpleStatement(&'a str, Vec<&'a str>),
    Statements(Vec<&'a ShellCommand<'a>>)
}


named!(statement<&str, ShellCommand>, alt!(compound_statement | simple_statement));
named!(and_statement<&str, ShellCommand>, chain!(
        s1: statement ~
        and ~
        s2: statement,
        || { ShellCommand::And(Box::new(s1), Box::new(s2)) }
        )
    );
named!(or_statement<&str, ShellCommand>, chain!(
        s1: statement ~
        or ~
        s2: statement,
        || { ShellCommand::Or(Box::new(s1), Box::new(s2)) }
        )
    );
named!(compound_statement<&str, ShellCommand>, alt!(and_statement | or_statement));

#[cfg(test)]
mod tests {
    use std::io;

    use nom::IResult::*;
    use nom::Err::Position;
    use nom::ErrorKind::*;
    use nom::Needed;

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
    fn test_statement_terminator_parser() {
        assert_eq!(super::statement_terminator(";"), Done("", ";"));
        assert_eq!(super::statement_terminator(" "),
                   Error(Position(IsAStr, " ")));
    }

    #[test]
    fn test_end_of_statement_parser() {
        assert_eq!(super::end_of_statement(";"), Done("", ";"));
        assert_eq!(super::end_of_statement("\n"), Done("", "\n"));
        assert_eq!(super::statement_terminator(" "),
                   Error(Position(IsAStr, " ")));
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
    #[ignore]
    fn test_simple_statement_parser() {
        assert_eq!(super::simple_statement("\n"),
           Done("", super::ShellCommand::SimpleStatement("", vec![])));
        assert_eq!(super::simple_statement("ls --color\n"),
           Done("", super::ShellCommand::SimpleStatement("ls", vec!["--color"])));
        assert_eq!(super::simple_statement("ls -a\n"),
           Done("", super::ShellCommand::SimpleStatement("ls", vec!["-a"])));
    }

    #[test]
    #[ignore]
    fn test_compound_statement_parser() {
        panic!()
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
    #[ignore]
    fn test_multiple_statement_parser() {
        panic!()
    }

}
