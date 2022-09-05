use itertools::Itertools;

fn main() {
    let numbers: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let n = 25;
    let answer = numbers.windows(n + 1).find_map(|s| {
        if (&s[..n]).iter().tuple_combinations().any(|(a, b)| a + b == s[n]) {
            None
        } else {
            Some(s[n])
        }
    });

    println!("answer = {:?}", answer);

    let answer2 = (2..numbers.len())
        .into_iter()
        .map(|n| {
            numbers.windows(n)
                .enumerate()
                .map(move |(i, s)| (n, i, s.iter().sum::<usize>()))
        })
        .flatten()
        .find(|&(_, _, sum)| sum == answer.unwrap());
    
    let (n, i, _) = answer2.unwrap();
    let sum = numbers[i] + numbers[i+n+1];

    println!("Range is {} - {} and sum = {}", n, i, sum);

    let set = &numbers[i..][..n];
    let answer3 = set.iter().max().unwrap() + set.iter().min().unwrap();
    dbg!(answer3);
}
