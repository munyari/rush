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

use std::io::{stdin, stdout, Result, Write};
use std::process::Command;

fn main() {
    print_disclaimer();

    start_shell();
}

fn start_shell() {
    'repl: loop {
        print_prompt();

        let mut user_input = String::new();

        match stdin().read_line(&mut user_input) {
            // EOF case
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => println!("Error: {}. Exiting", e),
        };

        for statement in user_input.split(";") {
            if statement.trim() == "exit" {
                println!("Goodbye!");
                break 'repl;
            }
            if let Err(e) = run_statement(statement) {
                println!("Invalid command: {}", e);
            }
        }
    }
}
fn print_prompt() {
    print!("$ ");
    stdout().flush().ok();
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
