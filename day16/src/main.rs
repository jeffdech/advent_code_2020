mod ticket;
mod sorter;
use crate::ticket::*;
use crate::sorter::*;

fn main() {
    let info = TicketInfo::build(include_str!("input.txt")).unwrap();
    // let info = TicketInfo::build(include_str!("example2.txt")).unwrap();
    // println!("{:?}", info);

    println!("{:?}", info);
    println!("Invalid score: {}", info.error_score());

    // println!("{:?}", info.valid_nearbys());

    let assignments = RuleAssignment::new(&info);
    let my_assign = assignments.ticket_assignment(&info.yours);

    let key = "departure";
    let result: u128 = my_assign.0.iter()
        .filter(|(k, v)| k.starts_with(key))
        .map(|(_, v)| *v as u128)
        .product();
    
    println!("Part II Result - {:?}", result);
}
