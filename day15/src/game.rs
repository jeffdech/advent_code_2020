use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct NHistory(Option<usize>, usize);

pub struct GameState {
    pub turn: usize,
    pub current: usize,
    history: HashMap<usize, NHistory>,
}

impl GameState {
    pub fn new(seed: Vec<usize>) -> Self {
        let turn = seed.len();
        let current = seed[turn - 1];

        Self {
            turn,
            current,
            history: HashMap::from_iter(
                (1..turn).map(|n| (seed[n-1], NHistory(None, n)))
            )
        }
    }

    pub fn next(mut self) -> Self {
        self.history.insert(
            self.current, 
            match self.history.get(&self.current) {
                None => NHistory(None, self.turn),
                Some(&NHistory(_, p)) => NHistory(Some(p), self.turn)
            }
        );

        let next_num = match self.history.get(&self.current).unwrap() {
            NHistory(None, _) => 0,
            NHistory(Some(p), c) => c - p
        };

        Self {
            turn: self.turn + 1,
            current: next_num,
            history: self.history
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Turn {} - {}: ", self.turn, self.current);

        self.history.iter()
            .for_each(|(k, nh)| {
                match nh {
                    NHistory(None, v) => {write!(f, "{} -> [{}] ", k, v);},
                    NHistory(Some(p), v) => {write!(f, "{} -> [{}, {}], ", k, p, v);},
                }
            });
        Ok(())
    }
}