use nom::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Statement<'a> {
    And(Box<Statement<'a>>, Box<Statement<'a>>),
    Or(Box<Statement<'a>>, Box<Statement<'a>>),
    Simple(&'a str, Vec<&'a str>),
}

fn alphanum_or_underscore_or_dash(c: char) -> bool {
    c == '_' || c == '-' || c.is_alphanum()
}
named!(whitespace<&str, &str>, is_a_s!(" \t"));
named!(command_terminator<&str, &str>, tag_s!(";"));
named!(executable<&str, &str>, take_while1_s!(alphanum_or_underscore_or_dash));
named!(argument<&str, &str>, take_while1_s!(alphanum_or_underscore_or_dash));
named!(arguments<&str, Vec<&str> >, many0!(argument));
named!(and<&str, &str>, tag_s!("&&"));
named!(or<&str, &str>, tag_s!("||"));
named!(simple_statement<&str, Statement>,
       chain!(
           whitespace? ~
           ex: executable ~
           whitespace? ~
           args: arguments ~
           whitespace?,
           || { Statement::Simple(ex, args) }
           )
      );


named!(statement<&str, Statement>, chain!(
        // TODO: This isn't the correct order!
        // s: alt!(simple_statement | compound_statement),
        s: alt_complete!(compound_statement | simple_statement),
        || { s }
        )
      );
named!(and_statement<&str, Statement>, chain!(
        s1: simple_statement ~
        and ~
        s2: statement,
        || { Statement::And(Box::new(s1), Box::new(s2)) }
        )
      );
named!(or_statement<&str, Statement>, chain!(
        s1: simple_statement ~
        or ~
        s2: statement,
        || { Statement::Or(Box::new(s1), Box::new(s2)) }
        )
      );
named!(compound_statement<&str, Statement>, alt!(and_statement | or_statement));
named!(pub statement_list<&str, Vec<Statement> >, separated_list!(command_terminator, statement));
