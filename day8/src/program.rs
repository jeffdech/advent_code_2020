#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Acc(isize),
    Jump(isize),
    NoOp(isize)
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct State {
    pub pos: usize,
    pub acc: isize
}

impl State {
    pub fn next(self, program: &Program) -> Option<Self> {
        if !(0..program.len()).contains(&self.pos) {
            return None;
        }

        Some(match program[self.pos] {
            Instruction::Acc(n) => Self {
                pos: self.pos + 1,
                acc: self.acc + n,
            },
            Instruction::Jump(n) => Self {
                pos: (self.pos as isize + n).try_into().unwrap(),
                ..self
            },
            Instruction::NoOp(n) => Self {
                pos: self.pos + 1,
                ..self
            }
        })
    }
}

pub type Program = Vec<Instruction>;

pub fn parse_program(input: &str) -> Program {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(input: &str) -> Instruction {
    let quantity = input[4..].parse::<isize>().unwrap();
    match &input[0..3] {
        "nop" => Instruction::NoOp(quantity),
        "jmp" => Instruction::Jump(quantity),
        "acc" => Instruction::Acc(quantity),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::iter::zip;

    #[test]
    fn test_parse_line() {
        use Instruction::*;

        let lines = vec![
            "acc +8",
            "acc -37",
            "jmp +328",
            "jmp -574",
            "nop +16",
            "nop -99"
        ];

        let results = vec![
            Acc(8),
            Acc(-37),
            Jump(328),
            Jump(-574),
            NoOp(16),
            NoOp(-99)
        ];

        for (l, r) in zip(lines, results) {
            assert_eq!(parse_line(l), r);
        }
    }
}