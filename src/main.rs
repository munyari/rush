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

use std::str::from_utf8;

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

        // TODO: parsing logic
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

named!(statement_terminator <&[u8]>, is_a!(";"));
named!(end_of_statement, alt!(line_ending | statement_terminator));

fn alpha_or_underscore_or_dash(c: u8) -> bool {
    // OR dash
    c as char == '_' || (c as char).is_alphanum() || (c as char) == '-'
}
named!(executable, take_while1!(alpha_or_underscore_or_dash));
named!(dashes_p, alt!(tag!("-") | tag!("--") ));
// TODO: refactor!
named!(argument<String>, chain!(
        dashes: dashes_p ~
        inner_arg: executable,
        || {
            let mut s1 = from_utf8(dashes).unwrap().to_string();
            s1.push_str(from_utf8(inner_arg).unwrap());
            s1
        }
        ));
named!(arguments<std::vec::Vec<String> >, many0!(argument));
named!(connective, alt!(tag!("&&") | tag!("||")));

// used space here because multispace recognizes line feeds
named!(empty, chain!(
        space? ~ end_of_statement,
        || { &b""[..] }
        )
      );

named!(statement<&[u8], (&str, std::vec::Vec<String>)>,
    chain!(
        ex: executable? ~
        // TODO: should I have nested chains?
        space? ~
        args: arguments? ~
        space? ~
        end_of_statement,
        || { 
            match ex {
                Some(v) => (from_utf8(v).unwrap_or(""), args.unwrap_or(vec![])),
                None => ("", args.unwrap_or(vec![])),
            }
        }

        )

      );

named!(compound_statement, alt!(chain!(statement ~ connective ~ statement,
                                       || { &b""[..] })));

#[cfg(test)]
mod tests {
    use super::{get_prompt, exit_message, statement_terminator, end_of_statement, executable, argument, connective,empty,statement};
    // use std::io::{Error, ErrorKind};
    use std::*;
    use nom::IResult::*;
    use nom::Err::Position;
    use nom::ErrorKind::{IsA,TakeWhile1,Alt};
    use nom::Needed;

    const ERROR_PROMPT: &'static str =  "[0m[01;31m$[0m "; // our prompt is red
    const NORMAL_PROMPT: &'static str = "$ ";

    #[test]
    fn test_prompt_on_successful_exit() {
        assert_eq!(NORMAL_PROMPT, get_prompt(&Ok(())));
    }

    #[test]
    fn test_prompt_on_unsuccessful_exit() {
        assert_eq!(ERROR_PROMPT,
                   get_prompt(
                       &Err(
                           io::Error::new(io::ErrorKind::Other, "oh no!")
                           )
                       )
                  );
    }

    #[test]
    fn test_exit_message() {
        assert_eq!("Goodbye!", exit_message());
    }


    // parser tests
    #[test]
    fn test_statement_terminator_parser() {
        assert_eq!(statement_terminator(&b";"[..]),
            Done(&b""[..], &b";"[..]));

        // on garbage
        assert_eq!(statement_terminator(&b" "[..]),
            Error(Position(IsA, &b" "[..])));
    }

    #[test]
    fn test_end_of_statement_parser() {
        assert_eq!(end_of_statement(&b";"[..]),
            Done(&b""[..], &b";"[..]));

        assert_eq!(end_of_statement(&b"\n"[..]),
            Done(&b""[..], &b"\n"[..]));

        assert_eq!(statement_terminator(&b" "[..]),
            Error(Position(IsA, &b" "[..])));
    }

    #[test]
    fn test_executable_parser() {
        assert_eq!(executable(&b"nvim"[..]),
            Done(&b""[..], &b"nvim"[..]));

        assert_eq!(executable(&b"_this_is_perfectly_valid_"[..]),
            Done(&b""[..], &b"_this_is_perfectly_valid_"[..]));

        assert_eq!(executable(&b" "[..]),
            Error(Position(TakeWhile1, &b" "[..])));
    }

    #[test]
    fn test_argument() {
        assert_eq!(argument(&b"--color"[..]),
            Done(&b""[..], "--color".to_string()));

        assert_eq!(argument(&b"-a"[..]),
            Done(&b""[..], "-a".to_string()));

        assert_eq!(argument(&b"a"[..]),
            Error(Position(Alt, &b"a"[..])));
    }

    #[test]
    fn test_connective_parser() {
        assert_eq!(connective(&b"&&"[..]),
            Done(&b""[..], &b"&&"[..]));

        assert_eq!(connective(&b"||"[..]),
            Done(&b""[..], &b"||"[..]));

        assert_eq!(connective(&b""[..]),
            Incomplete(Needed::Size(2)));

        assert_eq!(connective(&b"I'm not valid!"[..]),
            Error(Position(Alt, &b"I'm not valid!"[..])));

    }

    #[test]
    fn test_empty_parser() {
        assert_eq!(empty(&b"\n"[..]),
            Done(&b""[..], &b""[..]));

        assert_eq!(empty(&b"  \n"[..]),
            Done(&b""[..], &b""[..]));

        assert_eq!(empty(&b"\t\n"[..]),
            Done(&b""[..], &b""[..]));

        assert_eq!(empty(&b";"[..]),
            Done(&b""[..], &b""[..]));

        assert_eq!(empty(&b"  ;"[..]),
            Done(&b""[..], &b""[..]));

        assert_eq!(empty(&b"\t;"[..]),
            Done(&b""[..], &b""[..]));

        assert_eq!(empty(&b"I'm definitely not empty"[..]),
            Error(Position(Alt, &b"I'm definitely not empty"[..])));
    }

    #[test]
    fn test_statement_parser() {
        assert_eq!(statement(&b"\n"[..]),
            Done(&b""[..], ("", vec![])));

        assert_eq!(statement(&b"ls --color\n"[..]),
            Done(&b""[..], ("ls", vec!["--color".to_string()])));

        assert_eq!(statement(&b"ls -a\n"[..]),
            Done(&b""[..], ("ls", vec!["-a".to_string()])));
    }
}
