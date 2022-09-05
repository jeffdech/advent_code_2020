use std::collections::HashMap;

fn main() {
    let mut adaptors: Vec<usize> = std::iter::once(0)
        .chain(
            include_str!("input.txt")
                .lines()
                .map(|x| x.parse::<usize>().unwrap())
        )
        .collect();
    
    adaptors.sort_unstable();

    if let Some(x) = adaptors.iter().max() {
        adaptors.push(x + 3);
    }

    let mut ones = 0;
    let mut threes = 0;
    adaptors.windows(2).for_each(|w| {
        if let [x, y] = w {
            match y - x {
                1 => { ones += 1; },
                3 => { threes += 1; },
                n => { println!("I was lied to and the difference was {}", n);}
            }
        } else {
            unreachable!();
        }

    });

    println!("I got {} ones and {} threes = {}", ones, threes, ones * threes);

    let mut num_paths = HashMap::new();
    let length = adaptors.len();
    num_paths.insert(adaptors.last().copied().unwrap(), 1);

    for i in (0..(length - 1)).into_iter().rev() {
        let i_val = adaptors[i];
        let range = (i + 1)..=std::cmp::min(i+3, length-1);

        let num_neighbors: usize = range
            .filter_map(|j| {
                let j_val = adaptors[j];
                let gap = j_val - i_val;
                if (1..=3).contains(&gap) {
                    Some(num_paths.get(&j_val).unwrap())
                } else {
                    None
                }
            })
            .sum();
        num_paths.insert(i_val, num_neighbors);
    }

    for &n in adaptors.iter().rev() {
        let &m = num_paths.get(&n).unwrap();
        println!("From {} there is {} {}", n, m, if m == 1 {"path"} else {"paths"});
    }
}
