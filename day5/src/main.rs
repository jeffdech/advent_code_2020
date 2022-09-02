use bitvec::prelude::*;

#[derive(Debug, Default, PartialEq, Clone, Copy, Ord, Eq, PartialOrd)]
struct Seat {
    row: u8,
    col: u8
}

impl Seat {
    const ROW_BITS: usize = 7;
    const COL_BITS: usize = 3;
    const TOTAL_BITS: usize = Self::ROW_BITS + Self::COL_BITS;

    fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        let mut res: Seat = Default::default();

        {
            let row = BitSlice::<_, Msb0>::from_element_mut(&mut res.row);
            for (i, &b) in bytes[0..Self::ROW_BITS].iter().enumerate() {
                row.set(
                    (8 - Self::ROW_BITS) + i,
                    match b {
                        b'F' => false,
                        b'B' => true,
                        _ => panic!("unexpected row letter: {}", b as char),
                    }
                )
            }
        }

        {
            let col = BitSlice::<_, Msb0>::from_element_mut(&mut res.col);
            for (i, &b) in bytes[Self::ROW_BITS..Self::TOTAL_BITS].iter().enumerate() {
                col.set(
                    (8 - Self::COL_BITS) + i,
                    match b {
                        b'R' => true,
                        b'L' => false,
                        _ => panic!("unexpected column letter: {}", b as char),
                    }
                )
            }
        }

        res
    }

    fn id(&self) -> usize {
        8 * (self.row as usize) + (self.col as usize)
    }
}

impl From<(u8, u8)> for Seat {
    fn from(input: (u8, u8)) -> Self {
        Seat {
            row: input.0,
            col: input.1
        }
    }
}

fn main() {
    let seats = include_str!("input.txt")
        .lines()
        .map(Seat::parse)
        .map(|s| s.id());
    
    let max_id = itertools::max(seats);
    println!("The maximum seat ID is {:?}", max_id);

    let mut ids: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|l| Seat::parse(l).id())
        .collect::<Vec<usize>>();
    
    ids.sort();

    let mut last_id: Option<usize> = None;
    for id in ids {
        if let Some(last_id) = last_id {
            let gap = id - last_id;
            if gap > 1 {
                println!("Unfilled seat id is {}", id - 1);
                return;
            }
        }
        last_id = Some(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::iter::zip;

    #[test]
    fn test_parse() {
        let inputs = vec!["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
        let outputs = vec![(70, 7), (14, 7), (102, 4)];

        zip(inputs, outputs)
            .for_each(|(i, o)| {
                let parsed = Seat::parse(i);

                assert_eq!(parsed, Seat::from(o));
            })
    }

    #[test]
    fn test_id() {
        let posns = vec![(70, 7), (14, 7), (102, 4)];
        let ids = vec![567, 119, 820];

        zip(posns, ids)
            .for_each(|(p, id)| {
                let seat = Seat {
                    row: p.0,
                    col: p.1
                };

                assert_eq!(seat.id(), id);
            })
    }
}