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

    for statement in user_input.split(";") {
        let mut command_line = statement.split_whitespace();
        let command = command_line.next().unwrap();
        println!("command: {}", command);

        let arguments = command_line.collect::<Vec<&str>>();
        let mut out = Command::new(command)
            .args(&arguments[..])
            .spawn()
            .expect("Invalid command");
        let ecode = out.wait().expect("Ahh!!");
        let exit_status = ecode.success();
        println!("Exited {} ", exit_status);
    }
}
