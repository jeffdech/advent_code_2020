use std::fmt;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use multimap::MultiMap;
use itertools::Itertools;

use nom::{
    Finish,
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1,
        anychar,
        digit1,
        line_ending,
        newline,
        one_of,
        space1
    },
    combinator::{opt, map, recognize},
    error::Error,
    multi::{many1, separated_list1},
    sequence::tuple,
};

type Res<T, U> = IResult<T, U, nom::error::Error<T>>;
type ParseError<'a> = nom::error::Error<&'a str>;

type RangeSet = (RangeInclusive<usize>, RangeInclusive<usize>);
pub type TicketNums = Vec<usize>;

#[derive(Debug)]
pub struct TicketInfo<'a>{
    pub rules: HashMap<&'a str, RangeSet>,
    pub yours: TicketNums,
    nearby: Vec<TicketNums>,
}

impl<'a> TicketInfo<'a> {
    pub fn build(input: &'a str) -> Result<Self, ParseError> {
        let res = Self::parse(input).finish();

        if let Ok((_, ti)) = res {
            Ok(ti)
        } else {
            Err(res.unwrap_err())
        }
    }
    pub fn parse(input: &'a str) -> Res<&'a str, TicketInfo<'a>> {
        map(
            tuple((Self::parse_rules, newline, Self::parse_yours, newline, Self::parse_nearby)),
            |(rules, _, yours, _, nearby)| Self { 
                rules: HashMap::from_iter(rules.into_iter()), 
                yours, 
                nearby, 
            }
        )(input)
    }

    pub fn error_score(&self) -> usize {
        self.invalid_nearbys().iter().sum()
    }
    pub fn valid_nearbys(&self) -> Vec<&TicketNums> {
        self.nearby
            .iter()
            .filter(|n| self.invalid_ticket(n).is_none())
            .collect()
    }

    pub fn obeys_rule(&self, k: &'a str, num: &'a usize) -> bool {
        let (r1, r2) = self.rules.get(k).unwrap();
        r1.contains(num) || r2.contains(num)
    }
    
    fn invalid_nearbys(&self) -> Vec<usize> {
        self.nearby.iter().filter_map(|n| self.invalid_ticket(n)).collect()
    }

    fn invalid_ticket(&self, nums: &TicketNums) -> Option<usize> {
        nums.iter()
            .filter_map(|n| {
                let valid = self.rules.iter().any(|(k, (r1, r2))| r1.contains(n) || r2.contains(n));
                match valid {
                    true => None,
                    false => Some(*n)
                }
            })
            .next()
    }

    fn parse_rules(input: &'a str) -> Res<&'a str, Vec<(&'a str, RangeSet)>> {
        many1(Self::parse_one_rule)(input)
    }

    fn parse_one_rule(input: &'a str) -> Res<&'a str, (&'a str, RangeSet)> {
        map(
            tuple((recognize(many1(alt((alpha1::<&'a str, nom::error::Error<&'a str>>, space1)))), tag(": "), 
                    digit1, tag("-"), digit1, tag(" or "), 
                    digit1, tag("-"), digit1, newline)),
            |(k, _, d1, _, d2, _, d3, _, d4, _)| {
                let r1 = RangeInclusive::new(d1.parse::<usize>().unwrap(), d2.parse::<usize>().unwrap());
                let r2 = RangeInclusive::new(d3.parse::<usize>().unwrap(), d4.parse::<usize>().unwrap());

                (k, (r1, r2))
            }
        )(input)
    }

    fn parse_yours(input: &'a str) -> Res<&'a str, Vec<usize>> {
        let header = tuple((tag::<&'a str, &'a str, nom::error::Error<&'a str>>("your ticket:"), newline));
        map(
            tuple((header, separated_list1(tag(","), digit1), newline)),
            |(_, sl, _)| sl.iter().map(|n| n.parse::<usize>().unwrap()).collect()
        )(input)
    }

    fn parse_nearby(input: &'a str) -> Res<&'a str, Vec<TicketNums>> {
        let header = tuple((tag::<&'a str, &'a str, nom::error::Error<&'a str>>("nearby tickets:"), newline));
        let ticket_line = map(
            tuple((separated_list1(tag::<&'a str, &'a str, nom::error::Error<&'a str>>(","), digit1), opt(newline))),
            |(sl, _)| sl.iter().map(|n| n.parse::<usize>().unwrap()).collect()
        );

        map(
            tuple((header, many1(ticket_line))), 
            |(_, tls)| tls
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_one_rule() {
        let text = "departure location: 33-224 or 230-954\n";
        let expected = ("departure location", (RangeInclusive::new(33, 224), RangeInclusive::new(230, 954)));

        assert_eq!(TicketInfo::parse_one_rule(text), Ok(("", expected)));
    }

    #[test]
    fn test_parse_rules() {
        let text = "departure location: 33-224 or 230-954\ndeparture station: 32-358 or 371-974\n";
        let expected = vec![
            ("departure location", (RangeInclusive::new(33, 224), RangeInclusive::new(230, 954))),
            ("departure station", (RangeInclusive::new(32, 358), RangeInclusive::new(371, 974)))
        ];

        assert_eq!(TicketInfo::parse_rules(text), Ok(("", expected)));
    }

    #[test]
    fn test_parse_yours() {
        let text = "your ticket:\n1,2,3\n";
        let expected = vec![1, 2, 3];

        assert_eq!(TicketInfo::parse_yours(text), Ok(("", expected)));
    }

    #[test]
    fn test_parse_nearby() {
        let text = "nearby tickets:\n1,2,3\n4,5,6\n";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];

        assert_eq!(TicketInfo::parse_nearby(text), Ok(("", expected)));
    }
}