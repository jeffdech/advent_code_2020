mod parser;
use crate::parser::*;

fn main() {
    let hw_result: isize = include_str!("input.txt")
        .lines()
        .map(|l| parse_expr(l).eval())
        .sum();

    println!("The homework result is {}", hw_result);

    let hw_result2: isize = include_str!("input.txt")
        .lines()
        .map(|l| parse_adv(l).eval())
        .sum();

    println!("The advanced homework result is {}", hw_result2);
}
