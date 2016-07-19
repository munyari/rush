use std::env::current_dir;
use std::io::Result;
use std::io::{Error, ErrorKind};

pub fn pwd(_arguments: &[&str]) -> Result<()> {
    let p = current_dir()
            .expect("Unable to retrieve working directory");
    println!("{}", p.display());
    Ok(())
}

pub fn te() -> Result<()> {
    Ok(())
}

pub fn fe() -> Result<()> {
    Err(Error::new(ErrorKind::Other, "false"))
}
