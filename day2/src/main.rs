use std::ops::RangeInclusive;

use nom::{
    IResult,
    error::VerboseError,
};

use nom::character::complete;
use nom::character::complete::{
    digit1, space0, alpha1
};

use nom::bytes::complete::{
    tag
};

use nom::combinator;
use nom::combinator::{recognize};
use nom::sequence::{
    tuple, separated_pair
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    range: RangeInclusive<usize>,
    byte: u8
}

impl PasswordPolicy {
    pub fn is_valid(&self, s: &str) -> bool {
        let count = s.as_bytes()
            .iter()
            .filter(|&&c| c == self.byte)
            .count();
        
        self.range.contains(&count)
    }

    pub fn match_posns(&self, s: &str) -> bool {
        let v: Vec<u8> = Vec::from_iter(s.as_bytes().iter().copied());
        (v[*self.range.start() - 1] == self.byte) ^ (v[*self.range.end() - 1] == self.byte)
    }
}

#[derive(Debug)]
struct ParseError {}

fn parse_range(s: &str) -> Res<&str, RangeInclusive<usize>> {
    combinator::map(
        separated_pair(digit1, tag("-"), digit1),
        |(d1, d2): (&str, &str)| {
            RangeInclusive::new(
                d1.parse::<usize>().unwrap(),
                d2.parse::<usize>().unwrap()
            )
        }
    )(s)
}

fn parse_policy(s: &str) -> Res<&str, PasswordPolicy> {
    combinator::map(
        separated_pair(parse_range, tag(" "), complete::anychar),
        |(range, b)| {
            PasswordPolicy {
                range,
                byte: u8::try_from(b).unwrap()
            }
        }
    )(s)
}

fn parse_line(s: &str) -> Res<&str, (PasswordPolicy, &str)> {
    separated_pair(parse_policy, tag(": "), alpha1)(s)
}

fn main() {
    // let input = include_str!("input.txt")
    //     .lines()
    //     .map(parse_line)
    //     .map(Result::unwrap)
    //     .filter(|(ninput, (policy, password))| policy.is_valid(password))
    //     .count();

    // println!("Count is {input}");

    let count = include_str!("input.txt")
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .filter(|(ninput, (policy, password))| policy.match_posns(password))
        .count();
    
    println!("Count is {count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_policy() {
        let policy = PasswordPolicy {
            range: 1..=3,
            byte: b'a'
        };

        assert_eq!(policy.is_valid("banana"), true, "three 'a's");
        assert_eq!(policy.is_valid("ant"), true, "one 'a'");
        assert_eq!(policy.is_valid("hello"), false, "no 'a's");
        assert_eq!(policy.is_valid("aaaaah"), false, "too many 'a's");
    }

    #[test]
    fn test_parse_policy() 
    {
        let r: Res<&str, (&str, &str)> = separated_pair(digit1, tag("-"), digit1)("1-2");
        assert_eq!(r, Ok(("", ("1", "2"))));
    }

    #[test]
    fn test_parse_range()
    {
        let text = "4-8";
        let range = RangeInclusive::new(4, 8);
        assert_eq!(parse_range(text), Ok(("", range)));
    }

    #[test]
    fn test_parse_password_policy()
    {
        let text = "4-8 g";
        let expected_policy = PasswordPolicy {
            range: RangeInclusive::new(4, 8),
            byte: b'g'
        };

        assert_eq!(parse_policy(text), Ok(("", expected_policy)));
    }

    #[test]
    fn test_parse_line()
    {
        let text = "4-8 g: ogg";
        let expected_policy = PasswordPolicy {
            range: RangeInclusive::new(4, 8),
            byte: b'g'
        };

        assert_eq!(parse_line(text), Ok(("", (expected_policy, "ogg"))));
    }

    #[test]
    fn test_match_policy()
    {
        let policy = PasswordPolicy {
            range: RangeInclusive::new(2, 4),
            byte: b'a'
        };

        assert!(!policy.match_posns("banana"), "two occurrences");
        assert!(policy.match_posns("mangle"), "second position");
        assert!(policy.match_posns("endaor"), "fourth position");
        assert!(!policy.match_posns("trees"), "no occurences");
    }
}