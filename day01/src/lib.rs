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
    return get_highest_sum_and_index_of_list(&calorie_lists).0;
}

fn get_highest_sum_and_index_of_list(calorie_lists: &Vec<Vec<i32>>) -> (i32, usize) {
    let mut highest_total = -1;
    let mut highest_index: usize = 0;

    for (index, calorie_list) in calorie_lists.iter().enumerate() {
        let current_total = calorie_list.iter().sum();
        if current_total > highest_total {
            highest_total = current_total;
            highest_index = index;
        }
    }
    return (highest_total, highest_index);
}

fn recursive_highest(calorie_lists: &Vec<Vec<i32>>, n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return get_highest_sum_and_index_of_list(&calorie_lists).0;
    }
    let (highest_sum, highest_index) = get_highest_sum_and_index_of_list(&calorie_lists);
    let mut next_run = calorie_lists.clone();
    next_run.remove(highest_index);
    return highest_sum + recursive_highest(&next_run, n - 1);
}

/// Get the top 3 highest sum of a group of vectors
/// ```
/// let vec1 = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000,8000,9000], vec![10000]];
/// assert_eq!(day01::puzzle_b(&vec1), 45000);
/// ```
/// This has to be in here, due to how rust doctests work...
pub fn puzzle_b(calorie_lists: &Vec<Vec<i32>>) -> i32 {
    return recursive_highest(&calorie_lists, 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_highest_sum_and_index_of_list() {
        let vec1 = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let expected = (24000, 3);
        assert_eq!(get_highest_sum_and_index_of_list(&vec1), expected);
    }

    #[test]
    fn test_recursive_highest() {
        let vec1 = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let expected = 35000;
        assert_eq!(recursive_highest(&vec1, 2), expected);
    }

    #[test]
    fn test_recursive_highest_0_case() {
        let vec1 = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let expected = 0;
        assert_eq!(recursive_highest(&vec1, 0), expected);
        assert_eq!(recursive_highest(&vec1, -1), expected);
    }

    #[test]
    fn test_recursive_highest_1_case() {
        let vec1 = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let expected = 24000;
        assert_eq!(recursive_highest(&vec1, 1), expected);
    }
}
