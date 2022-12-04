extern crate filelib;

pub use filelib::load_no_blanks;

pub type DoublePair = (u32, u32, u32, u32);

fn fully_contains(a_lower: u32, a_higher: u32, b_lower: u32, b_higher: u32) -> bool {
    return a_lower <= b_lower && a_higher >= b_higher;
}

fn pairwise_fully_contains(a_lower: u32, a_higher: u32, b_lower: u32, b_higher: u32) -> bool {
    return fully_contains(a_lower, a_higher, b_lower, b_higher)
        || fully_contains(b_lower, b_higher, a_lower, a_higher);
}

/// Parse the line to just the lows and highs.
/// ```
/// let line = "2-4,6-8";
/// assert_eq!(day04::parse_line(line), (2, 4, 6, 8));
/// ```
pub fn parse_line(s: &str) -> DoublePair {
    let parsed: Vec<u32> = s
        .split(",")
        .map(|half| parse_pair(half))
        .flatten()
        .collect();
    if parsed.len() != 4 {
        return (999, 999, 999, 999);
    }
    return (parsed[0], parsed[1], parsed[2], parsed[3]);
}

fn parse_pair(s: &str) -> Vec<u32> {
    return s.split("-").map(|v| v.parse::<u32>().unwrap()).collect();
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<day04::DoublePair> = vec![(2,4,6,8), (2,3,4,5), (5,7,7,9), (2,8,3,7), (6,6,4,6), (2,6,4,8)];
/// assert_eq!(day04::puzzle_a(&vec1), 2);
/// ```
pub fn puzzle_a(input: &Vec<DoublePair>) -> usize {
    return input
        .iter()
        .filter(|(a, b, c, d)| pairwise_fully_contains(*a, *b, *c, *d))
        .count();
}

fn overlap_at_all(a_lower: u32, a_higher: u32, b_lower: u32, b_higher: u32) -> bool {
    // Implemented only from a's perspective, call also with arguments reversed (pairwise)
    // first check the lower cases, where a_lower is between b_lower and b_higher
    if b_lower <= a_lower && b_higher >= a_lower {
        return true;
    }
    // Check the higher cases, where a_higher is between b_lower and b_higher
    if b_lower <= a_higher && b_higher >= a_higher {
        return true;
    }
    // Finally check if one fully contains the other.
    return fully_contains(a_lower, a_higher, b_lower, b_higher);
}

fn pairwise_overlap_at_all(a_lower: u32, a_higher: u32, b_lower: u32, b_higher: u32) -> bool {
    return overlap_at_all(a_lower, a_higher, b_lower, b_higher)
        || overlap_at_all(b_lower, b_higher, a_lower, a_higher);
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<day04::DoublePair> = vec![(2,4,6,8), (2,3,4,5), (5,7,7,9), (2,8,3,7), (6,6,4,6), (2,6,4,8)];
/// assert_eq!(day04::puzzle_b(&vec1), 4);
/// ```
pub fn puzzle_b(input: &Vec<DoublePair>) -> usize {
    return input
        .iter()
        .filter(|(a, b, c, d)| pairwise_overlap_at_all(*a, *b, *c, *d))
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairwise_fully_contains() {
        assert_eq!(pairwise_fully_contains(2, 4, 1, 5), true);
        assert_eq!(pairwise_fully_contains(1, 5, 2, 4), true);
        assert_eq!(pairwise_fully_contains(1, 5, 1, 4), true);
        assert_eq!(pairwise_fully_contains(1, 5, 2, 5), true);
        assert_eq!(pairwise_fully_contains(7, 8, 2, 5), false);
    }

    #[test]
    fn test_pairwise_overlap_at_all() {
        assert_eq!(pairwise_overlap_at_all(2, 4, 1, 5), true);
        assert_eq!(pairwise_overlap_at_all(1, 5, 2, 4), true);
        assert_eq!(pairwise_overlap_at_all(1, 5, 1, 4), true);
        assert_eq!(pairwise_overlap_at_all(1, 5, 2, 5), true);
        assert_eq!(pairwise_overlap_at_all(7, 8, 2, 5), false);
        assert_eq!(pairwise_overlap_at_all(1, 2, 2, 5), true);
        assert_eq!(pairwise_overlap_at_all(1, 3, 2, 5), true);
        assert_eq!(pairwise_overlap_at_all(5, 6, 3, 5), true);
        assert_eq!(pairwise_overlap_at_all(4, 6, 3, 5), true);
    }
}
