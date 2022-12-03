extern crate filelib;

pub use filelib::load_no_blanks;

use std::collections::HashSet;

fn to_set(s: &String) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }
    return set;
}

fn to_set_from_str(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }
    return set;
}

fn split_rutsack(rutsack: &String) -> (HashSet<char>, HashSet<char>) {
    let half_index = rutsack.len() / 2;
    let first = &rutsack[..half_index];
    let second = &rutsack[half_index..];
    let compartment_a = to_set_from_str(first);
    let compartment_b = to_set_from_str(second);
    return (compartment_a, compartment_b);
}

fn find_common_item(compartment_a: &HashSet<char>, compartment_b: &HashSet<char>) -> char {
    let mut intersection = compartment_a.intersection(compartment_b);
    return *(intersection.next().unwrap());
}

fn get_char_point(c: char) -> i32 {
    let i = c as u32;
    if i >= 97 {
        // 1 through 26 lower case
        // little A starts at 97
        return i32::try_from(i - 96).unwrap_or(-999);
    }
    // Capital A starts at 65
    // 27 through 52 upper case
    return i32::try_from(i - 64 + 26).unwrap_or(-999);
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
/// "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_a(&vec1), 157);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> i32 {
    return input
        .iter()
        .map(|rutsack| {
            let (a, b) = split_rutsack(rutsack);
            get_char_point(find_common_item(&a, &b))
        })
        .sum();
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
/// "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_b(&vec1), 70);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> i32 {
    let chunks = input.chunks(3);
    return chunks
        .map(|chunk| get_char_point(process_chunk(chunk)))
        .sum();
}

fn process_chunk(chunk: &[String]) -> char {
    let mut first: HashSet<char> = to_set(&chunk[0]);
    for new_item in chunk.iter().skip(1) {
        let next: HashSet<char> = to_set(new_item);
        first.retain(|&k| next.contains(&k));
    }
    return *(first.iter().next().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_rutsack() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let (one, two) = split_rutsack(&s);
        // vJrwpWtwJgWr remove duplicates = 8 letters
        assert_eq!(one.len(), 8);
        // hcsFMMfFFhFp remove duplicates = 7 letters
        assert_eq!(two.len(), 7);
    }

    #[test]
    fn test_find_common_item() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let (one, two) = split_rutsack(&s);
        let c = find_common_item(&one, &two);
        assert_eq!(c, 'p');
    }

    #[test]
    fn test_get_char_point() {
        assert_eq!(get_char_point('a'), 1);
        assert_eq!(get_char_point('j'), 10);
        assert_eq!(get_char_point('z'), 26);
        assert_eq!(get_char_point('A'), 27);
        assert_eq!(get_char_point('O'), 41);
        assert_eq!(get_char_point('Z'), 52);
    }

    #[test]
    fn test_process_chunk() {
        let s = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];
        let c = s.chunks(3);
        for chunk in c {
            let new_char = process_chunk(chunk);
            assert_eq!(new_char, 'r');
        }
    }
}
