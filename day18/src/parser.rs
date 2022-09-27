use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag},
    combinator::{map_res, map, cut},
    character::complete::{digit1, char},
    multi::{fold_many0},
    sequence::{pair, separated_pair, delimited, preceded, terminated},
    IResult,
};

#[derive(PartialEq, Clone)]
pub enum MathExpr {
    Value(isize),
    Add(Box<MathExpr>, Box<MathExpr>),
    Mult(Box<MathExpr>, Box<MathExpr>),
    Bracket(Box<MathExpr>)
}

impl MathExpr {
    pub fn eval(&self) -> isize {
        use MathExpr::*;

        match self {
            Value(x) => *x,
            Add(ba, bb) => ba.eval() + bb.eval(),
            Mult(ba, bb) => ba.eval() * bb.eval(),
            Bracket(ba) => ba.eval()
        }
    }
}

impl fmt::Debug for MathExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MathExpr::*;

        match self {
            Value(x) => write!(f, "{}", *x),
            Add(ba, bb) => write!(f, "( {:?} + {:?} )", ba, bb),
            Mult(ba, bb) => write!(f, "( {:?} * {:?} )", ba, bb),
            Bracket(ba) => write!(f, "( {:?} )", ba)
        }
    }
}

pub fn parse_expr(input: &str) -> MathExpr {
    parse::expr(input).unwrap().1
}

pub fn parse_adv(input: &str) -> MathExpr {
    parse::adv_expr(input).unwrap().1
}

mod parse {
    use super::*;

    type Res<T, U> = IResult<T, U, nom::error::Error<T>>;

    pub fn expr(input: &str) -> Res<&str, MathExpr> {
        let (i, init): (&str, MathExpr) = term(input)?;

        let x = fold_many0(
            pair(alt((tag(" + "), tag(" * "))), term),
            || init.clone(),
            |acc, (op, val): (&str, MathExpr)| {
                if op == " + " {
                    MathExpr::Add(Box::new(acc), Box::new(val))
                } else {
                    MathExpr::Mult(Box::new(acc), Box::new(val))
                }
            }
        )(i);

        x
    }

    pub fn adv_expr(input: &str) -> Res<&str, MathExpr> {
        let (i, init): (&str, MathExpr) = alt((add, term))(input)?;

        let x = fold_many0(
            pair(tag(" * "), add),
            || init.clone(),
            |acc, (op, val): (&str, MathExpr)| {
                MathExpr::Mult(Box::new(acc), Box::new(val))
            }
        )(i);

        x
    }

    pub fn add(input: &str) -> Res<&str, MathExpr> {
        let (i, init): (&str, MathExpr) = adv_term(input)?;

        let x = fold_many0(
            pair(tag(" + "), adv_term),
            || init.clone(),
            |acc, (op, val): (&str, MathExpr)| {
                MathExpr::Add(Box::new(acc), Box::new(val))
            }
        )(i);

        x       
    }

    pub fn term(input: &str) -> Res<&str, MathExpr> {
        alt((bracket, value))(input)
    }

    pub fn adv_term(input: &str) -> Res<&str, MathExpr> {
        alt((adv_bracket, value))(input)
    }

    pub fn bracket(input: &str) -> Res<&str, MathExpr> {
        map(
            delimited(tag("("), expr, tag(")")),
            |b| MathExpr::Bracket(Box::new(b))
        )(input)
    }

    pub fn adv_bracket(input: &str) -> Res<&str, MathExpr> {
        map(
            delimited(tag("("), adv_expr, tag(")")),
            |b| MathExpr::Bracket(Box::new(b))
        )(input)
    }

    pub fn value(input: &str) -> Res<&str, MathExpr> {
        map(digit1, |d: &str| MathExpr::Value(d.parse::<isize>().unwrap()))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mathexpr() {
        use std::iter::zip;

        let inputs: Vec<&str> = vec![
            "1 + (2 * 3) + (4 * (5 + 6))",
            "2 * 3 + (4 * 5)",
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
        ];

        let outputs: Vec<isize> = vec![51, 26, 437, 12240, 13632];

        for (i, o) in zip(inputs, outputs) {
            assert_eq!(parse_expr(i).eval(), o);
        }
    }

    #[test]
    fn test_parse_adv_mathexpr() {
        use std::iter::zip;

        let inputs: Vec<&str> = vec![
            "1 + (2 * 3) + (4 * (5 + 6))",
            "2 * 3 + (4 * 5)",
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
        ];

        let outputs: Vec<isize> = vec![51, 46, 1445, 669060, 23340];

        for (i, o) in zip(inputs, outputs) {
            let expr = parse_adv(i);
            dbg!(i);
            dbg!(expr.clone());
            assert_eq!(expr.eval(), o);
        }
    }
    
    #[test]
    fn test_string_parsing() {
        use std::iter::zip;

        let inputs = vec![
            "1 + 2",
            "1 * (2 + 3)"
        ];

        let outputs = vec![
            MathExpr::Add(Box::new(MathExpr::Value(1)), Box::new(MathExpr::Value(2))),
            MathExpr::Mult(
                Box::new(MathExpr::Value(1)), 
                Box::new(MathExpr::Bracket(
                    Box::new(MathExpr::Add(
                        Box::new(MathExpr::Value(2)), Box::new(MathExpr::Value(3)))))))
        ];

        for (i, o) in zip(inputs, outputs) {
            assert_eq!(parse::expr(i), Ok(("", o)));
        }
    }

    #[test]
    fn test_parse_bracket() {
        assert_eq!(parse::bracket("(1)"), Ok(("", MathExpr::Bracket(Box::new(MathExpr::Value(1))))));
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(parse::value("123"), Ok(("", MathExpr::Value(123))));
    }

    #[test]
    fn test_parse_add() {
        let input = "1 + 2 + 3";
        let expected = MathExpr::Add(
            Box::new(MathExpr::Add(
                Box::new(MathExpr::Value(1)),
                Box::new(MathExpr::Value(2))
            )),
            Box::new(MathExpr::Value(3)),
        );

        assert_eq!(parse::add(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_adv_expr() {
        let input = "2 * 3 + (4 * 5)";
        let expected = MathExpr::Mult(
            Box::new(MathExpr::Value(2)),
            Box::new(MathExpr::Add(
                Box::new(MathExpr::Value(3)),
                Box::new(MathExpr::Bracket(
                    Box::new(
                        MathExpr::Mult(
                            Box::new(MathExpr::Value(4)),
                            Box::new(MathExpr::Value(5))
                        )
                    )
                ))
            ))
        );

        assert_eq!(parse::adv_expr(input), Ok(("", expected)));
    }
}