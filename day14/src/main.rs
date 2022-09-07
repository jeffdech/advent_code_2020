mod bitparse;
use crate::bitparse::*;

use std::collections::HashMap;

fn main() {
    let prog = Program::parse(include_str!("example.txt"));
    // let prog = Program::parse(include_str!("input.txt"));

    let mut mask: Mask = Default::default();
    let mut mem = HashMap::<u64, u64>::new();

    for ins in &prog.instructions {
        match *ins {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::Assign { addr, val } => {
                mem.insert(addr, mask.apply(val));
            }
        }
    }

    println!("Answer: {}", mem.values().sum::<u64>());
}
