use std::collections::HashSet;
use itertools::Itertools;

mod program;
use crate::program::*;

fn main() {
    // My solution

    // let program = parse_program(include_str!("input.txt"));
    // let mut state: State = Default::default();

    // let mut run_posns: HashSet<usize> = Default::default();

    // while !run_posns.contains(&state.pos) {
    //     run_posns.insert(state.pos);
    //     state = state.next(&program);
    // }

    // println!("Accumulator state will be {}", state.acc);

    // Fasterthanli solution
    let mut program = parse_program(include_str!("input.txt"));

    let mut state_iter = itertools::iterate(Some(State::default()), |s| s.unwrap().next(&program));
    let mut run_inst: HashSet<usize> = Default::default();

    let answer = state_iter.find(|state| !run_inst.insert(state.unwrap().pos)).unwrap().unwrap();
    println!("Accumulator state will be {}", answer.acc);

    // find_variant(&program);
    flip_instruction(&mut program[327]);
    println!("After fix, accumulator is {:?}", eval(&program));
}

fn flip_instruction(inst: &mut Instruction) {
    *inst = match *inst {
        Instruction::Jump(n) => Instruction::NoOp(n),
        Instruction::NoOp(n) => Instruction::Jump(n),
        x => x,
    }
}

fn find_variant(program: &Program) {
    let mut variants: Vec<_> = program
        .iter()
        .enumerate()
        .filter_map(|(idx, inst)| match inst {
            Instruction::Jump(n) | Instruction::NoOp(n) => Some(idx),
            _ => None
        })
        .map(|i| {
            let mut variant = program.clone();
            flip_instruction(&mut variant[i]);
            (i, variant)
        })
        .map(|(idx, var)| {
            itertools::iterate(Some(State::default()), move |s| {
                s.unwrap_or_else(|| panic!("Variant {} terminated at {:?}", idx, s))
                    .next(&var)
            })
        })
        .collect();
    
    loop {
        for v in &mut variants {
            v.next();
        }
    }
}

fn eval(program: &Program) -> Option<isize> {
    itertools::iterate(Some(State::default()), |state| {
        state.and_then(|s| s.next(&program))
    })
    .while_some()
    .last()
    .map(|s| s.acc)
}