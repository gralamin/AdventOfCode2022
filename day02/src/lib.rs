extern crate filelib;

pub use filelib::load_no_blanks;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

const WIN_POINTS: i32 = 6;
const DRAW_POINTS: i32 = 3;
const LOSE_POINTS: i32 = 0;

fn parse_input_puzzle_1(input: &Vec<String>) -> Vec<(RockPaperScissors, RockPaperScissors)> {
    return input.iter().map(|s| parse_line_puzzle_1(s)).collect();
}

fn parse_line_puzzle_1(s: &String) -> (RockPaperScissors, RockPaperScissors) {
    let mut split_value = s.split_whitespace();
    let first = abc_to_rps(split_value.next().unwrap_or(""));
    let second = xyz_to_rps(split_value.next().unwrap_or(""));
    return (first, second);
}

fn abc_to_rps(i: &str) -> RockPaperScissors {
    return match i {
        "A" => RockPaperScissors::Rock,
        "B" => RockPaperScissors::Paper,
        "C" => RockPaperScissors::Scissors,
        _ => RockPaperScissors::Rock,
    };
}

fn xyz_to_rps(i: &str) -> RockPaperScissors {
    return match i {
        "X" => RockPaperScissors::Rock,
        "Y" => RockPaperScissors::Paper,
        "Z" => RockPaperScissors::Scissors,
        _ => RockPaperScissors::Rock,
    };
}

fn get_match_point(opponent: &RockPaperScissors, you: &RockPaperScissors) -> i32 {
    if *opponent == *you {
        return DRAW_POINTS;
    } else if *opponent == RockPaperScissors::Rock && *you == RockPaperScissors::Scissors {
        return LOSE_POINTS;
    } else if *opponent == RockPaperScissors::Scissors && *you == RockPaperScissors::Paper {
        return LOSE_POINTS;
    } else if *opponent == RockPaperScissors::Paper && *you == RockPaperScissors::Rock {
        return LOSE_POINTS;
    }
    return WIN_POINTS;
}

fn get_throw_points(throw: &RockPaperScissors) -> i32 {
    return match throw {
        RockPaperScissors::Rock => 1,
        RockPaperScissors::Paper => 2,
        RockPaperScissors::Scissors => 3,
    };
}

/// Get the score for the puzzle
/// ```
/// let vec1: Vec<String> = vec!["A Y", "B X", "C Z"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day02::puzzle_a(&vec1), 15);
/// ```
pub fn puzzle_a(raw_matches: &Vec<String>) -> i32 {
    let matches = parse_input_puzzle_1(raw_matches);
    return matches
        .iter()
        .map(|(a, b)| get_match_point(a, b) + get_throw_points(b))
        .sum();
}

fn match_result_to_rps(opponent: &RockPaperScissors, you: &str) -> RockPaperScissors {
    if you == "Y" {
        return *opponent;
    } else if you == "X" {
        // Lose
        return match opponent {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper,
        };
    } else if you == "Z" {
        // Win
        return match opponent {
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock,
        };
    }
    // No error handling only rock.
    return RockPaperScissors::Rock;
}

fn parse_line_puzzle_2(s: &String) -> (RockPaperScissors, RockPaperScissors) {
    let mut split_value = s.split_whitespace();
    let first = abc_to_rps(split_value.next().unwrap_or(""));
    let second = match_result_to_rps(&first, split_value.next().unwrap_or(""));
    return (first, second);
}

pub fn parse_input_puzzle_2(input: &Vec<String>) -> Vec<(RockPaperScissors, RockPaperScissors)> {
    return input.iter().map(|s| parse_line_puzzle_2(s)).collect();
}

/// Get the score for the puzzle
/// ```
/// let vec1: Vec<String> = vec!["A Y", "B X", "C Z"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day02::puzzle_b(&vec1), 12);
/// ```
pub fn puzzle_b(raw_matches: &Vec<String>) -> i32 {
    let matches = parse_input_puzzle_2(raw_matches);
    return matches
        .iter()
        .map(|(a, b)| get_match_point(a, b) + get_throw_points(b))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_1() {
        let vec1: Vec<String> = vec!["A Y", "B X", "C Z"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let vec2 = vec![
            (RockPaperScissors::Rock, RockPaperScissors::Paper),
            (RockPaperScissors::Paper, RockPaperScissors::Rock),
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors),
        ];
        assert_eq!(parse_input_puzzle_1(&vec1), vec2);
    }

    #[test]
    fn test_parse_input_2() {
        let vec1: Vec<String> = vec!["A Y", "B X", "C Z"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let vec2 = vec![
            (RockPaperScissors::Rock, RockPaperScissors::Rock),
            (RockPaperScissors::Paper, RockPaperScissors::Rock),
            (RockPaperScissors::Scissors, RockPaperScissors::Rock),
        ];
        assert_eq!(parse_input_puzzle_2(&vec1), vec2);
    }
}
