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

use std::io::Result;
use std::process::Command;
extern crate readline;
use readline::Error::*;

fn main() {
    print_disclaimer();

    start_shell();
}

fn start_shell() {
    'repl: loop {
        let user_input = match readline::readline(get_prompt()) {
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
            if let Err(e) = run_statement(statement) {
                println!("Invalid command: {}", e);
            }
        }
    }
}


fn get_prompt() -> &'static str {
    "$ "
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
    println!("command: {}", command);

    let arguments = command_line.collect::<Vec<&str>>();
    let mut out = try!(Command::new(command)
                       .args(&arguments[..])
                       .spawn());

    let ecode = try!(out.wait());

    let exit_status = ecode.success();
    println!("Exited {} ", exit_status);

    Ok(())
}

fn print_disclaimer() -> () {
    let disclaimer = "Rush -  Copyright (C) 2016  Panashe M. Fundira \
    \nThis program comes with ABSOLUTELY NO WARRANTY; for details type `show w'. \
    \nThis is free software, and you are welcome to redistribute it \
    \nunder certain conditions; type `show c' for details.";
    println!("{}", disclaimer);
}

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
