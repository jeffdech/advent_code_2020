use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").unwrap()
        .split('\n')
        .map(|s| str::parse::<i64>(s.trim()))
        .map(Result::unwrap)
        .collect();
    
    // let pair = find_matching_pair(input);
    // match pair {
    //     Some((x, y)) => {println!("Result of {x} * {y} = {}", x * y);},
    //     None => {println!("Could not find a valid pair...");}
    // }

    match find_matching_triplet(input) {
        Some((a, b, c)) => {println!("Result of {a} * {b} * {c} = {}", a * b * c);},
        None => {println!("Could not find a valid triplet...");}
    }
}

fn find_matching_pair(input: Vec<i64>) -> Option<(i64, i64)> {
    input.into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
}

fn find_matching_triplet(input: Vec<i64>) -> Option<(i64, i64, i64)> {
    input.into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
}
