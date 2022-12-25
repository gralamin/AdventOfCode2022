extern crate filelib;

pub use filelib::load_no_blanks;

// Set a type alias in case I run out of space
type ISnafu = i128;

// Snafu is: Powers of 5, right to left, with some special symbols
// digits however, are 2, 1, 0, minus, double minus (=). So 8 is two 5s, then minus two, so 2=.

fn from_snafu(s: &Vec<String>) -> Vec<ISnafu> {
    let mut ans = vec![];
    let base: ISnafu = 5;
    for line in s {
        let highest_power = line.len() - 1;
        let mut cur_number: ISnafu = 0;
        // 2=
        // 5^1 * 2 + double minus * -2
        for (x, c) in line.chars().enumerate() {
            let digit = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Bad input"),
            };
            let power: ISnafu = base.pow((highest_power - x) as u32);
            cur_number += digit * power;
        }
        ans.push(cur_number);
    }
    return ans;
}

fn to_snafu(u: ISnafu) -> String {
    let mut n = "".to_string();

    // to get a digit:
    // (number + 2) % 5 then subtract 2 from result.
    let mut cur_number = u;
    while cur_number > 0 {
        let i = (cur_number + 2) % 5 - 2;
        let s = match i {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => panic!("Digit to snafu got an unexpected digit"),
        };
        n.push(s);
        cur_number = (cur_number - i) / 5;
    }

    return n.chars().rev().collect();
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["1=-0-2","12111","2=0=","21","2=01","111","20012","112","1=-1=","1-12","12","1=","122"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day25::puzzle_a(&vec1), "2=-1=0");
/// ```
pub fn puzzle_a(input: &Vec<String>) -> String {
    let parsed = from_snafu(input);
    let sum = parsed.iter().sum();
    return to_snafu(sum);
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["A Y", "B X", "C Z"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day25::puzzle_b(&vec1), 2);
/// ```
pub fn puzzle_b(_input: &Vec<String>) -> ISnafu {
    return 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_snafu() {
        let input = vec!["2=-01".to_string()];
        assert_eq!(from_snafu(&input), vec![976]);
    }

    #[test]
    fn test_to_snafu() {
        let input = 976;
        assert_eq!(to_snafu(input), "2=-01");
    }
}
