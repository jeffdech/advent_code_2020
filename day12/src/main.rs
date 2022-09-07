mod ship;
use crate::ship::*;

fn main() {
    // let insts = parse_instructions(include_str!("example.txt"));
    let insts = parse_instructions(include_str!("input.txt"));

    let start = ShipState {
        pos: (0, 0).into(),
        dir: Direction::East,
        waypoint: (10, 1).into()
    };
    
    let end = insts.fold(start, |state, ins| state + ins);
    let delta = start.pos - end.pos;

    println!("Start state {:?}", start);
    println!("End state {:?}", end);
    println!("Manhattan distance {}", delta.manhattan_distance());
}
