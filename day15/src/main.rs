mod game;
use crate::game::*;

fn run_game(seed: Vec<usize>, turns: usize) -> GameState {
    let running_turns = turns - seed.len();
    let state = GameState::new(seed);

    let final_state = (0..running_turns).fold(state, |s, _| {
        let ns = s.next();
        // println!("{}", ns);
        ns
    });
    final_state
}

fn main() {
    let input = vec![11,0,1,10,5,19];
    let final_state = run_game(input, 30000000);

    println!("The final number spoken is {}", final_state.current);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_answer() {
        let final_state = run_game(vec![0, 3, 6], 10);
        assert_eq!(final_state.current, 0 as usize);
    }

    #[test]
    fn test_answers() {
        let examples = vec![
            (vec![1, 3, 2], 1),
            (vec![2, 1, 3], 10),
            (vec![1, 2, 3], 27),
            (vec![2, 3, 1], 78),
            (vec![3, 2, 1], 438),
            (vec![3, 1, 2], 1836)
        ];

        for (seed, result) in examples.iter()
        {
            println!("Testing {:?} -> {}", seed, result);
            let final_state = run_game(seed.to_vec(), 2020);
            assert_eq!(final_state.current, *result as usize);
        }
    }

    #[test]
    fn test_long_answers() {
        let examples = vec![
            (vec![0, 3, 6], 175594),
            (vec![1, 3, 2], 2578),
            (vec![2, 1, 3], 3544142),
            (vec![1, 2, 3], 261214),
            (vec![2, 3, 1], 6895259),
            (vec![3, 2, 1], 18),
            (vec![3, 1, 2], 362)
        ];

        for (seed, result) in examples.iter()
        {
            println!("Testing {:?} -> {}", seed, result);
            let final_state = run_game(seed.to_vec(), 30000000);
            assert_eq!(final_state.current, *result as usize);
        }        
    }
}