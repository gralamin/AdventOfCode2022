extern crate filelib;

pub use filelib::load;

use std::collections::HashSet;

/// Solution to puzzle_a entry point
/// ```
/// let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
/// assert_eq!(day06::puzzle_a(input), 7);
/// ```
pub fn puzzle_a(input: &str) -> usize {
    return find_unique_char_pos(input, 4);
}

fn find_unique_char_pos(input: &str, num_unique: usize) -> usize {
    // for 0 indexing
    let num_unique_offset = num_unique - 1;
    for i in num_unique_offset..input.len() {
        let chars: Vec<char> = input[i - num_unique_offset..=i].chars().collect();
        let set: HashSet<char> = chars.into_iter().collect();
        if set.len() == num_unique {
            return i + 1;
        }
    }
    panic!("Never found");
}

/// Solution to puzzle_b entry point
/// ```
/// let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
/// assert_eq!(day06::puzzle_b(input), 19);
/// ```
pub fn puzzle_b(input: &str) -> usize {
    return find_unique_char_pos(input, 14);
}
