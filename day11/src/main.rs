mod seats;
use crate::seats::*;

fn main() {
    let plan = FloorPlan::<Tile>::parse(include_str!("input.txt"));

    let last = plan.last();
    println!("{:?}", last);
    println!("At steady state, {} seats are occupied", last.occupied_seats());
}
