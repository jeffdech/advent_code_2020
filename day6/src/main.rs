use std::collections::HashSet;

fn get_union(input: &str) -> HashSet<char> {
    let all_chars = HashSet::from_iter('a'..='z');
    let result = input.lines()
        .map(|l| HashSet::from_iter(l.chars()))
        .fold(all_chars, |acc, x| acc.intersection(&x).copied().collect());

    println!("{}", input);
    println!("========");
    println!("{:?}", result);
    println!("~~~~~~~~~~~~~~~~~~~");
    result
}

fn main() {
    let result: usize = include_str!("input.txt")
        .split("\r\n\r\n")
        .into_iter()
        .map(get_union)
        .map(|s| s.len())
        .sum();

    println!("{}!", result);
}
