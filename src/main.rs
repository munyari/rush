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

        for statement in user_input.split(";") {
            if statement.trim() == "exit" {
                println!("{}", exit_message());
                break 'repl;
            }
            return_status = run_statement(statement);
            if let Err(ref e) = return_status {
                println!("Invalid command: {}", e);
            }
        }
    }
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
    let mut out = Command::new(command)
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

named!(statement_terminator, is_a!(";"));
named!(end_of_statement, alt!(line_ending | statement_terminator));
named!(executable, alt!(alphanumeric | is_a!("_")));
named!(arguments, delimited!(
        alt!(tag!("-") | tag!("--")),
        alphanumeric,
        opt!(multispace)));
named!(connective, alt!(tag!("&&") | tag!("||")));
named!(empty, chain!(
        acc: alt!(
                    tag!("") | multispace
                ) ~
            end_of_statement,
            || { return acc }
            )
    );
named!(statement, alt!(empty |
                   chain!(
                        acc: executable ~
                        opt!(arguments) ~
                        opt!(connective) ~
                        opt!(statement) ~
                        end_of_statement,
                    || { return acc }
                    )
                   )
       );

#[cfg(test)]
mod tests {
    use super::{get_prompt, exit_message};

    #[test]
    fn test_get_prompt() {
        assert_eq!("$ ", get_prompt());
    }

    #[test]
    fn test_exit_message() {
        assert_eq!("Goodbye!", exit_message());
    }
}
