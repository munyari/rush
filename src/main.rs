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
use std::process::Command;
use readline::Error::*;

mod builtin;
mod parser;
mod error;
mod tests;
use parser::Statement;
use error::ShellError;

fn main() {
    print_disclaimer();

    start_shell();
}

fn start_shell() {

    let mut return_status: Result<(), ShellError> = Ok(());

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

        match parser::statement_list(&user_input) {
            IResult::Done(_, list) => {
                for statement in list {
                    return_status = run_statement(&statement);
                    if let Err(ref e) = return_status {
                        // TODO: not right. When I run false this doesn't have
                        // the right behavior
                        println!("Invalid command: {}", e);
                    }
                }
            }
            IResult::Incomplete(_) => {}
            IResult::Error(e) => panic!("Fatal parse error: {}", e)
        }
    }
}

fn get_prompt(return_status: &Result<(), ShellError>) -> String {
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

// TODO: Named fields in Struct
fn run_statement(statement: &Statement) -> Result<(), ShellError> {
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
            // TODO: use a set that contains all built ins
            match command {
                "exit" => {
                    println!("{}", exit_message());
                    std::process::exit(0)
                }
                "cd" => builtin::cd(arguments),
                "true" => builtin::te(),
                "false" => builtin::fe(),
                "pwd" => builtin::pwd(arguments),
                _ => {
                    let out = Command::new(command)
                        .args(&arguments[..])
                        .spawn();

                    if let Err(e) = out {
                        return Err(ShellError::from(e));
                    }

                    try!(out.unwrap().wait());

                    Ok(())
                }
            }
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
