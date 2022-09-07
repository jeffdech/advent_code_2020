use std::fmt;

use nom::{
    bytes::complete::{tag},
    character::complete::{one_of, digit1},
};

#[derive(Copy, Clone, PartialEq)]
pub enum Instruction {
    Assign { addr: u64, val: u64},
    SetMask(Mask)
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Assign {addr, val} => {
                write!(f, "mem[{addr}] = {val}")
            },
            Instruction::SetMask(mask) => {
                write!(f, "mask: {:?}", mask)
            }
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Mask {
    set: u64,
    clear: u64
}

impl fmt::Debug for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set: {:b}, clear {:b}", self.set, self.clear)
    }
}

impl Mask {
    pub fn apply(&self, x: u64) -> u64 {
        x | self.set & (!self.clear)
    }
}

pub struct Program {
    pub instructions: Vec<Instruction>
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.instructions.iter().for_each(|inst| println!("{:?}", inst));
        Ok(())
    }
}

type Res<T, U> = nom::IResult<T, U, nom::error::Error<T>>;

impl Program {
    pub fn parse(input: &str) -> Self {
        let instructions: Vec<Instruction> = input.lines()
            .map(nom::branch::alt((Program::parse_mask, Program::parse_assign)))
            .map(|r| r.unwrap().1)
            .collect();
        Self { instructions }
    }

    fn parse_mask(input: &str) -> Res<&str, Instruction> {
        let mask_str = nom::multi::many1(one_of("10X"));
        nom::combinator::map(
            nom::sequence::tuple((tag("mask = "), mask_str)),
            |(_, mstr)| 
            {
                let mut mask: Mask = Default::default();

                mstr.iter().rev().enumerate()
                    .for_each(|(n, c)|{
                        match c {
                            '1' => {mask.set |= 2_u64.pow(n as _)},
                            '0' => {mask.clear |= 2_u64.pow(n as _)},
                            'X' => {},
                            _ => unreachable!()
                        }
                    });
                
                Instruction::SetMask(mask)
            }
        )(input)
    }

    fn parse_assign(input: &str) -> Res<&str, Instruction> {
        nom::combinator::map(
            nom::sequence::tuple((tag::<&str, &str, nom::error::Error<&str>>("mem["), digit1, tag("] = "), digit1)),
            |(_, d1, _, d2)| {
                Instruction::Assign{
                    addr: d1.parse::<u64>().unwrap(),
                    val: d2.parse::<u64>().unwrap()
                }
            }
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mask() {
        let mask_line = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let expected = Instruction::SetMask(Mask {set: 1 << 6, clear: 2});

        assert_eq!(Program::parse_mask(mask_line), Ok(("", expected)));
    }

    #[test]
    fn test_parse_assign() {
        use std::iter::zip;

        let set_lines = vec![
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0"
        ];

        let set_values = vec![(8, 11), (7, 101), (8, 0)];
        
        for (l, (addr, val)) in zip(set_lines, set_values) {
            let inst = Instruction::Assign{addr, val};
            assert_eq!(Program::parse_assign(l), Ok(("", inst)));
        }
    }
}