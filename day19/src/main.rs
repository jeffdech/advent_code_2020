mod parser;
use crate::parser::*;

fn main() {
    let (rules, body) = parsing::parse(include_str!("input.txt")).unwrap().1;

    let re = rules.regex();

    let matches = body.iter()
        .filter(|s| re.is_match(s))
        .count();

    println!("There are {} matches", matches);
}
