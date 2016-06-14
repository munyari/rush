use std::io::Write;
use std::io;
use std::io::stdout;
use std::io::stdin;
use std::process::Command;

fn main() {
    // print the prompt to the user
    print!("$ ");

    stdout().flush().ok();

    let mut user_input = String::new();

    stdin().read_line(&mut user_input).expect("Failed to read line");

    for statement in user_input.split(";") {
        if let Err(e) = run_statement(statement) {
            println!("Invalid command: {}", e);
        }
    }
}

fn run_statement(statement: &str) -> io::Result<()> {
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
