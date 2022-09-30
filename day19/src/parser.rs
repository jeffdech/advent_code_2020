use std::collections::HashMap;
use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, alpha1, anychar, newline},
    combinator::map,
    multi::{separated_list1, many1},
    sequence::{delimited, separated_pair, tuple, terminated},
    IResult,
};

use nom::character::complete::char as cchar;

use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum ParserRule {
    Value(char),
    List(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>)
}

#[derive(Debug)]
pub struct RuleSet {
    rules: HashMap<usize, ParserRule>,
}

impl RuleSet {
    pub fn regex(&self) -> Regex {
        let re_string = self.build_regex_string(0) + "$";
        Regex::new(re_string.as_str()).unwrap()
    }

    pub fn build_regex_string(&self, start_rule: usize) -> String {
        use ParserRule::*;

        match self.rules.get(&start_rule).unwrap() {
            Value(c) => vec![c].into_iter().collect(),
            List(ns) => self.list_string(ns.to_vec()),
            Alt(ms, ns) => {
                let mstring = self.list_string(ms.to_vec());
                let nstring = self.list_string(ns.to_vec());

                String::from("(") + &mstring + &String::from("|") + &nstring + &String::from(")")
            }
        }
    }

    fn list_string(&self, ns: Vec<usize>) -> String {
        ns.iter()
            .map(|&n| self.build_regex_string(n))
            .collect::<Vec<String>>()
            .join("")
    }
}

pub mod parsing {
    use super::*;

    type Res<T, U> = IResult<T, U, nom::error::Error<T>>;

    pub fn parse<'a>(input: &'a str) -> Res<&'a str, (RuleSet, Vec<&'a str>)> {
        let body_lines = many1(terminated(alpha1, newline));

        tuple((rule_set, body_lines))(input)
    }

    pub fn rule_set(input: &str) -> Res<&str, RuleSet> {
        map(
            terminated(many1(rule), newline),
            |rs| RuleSet{ rules: rs.into_iter().collect() }
        )(input)
    }

    pub fn rule(input: &str) -> Res<&str, (usize, ParserRule)> {
        let list = map(digit_list, |d| ParserRule::List(d));
        let rule_body = alt((value, alt_list, list));

        map(
            tuple((digit1, tag(": "), rule_body, newline)),
            |(d, _, r, _)| {
                (d.parse().unwrap(), r)
            }
        )(input)
    }

    pub fn digit_list(input: &str) -> Res<&str, Vec<usize>> {
        map(
            separated_list1(tag(" "), digit1),
            |ds: Vec<&str>| ds.iter().map(|n| n.parse().unwrap()).collect()
        )(input)
    }

    pub fn value(input: &str) -> Res<&str, ParserRule> {
        map(
            delimited(tag("\""), anychar, tag("\"")),
            |c| ParserRule::Value(c)
        )(input)
    }

    pub fn alt_list(input: &str) -> Res<&str, ParserRule> {
        map(
            separated_pair(digit_list, tag(" | "), digit_list),
            |(d1, d2)| ParserRule::Alt(d1, d2)
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;
    use indoc::indoc;

    #[test]
    fn test_parse_rule() {
        let input = vec![
            "0: 4 1 5\n",
            "1: 2 3 | 3 2\n",
            "5: \"b\"\n",
        ];

        let output: Vec<(usize, ParserRule)> = vec![
            (0, ParserRule::List(vec![4, 1, 5])),
            (1, ParserRule::Alt(vec![2, 3], vec![3, 2])),
            (5, ParserRule::Value('b'))
        ];

        for (i, o) in std::iter::zip(input, output) {
            assert_eq!(parsing::rule(i), Ok(("", o)));
        }
    }

    #[test]
    fn test_parse_digit_list() {
        let inputs = vec![
            "1 2 3",
            "3 4 5 6 7"
        ];

        let outputs = vec![
            vec![1, 2, 3],
            vec![3, 4, 5, 6, 7]
        ];

        for (i, o) in zip(inputs, outputs) {
            assert_eq!(parsing::digit_list(i), Ok(("", o)));
        }
    }

    #[test]
    fn test_parse_value() {
        let inputs = vec![
            "\"a\"",
            "\"b\""
        ];

        let outputs = vec!['a', 'b'];

        for (i, o) in zip(inputs, outputs) {
            assert_eq!(parsing::value(i), Ok(("", ParserRule::Value(o))));
        }
    }

    #[test]
    fn test_parse_alt_list() {
        let input = "1 2 | 3 4";
        let expected = ParserRule::Alt(vec![1, 2], vec![3, 4]);

        assert_eq!(parsing::alt_list(input), Ok(("", expected)));
    }

    #[test]
    fn test_regex_matches() {
        let ruleset_input = indoc!{"
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: \"a\"
        5: \"b\"\n
        "};

        let text = indoc!{"
        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb
        "};

        let expected = vec![true, false, true, false, false];
        let regex = parsing::rule_set(ruleset_input).unwrap().1.regex();
        let matches: Vec<bool> = text.lines().map(|l| regex.is_match(l)).collect();

        assert_eq!(matches, expected);
    }
}