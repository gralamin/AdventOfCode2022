extern crate filelib;

pub use filelib::load_no_blanks;
use std::collections::VecDeque;

pub fn parse_ints(lines: &Vec<String>) -> Vec<i64> {
    return lines.iter().map(|l| l.parse::<i64>().unwrap()).collect();
}

pub fn encrypt_values(input: Vec<i64>, encryption_key: i64) -> Vec<i64> {
    return input.iter().map(|v| v * encryption_key).collect();
}

pub fn mix_numbers(input: Vec<i64>, iters: usize) -> Vec<i64> {
    let mut queue: VecDeque<(usize, &i64)> = input.iter().enumerate().collect();

    // Now read through original array, and move from those
    for _ in 0..iters {
        for i in 0..queue.len() {
            // The key thing here is there are DUPLICATES.
            // if we just go by the queue values, we will end up hitting the first
            // value, instead of the proper indexed number.
            let idx = queue
                .iter()
                .position(|(j, _)| i == *j)
                .unwrap();
            queue.rotate_left(idx);
            let (j, v) = queue.pop_front().unwrap();
            let d = v.rem_euclid(queue.len() as i64) as usize;
            queue.rotate_left(d);
            queue.push_front((j, v));

            //println!("value is now: {:?}", queue)
        }
    }

    return queue.iter().map(|(_, j)| **j).collect();
}

fn get_coords(queue: Vec<i64>) -> Vec<i64> {
    let zero_pos = queue
        .iter()
        .position(|j| j == &0)
        .unwrap();
    return vec![1000, 2000, 3000]
        .iter()
        .map(|index| queue[(zero_pos + index) % queue.len()])
        .collect();
}


/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["1","2","-3","3","-2","0","4"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day20::puzzle_a(&vec1), 3);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> i64 {
    // 3466 should be result for my input
    let n = parse_ints(input);
    let mixed = mix_numbers(n, 1);
    let coords = get_coords(mixed);
    return coords.iter().sum();
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["1","2","-3","3","-2","0","4"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day20::puzzle_b(&vec1), 1623178306);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> i64 {
    // 9995532008348 should be result for my input
    let key = 811589153;
    let n = parse_ints(input);
    let values = encrypt_values(n, key);
    let mixed = mix_numbers(values, 10);
    let coords = get_coords(mixed);
    return coords.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_numbers() {
        let start = vec![1, 2, -3, 3, -2, 0, 4];
        let result = mix_numbers(start, 1);
        // Note, this is the same answer as the example, but rotated
        assert_eq!(result, vec![4, 0, 3, -2, 1, 2, -3]);
    }

    #[test]
    fn test_mix_numbers_duplicated() {
        let start = vec![1, 2, -3, 3, -2, 0, 4, 4, 4];
        let result = mix_numbers(start, 1);
        // Note, this is the same answer as the example, but rotated
        assert_eq!(result, vec![4, 2, 0, 3, -3, 4, -2, 1, 4]);
    }

    #[test]
    fn test_get_coords() {
        let vec: Vec<i64> = vec![1, 2, -3, 4, 0, 3, -2];
        let result = get_coords(vec);
        assert_eq!(result, vec![4, -3, 2]);
    }

}
