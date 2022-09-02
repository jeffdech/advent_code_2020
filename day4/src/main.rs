// extern crate pest;
// #[macro_use]
// extern crate pest_derive;

// use pest::Parser;
// use pest::iterators::Pair;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Year(u64);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Length {
    Cm(u64),
    In(u64),
    Unitless(u64)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Color<'a>(&'a str);

#[derive(Debug, Clone, Copy, PartialEq)]
struct ID<'a>(&'a str);

#[derive(Debug, PartialEq)]
struct Passport<'a>{
    birth_year: Year,
    issue_year: Year,
    expiry_year: Year,
    height: Length,
    hair_color: Color<'a>,
    eye_color: Color<'a>,
    passport_id: ID<'a>,
    country_id: Option<ID<'a>>,
}

impl<'a> Passport<'a> {
    fn is_valid(&self) -> bool {
        self.valid_birth_year() &
        self.valid_expiry_year() &
        self.valid_hair_color() &
        self.valid_height() &
        self.valid_issue_year() & 
        self.valid_eye_color() &
        self.valid_issue_year()
    }

    fn valid_birth_year(&self) -> bool {
        let yr = self.birth_year.0;
        (yr >= 1920) & (yr <= 2002)
    }

    fn valid_issue_year(&self) -> bool {
        let yr = self.issue_year.0;
        (yr >= 2010) & (yr <= 2020)
    }

    fn valid_expiry_year(&self) -> bool {
        let yr = self.expiry_year.0;
        (yr >= 2020) & (yr <= 2030)
    }

    fn valid_height(&self) -> bool {
        match self.height {
            Length::In(v) => (v >= 59) & (v <= 76),
            Length::Cm(v) => (v >= 150) & (v <= 193),
            Length::Unitless(_) => false,
        }
    }

    fn valid_hair_color(&self) -> bool {
        if self.hair_color.0.len() == 7 {
            let string = self.hair_color.0;
            let hex_color: bool = string[1..7]
                .chars()
                .into_iter()
                .all(|c| c.is_ascii_hexdigit());
            (string.chars().nth(0) == Some('#')) & hex_color
        } else {
            false
        }
    }

    fn valid_eye_color(&self) -> bool {
        let valids = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        valids.contains(&self.eye_color.0)
    }


}

#[derive(Debug, Default, PartialEq)]
struct PassportBuilder<'a> {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiry_year: Option<Year>,
    height: Option<Length>,
    hair_color: Option<Color<'a>>,
    eye_color: Option<Color<'a>>,
    passport_id: Option<ID<'a>>,
    country_id: Option<ID<'a>>,  
    bad_keys: Vec<(&'a str, &'a str)>,  
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("missing field: {0}")]
    MissingField(&'static str),
}

// #[derive(Parser)]
// #[grammar = "passport.pest"]
// pub struct PassportParser;

impl<'a> PassportBuilder<'a> {
    fn build(self) -> Result<Passport<'a>, Error> {
        // Ok(Passport{
        //     birth_year: self.birth_year.ok_or(Error::MissingField("birth year")?,
        //     issue_year: self.issue_year.ok_or(Error::MissingField("issue year"))?,
        //     expiry_year: self.expiry_year.ok_or(Error::MissingField("expiry year"))?,
        //     height: self.height.ok_or(Error::MissingField("height"))?,
        //     hair_color: self.hair_color.ok_or(Error::MissingField("hair color"))?,
        //     eye_color: self.eye_color.ok_or(Error::MissingField("eye color"))?,
        //     passport_id: self.passport_id.ok_or(Error::MissingField("passport ID"))?,
        //     country_id: self.country_id,
        // })

        macro_rules! build {
            (
                required => {
                    $($req:ident),*$(,)*
                }$(,)*
                optional => {
                    $($opt: ident),*$(,)*
                }$(,)*
            ) => {
                Ok(Passport{
                    $($req: self.$req.ok_or(Error::MissingField(stringify!($req)))?),*,
                    $($opt: self.$opt),*
                })
            }
        }

        build! {
            required => {
                birth_year,
                issue_year,
                expiry_year,
                height,
                hair_color,
                eye_color,
                passport_id
            },
            optional => {
                country_id
            }
        }
    }
}

type PassportKV<'a> = Vec<(&'a str, &'a str)>;

fn passport_tokens<'a>(input: &'a str) -> Vec<PassportKV<'a>> {
    input.split("\n\n")
        .map(|s| {
            s.split(&['\n', ' '][..])
                .map(|e|{
                    let mut sobj = e.split(':');
                    (sobj.next().unwrap(),
                     sobj.next().unwrap())
                })
                .collect::<PassportKV<'a>>()
        })
        .collect::<Vec<_>>()
}

fn parse_passport_build<'a>(tokens: PassportKV<'a>) -> PassportBuilder {
    let mut pp: PassportBuilder = Default::default();

    for (k, v) in tokens {
        match k {
            "hcl" => {pp.hair_color = Some(Color(v))},
            "ecl" => {pp.eye_color = Some(Color(v))},
            "cid" => {pp.country_id = Some(ID(v))},
            "hgt" => {
                let n = v.len();

                let val = if let Ok(num) = v.parse::<u64>() {
                    Some(Length::Unitless(num))
                } else if let Ok(num) = &v[0..n-2].parse::<u64>() {
                    match &v[n-2..n] {
                        "in" => Some(Length::In(*num)),
                        "cm" => Some(Length::Cm(*num)),
                        _ => None,
                    }
                } else {
                    None
                };

                if let Some(n) = val {
                    pp.height = val;
                } else {
                    pp.bad_keys.push((k, v));
                    pp.height = None;
                }
            },
            "byr" => { pp.birth_year = parse_year(v) },
            "eyr" => { pp.expiry_year = parse_year(v) },
            "iyr" => { pp.issue_year = parse_year(v) },
            "pid" => {
                if (v.len() == 9) & (v.parse::<u64>().is_ok()){
                    pp.passport_id = Some(ID(v));
                } else {
                    pp.passport_id = None;
                    pp.bad_keys.push((k, v));
                }
            },
            _ => {pp.bad_keys.push((k, v))}
        }
    }

    pp
}

fn parse_file<'a>(input: &'a str) -> Vec<PassportBuilder<'a>> {
    passport_tokens(input)
        .into_iter()
        .map(parse_passport_build)
        // .map(PassportBuilder::build)
        .collect()
}

fn parse_year(s: &str) -> Option<Year> {
    match s.parse::<u64>() {
        Ok(n) => Some(Year(n)),
        Err(_e) => None
    }
}

fn main() {
    let text = include_str!("input.txt");
    let builds = parse_file(text);

    // builds.into_iter()
    //     .for_each(|i| {
    //         println!("{:?}", i);
    //         println!("{:?}", i.build());
    //         println!("");
    //     });

    let valid = builds.into_iter()
        .map(PassportBuilder::build)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .filter(Passport::is_valid)
        .count();

    println!("There are {valid} passports parsed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        assert!(PassportBuilder {
            ..Default::default()
        }
        .build()
        .is_err());
        assert!(PassportBuilder {
            birth_year: Some(Year(2014)),
            issue_year: Some(Year(2017)),
            expiry_year: Some(Year(2023)),
            height: Some(Length::Cm(195)),
            hair_color: Some(Color("#ffffff")),
            eye_color: Some(Color("#ee7812")),
            passport_id: Some(ID("00023437")),
            country_id: None,
        }
        .build()
        .is_ok());
    }
}