mod bitparse;
use crate::bitparse::*;

use std::collections::HashMap;

fn main() {
    // let prog = Program::parse(include_str!("example.txt"));
    let prog = Program::parse(include_str!("input.txt"));

    let answer = prog.run();

    println!("Answer: {}", answer);
}
