extern crate filelib;

pub use filelib::load_no_blanks;

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["A Y", "B X", "C Z"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day22::puzzle_a(&vec1), 1);
/// ```
pub fn puzzle_a(_input: &Vec<String>) -> i32 {
    return 1;
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["A Y", "B X", "C Z"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day22::puzzle_b(&vec1), 2);
/// ```
pub fn puzzle_b(_input: &Vec<String>) -> i32 {
    return 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(1, 1);
    }
}
