use std::io::Write;
use std::io::stdout;
use std::io::stdin;
use std::process::Command;

fn main() {
    // print the prompt to the user
    print!("$ ");

    stdout().flush().ok();

    let mut user_input = String::new();

    stdin().read_line(&mut user_input).expect("Failed to read line");

    // TODO: needs to preserve string quoted areas
    // Do I need to use a collection of strings?
    let mut command_line = user_input.split_whitespace(); //.collect::<Vec<&str>>();
    let command = command_line.next().unwrap();

    println!("command: {}", command);

    for argument in command_line {
        println!("argument: {}", argument);
    }
    let mut out = Command::new(command).spawn().expect("Invalid command");
    let ecode = out.wait().expect("Ahh!!");
    let exit_status = ecode.success();
    println!("Exited {} ", exit_status);
    // DONE: read in command
    // tokenize command (executable, argumentList, optional connectors)
    // execute the command
    // must be a REPL
    // exit exists the shell
    // # is a comment

}
