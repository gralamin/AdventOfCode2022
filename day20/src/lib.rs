extern crate filelib;

pub use filelib::load_no_blanks;
use std::collections::VecDeque;

pub fn parse_ints(lines: &Vec<String>) -> Vec<i64> {
    return lines.iter().map(|l| l.parse::<i64>().unwrap()).collect();
}

pub fn encrypt_values(input: Vec<i64>, encryption_key: i64) -> Vec<i64> {
    return input.iter().map(|v| v * encryption_key).collect();
}

pub fn mix_numbers(input: Vec<i64>, original_order: &Vec<i64>) -> VecDeque<i64> {
    let mut queue = VecDeque::new();

    // Set up initial values
    for i in input {
        queue.push_back(i);
    }

    let values: Vec<i64> = original_order.iter().map(|&x| x).collect();

    // Now read through original array, and move from those
    for i in values {
        let cur_pos: usize = queue.iter().position(|&j| j == i).unwrap();
        // Rotate the queue, so the current element is at 0, this makes reasoning about it much easier.
        queue.rotate_left(cur_pos);
        // Remove the item
        queue.pop_front();

        let pos: i64 = i.rem_euclid(queue.len().try_into().unwrap());
        queue.rotate_left(pos.try_into().unwrap());
        
        queue.push_front(i);
        //println!("value is now: {:?}", queue);
    }

    return queue;
}

fn get_coords(queue: &VecDeque<i64>) -> Vec<i64> {
    // Now results, find 0:
    let zero_pos = queue.iter().position(|&j| j == 0).unwrap();
    // results are at 1000, 2000, and 3000 after 0;
    return vec![1000, 2000, 3000]
        .iter()
        .map(|&off| queue[(zero_pos + off) % queue.len()])
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
    return get_coords(&mix_numbers(n.clone(), &n)).iter().sum();
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
    let mut cur_input = values.clone();
    let mut mixed = VecDeque::new();
    for _ in 0..10 {
        mixed = mix_numbers(cur_input, &values);
        cur_input = mixed.iter().map(|&x| x).collect();
    }
    return get_coords(&mixed).iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_numbers() {
        let start = vec![1, 2, -3, 3, -2, 0, 4];
        let result = mix_numbers(start.clone(), &start);
        // Note, this is the same answer as the example, but rotated
        assert_eq!(result, vec![4, 0, 3, -2, 1, 2, -3]);
    }

    #[test]
    fn test_get_coords() {
        let vec = vec![1, 2, -3, 4, 0, 3, -2];
        let mut deque = VecDeque::new();
        for i in vec {
            deque.push_back(i);
        }
        let result = get_coords(&deque);
        assert_eq!(result, vec![4, -3, 2]);
    }

    #[test]
    fn test_deque_rotations() {
        // figuring out how this works, since I seem to be wrong
        // for reasons I can't figure out. I'm used to python deques.
        let mut start = VecDeque::from(vec![1, 2, 3, 4]);
        let pos = start.iter().position(|&n| n == 4).unwrap();
        assert_eq!(pos, 3);
        start.rotate_left(3);
        assert_eq!(start, VecDeque::from(vec![4, 1, 2, 3]));
        start.pop_front();
        assert_eq!(start, VecDeque::from(vec![1, 2, 3]));
        // Can't rotate_left > len(), so need to find an equivalent
        let rem = 4usize.rem_euclid(start.len());
        start.rotate_left(rem);
        assert_eq!(start, VecDeque::from(vec![2, 3, 1]));
        start.push_front(4);
        assert_eq!(start, VecDeque::from(vec![4, 2, 3, 1]));
    }

    #[test]
    fn test_deque_rotations_negative() {
        // figuring out how this works, since I seem to be wrong
        // for reasons I can't figure out. I'm used to python deques.
        let mut start = VecDeque::from(vec![-2, 2, 3, 4]);
        let pos = start.iter().position(|&n| n == -2).unwrap();
        assert_eq!(pos, 0);
        start.rotate_left(0);
        assert_eq!(start, VecDeque::from(vec![-2, 2, 3, 4]));
        start.pop_front();
        assert_eq!(start, VecDeque::from(vec![2, 3, 4]));
        // Can't rotate_left > len(), so need to find an equivalent
        let rem: i32 = (-2i32).rem_euclid(start.len().try_into().unwrap());
        assert_eq!(rem, 1);
        start.rotate_left(rem.try_into().unwrap());
        assert_eq!(start, VecDeque::from(vec![3, 4, 2]));
        start.push_front(-2);
        assert_eq!(start, VecDeque::from(vec![-2, 3, 4, 2]));
    }

}
