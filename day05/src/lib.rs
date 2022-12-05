extern crate filelib;

pub use filelib::load;
use std::fmt::{Display, Formatter};

pub type Move = (usize, usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Crate {
    label: char,
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return writeln!(f, "[{}]", self.label);
    }
}

impl Crate {
    fn get_label(&self) -> char {
        return self.label;
    }
}

#[derive(Debug, Clone)]
pub struct OverallState {
    stacks: [Vec<Crate>; 9],
}

impl OverallState {
    pub fn get_stack_clone(&self, index: usize) -> Vec<Crate> {
        return self.stacks[index - 1].clone();
    }

    fn remove_from_stack(&mut self, index: usize, number: usize) -> Vec<Crate> {
        let stack = &mut self.stacks[index - 1];
        return stack.split_off(stack.len() - number);
    }

    fn add_to_stack(&mut self, index: usize, mut crates: Vec<Crate>) {
        let stack = &mut self.stacks[index - 1];
        stack.append(&mut crates);
    }

    fn add_individual_item_to_stack(&mut self, index: usize, c: Crate) {
        let stack = &mut self.stacks[index - 1];
        stack.push(c);
    }

    fn get_top_of_stacks(self) -> String {
        let mut s_builder = "".to_string();
        s_builder.push(self.stacks[0].last().map_or(' ', |c| c.get_label()));
        s_builder.push(self.stacks[1].last().map_or(' ', |c| c.get_label()));
        s_builder.push(self.stacks[2].last().map_or(' ', |c| c.get_label()));
        s_builder.push(self.stacks[3].last().map_or(' ', |c| c.get_label()));
        s_builder.push(self.stacks[4].last().map_or(' ', |c| c.get_label()));
        s_builder.push(self.stacks[5].last().map_or(' ', |c| c.get_label()));
        s_builder.push(self.stacks[6].last().map_or(' ', |c| c.get_label()));
        s_builder.push(self.stacks[7].last().map_or(' ', |c| c.get_label()));
        s_builder.push(self.stacks[8].last().map_or(' ', |c| c.get_label()));
        return s_builder.trim().to_string();
    }

    fn reverse_stacks(&mut self) {
        for i in 0..9 {
            self.stacks[i] = self.stacks[i].clone().into_iter().rev().collect();
        }
    }
}

fn create_state() -> OverallState {
    return OverallState {
        stacks: [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
    };
}

impl Display for OverallState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            match writeln!(f, "{} {:?}", i + 1, self.stacks[i]) {
                Ok(_) => continue,
                Err(err) => return Err(err),
            }
        }
        return Ok(());
    }
}

/// Parse columns into stacks
/// ```
/// let columns = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";
/// let state = day05::parse_stacks(columns);
/// let expected_state = day05::create_example_state();
/// assert_eq!(state.get_stack_clone(1), expected_state.get_stack_clone(1));
/// assert_eq!(state.get_stack_clone(2), expected_state.get_stack_clone(2));
/// assert_eq!(state.get_stack_clone(3), expected_state.get_stack_clone(3));
/// assert_eq!(state.get_stack_clone(4), expected_state.get_stack_clone(4));
/// assert_eq!(state.get_stack_clone(5), expected_state.get_stack_clone(5));
/// assert_eq!(state.get_stack_clone(6), expected_state.get_stack_clone(6));
/// assert_eq!(state.get_stack_clone(7), expected_state.get_stack_clone(7));
/// assert_eq!(state.get_stack_clone(8), expected_state.get_stack_clone(8));
/// assert_eq!(state.get_stack_clone(9), expected_state.get_stack_clone(9));
/// ```
pub fn parse_stacks(stacks: &str) -> OverallState {
    let mut state = create_state();
    for l in stacks.lines() {
        if l.chars().nth(1).unwrap() == '1' {
            break;
        }
        //                01234567890  01234567890  01234567890
        // let columns = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";
        // character will always be at  1, 5, 9, etc.
        let num_columns = (l.len() + 1) / 4;
        for i in 0..num_columns {
            let c = l.chars().nth(1 + 4 * i).unwrap();
            if c != ' ' {
                state.add_individual_item_to_stack(i + 1, Crate { label: c });
            }
        }
    }
    state.reverse_stacks();

    return state;
}

/// Parse text into moves
/// ```
/// let moves = "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
/// assert_eq!(day05::parse_moves(moves), vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)])
/// ```
pub fn parse_moves(moves: &str) -> Vec<Move> {
    return moves
        .lines()
        .map(|s| {
            let (movefrom_str, to_str) = s.split_once(" to ").unwrap();
            let (move_unparsed_str, from_str) = movefrom_str.split_once(" from ").unwrap();
            let (_, move_str) = move_unparsed_str.split_once("move ").unwrap();
            let to_num = to_str.parse::<usize>().unwrap();
            let from_num = from_str.parse::<usize>().unwrap();
            let move_num = move_str.parse::<usize>().unwrap();
            return (move_num, from_num, to_num);
        })
        .collect();
}

// For doc tests
pub fn create_example_state() -> OverallState {
    return OverallState {
        stacks: [
            vec![Crate { label: 'Z' }, Crate { label: 'N' }],
            vec![
                Crate { label: 'M' },
                Crate { label: 'C' },
                Crate { label: 'D' },
            ],
            vec![Crate { label: 'P' }],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
    };
}

/// Solution to puzzle_a entry point
/// ```
/// let state = day05::create_example_state();
/// let moves = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
/// assert_eq!(day05::puzzle_a(&state, &moves), "CMZ");
/// ```
pub fn puzzle_a(state: &OverallState, moves: &Vec<Move>) -> String {
    let mut puzzle_state = state.clone();

    for (move_num, from_num, to_num) in moves {
        // In puzzle a, crates are moved 1 by 1, which I missed, so move_num is always 1:
        for _ in 1..=*move_num {
            let v = puzzle_state.remove_from_stack(*from_num, 1);
            puzzle_state.add_to_stack(*to_num, v);
        }
    }

    return puzzle_state.get_top_of_stacks();
}

/// Solution to puzzle_b entry point
/// ```
/// let state = day05::create_example_state();
/// let moves = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
/// assert_eq!(day05::puzzle_b(&state, &moves), "MCD");
/// ```
pub fn puzzle_b(state: &OverallState, moves: &Vec<Move>) -> String {
    let mut puzzle_state = state.clone();

    for (move_num, from_num, to_num) in moves {
        let v = puzzle_state.remove_from_stack(*from_num, *move_num);
        puzzle_state.add_to_stack(*to_num, v);
    }

    return puzzle_state.get_top_of_stacks();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_state() {
        let mut state = create_state();
        state.add_individual_item_to_stack(1, Crate { label: 'A' });
        assert_eq!(state.get_stack_clone(1).len(), 1);
        state.add_individual_item_to_stack(2, Crate { label: 'B' });
        assert_eq!(state.get_stack_clone(2).len(), 1);
        state.add_individual_item_to_stack(2, Crate { label: 'C' });
        assert_eq!(state.get_stack_clone(2).len(), 2);
        let grabbed = state.remove_from_stack(2, 2);
        assert_eq!(state.get_stack_clone(2).len(), 0);
        state.add_to_stack(1, grabbed);
        assert_eq!(
            state.get_stack_clone(1),
            vec![
                Crate { label: 'A' },
                Crate { label: 'B' },
                Crate { label: 'C' }
            ]
        );
    }
}
