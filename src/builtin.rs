use std::env::{current_dir, set_current_dir, home_dir};
use std::io::Write;
use std::io::stderr;
use super::error;
use super::error::ShellError;

pub fn pwd(_arguments: &[&str]) -> Result<(), error::ShellError> {
    let p = current_dir()
        .expect("Unable to retrieve working directory");
    println!("{}", p.display());
    Ok(())
}

pub fn te() -> Result<(), error::ShellError> {
    Ok(())
}

pub fn fe() -> Result<(), error::ShellError> {
    Err(error::ShellError::False)
}

pub fn cd(arguments: &[&str]) -> Result<(), ShellError> {
    match arguments.len() {
        0 => {
            set_current_dir(home_dir().unwrap()).unwrap();
            Ok(())
        },
        1 => {
            if let Err(_) = set_current_dir(arguments[0]) {
                let err_msg = format!("cd: no such file or directory: {}",
                                      arguments[0]);
                writeln!(stderr(), "{}", err_msg).unwrap();
                Err(ShellError::Cd(err_msg))
            }
            else {
                Ok(())
            }
        },
        _ => {
            writeln!(stderr(), "cd: too many arguments").unwrap();
            Err(ShellError::Cd("".to_string()))
        }
    }
}
