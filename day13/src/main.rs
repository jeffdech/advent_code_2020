use std::iter::zip;

mod bus;
use crate::bus::*;

const MAX_TIMESTAMP: usize = 100000000000000;

fn main() {
    // let statement = ProblemStatement::parse(include_str!("example.txt"));
    let statement = ProblemStatement::parse(include_str!("input.txt"));

    println!("{:?}", statement);

    let wait_times: Vec<_> = statement.buses
        .iter()
        .map(|&bus_id| WaitTime {
            bus_id,
            wait: bus_id - (statement.departure_time % bus_id)
        })
        .collect();

    let answer = wait_times.iter().min_by_key(|wt| wt.wait);
    
    match answer {
        Some(wt) => println!("{:?} => {:?}", &wt, &wt.wait * &wt.bus_id),
        None => println!("No answer found!")
    }

    let (max_id, max_offset): (usize, usize) = zip(statement.buses.clone(), statement.posns.clone())
        .max_by_key(|&(mid, _)| mid)
        .unwrap();
    // (max_id..MAX_TIMESTAMP)
    // (1068780..1068782)
    let start_pos = max_id - max_offset;
    let answer2 = (start_pos..MAX_TIMESTAMP)
        .step_by(max_id)
        .filter_map(move |ts| {
            // print!("\n Timestamp: {}", ts);
            let valid = zip(statement.posns.clone(), statement.buses.clone())
                .fold(true, |acc, (offset, bus_id)| {
                    let val = (ts + offset) % bus_id;
                    // print!(" ({}) {} - {}", offset, bus_id, val);
                    // println!();
                    acc & (val == 0)
                });
                
            if valid {
                Some(ts)
            } else {
                None
            }
        })
        .next();
    
    match answer2 {
        Some(ts) => println!("First timestamp is {}", ts),
        None => println!("No answer found")
    }
}
