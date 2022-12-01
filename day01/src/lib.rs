extern crate filelib;

pub use filelib::load;
pub use filelib::split_lines_by_blanks;

/// Get the highest sum of a group of vectors
/// ```
/// let vec1 = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000,8000,9000], vec![10000]];
/// assert_eq!(day01::puzzle_a(&vec1), 24000);
/// ```
/// This has to be in here, due to how rust doctests work...
pub fn puzzle_a(calorie_lists: &Vec<Vec<i32>>) -> i32 {
    let mut sums = to_sums(calorie_lists);
    sums.sort();
    return sums.pop().unwrap();
}

fn to_sums(calorie_list: &Vec<Vec<i32>>) -> Vec<i32> {
    return calorie_list.iter().map(|l| l.iter().sum()).collect();
}

/// Get the top 3 highest sum of a group of vectors
/// ```
/// let vec1 = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000,8000,9000], vec![10000]];
/// assert_eq!(day01::puzzle_b(&vec1), 45000);
/// ```
/// This has to be in here, due to how rust doctests work...
pub fn puzzle_b(calorie_lists: &Vec<Vec<i32>>) -> i32 {
    let mut sums = to_sums(calorie_lists);
    sums.sort();
    return sums.pop().unwrap() + sums.pop().unwrap() + sums.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_sums() {
        let vec1 = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let expected = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(to_sums(&vec1), expected);
    }
}
