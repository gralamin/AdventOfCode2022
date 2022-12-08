extern crate filelib;

pub use filelib::load_no_blanks;

fn char_to_u8(c: char) -> u8 {
    return match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("failed to parse"),
    };
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<u8>> {
    return input
        .iter()
        .map(|line| line.chars().map(|n| char_to_u8(n)).collect())
        .collect();
}

fn get_left_index(x: usize, _y: usize) -> Option<usize> {
    if x <= 0 {
        return None;
    }
    return Some(x - 1);
}

fn get_up_index(_x: usize, y: usize) -> Option<usize> {
    if y <= 0 {
        return None;
    }
    return Some(y - 1);
}

fn get_right_index(x: usize, _y: usize, width: usize) -> Option<usize> {
    if x >= width - 1 {
        return None;
    }
    return Some(x + 1);
}

fn get_down_index(_x: usize, y: usize, height: usize) -> Option<usize> {
    if y >= height - 1 {
        return None;
    }
    return Some(y + 1);
}

fn is_visible(x: usize, y: usize, map: &Vec<Vec<u8>>) -> bool {
    let height = map.len();
    let width = map[0].len();
    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        return true;
    }
    let value = map[y][x];

    // Check left
    let mut last_x = x;
    let mut last_y = y;
    let mut visible_left = true;
    let mut index: Option<usize>;
    loop {
        index = get_left_index(last_x, last_y);
        match index {
            Some(i) => last_x = i,
            None => break,
        };
        if map[last_y][last_x] >= value {
            visible_left = false;
            break;
        }
    }
    if visible_left {
        return true;
    }

    // Check up
    last_x = x;
    last_y = y;
    let mut visible_up = true;
    loop {
        index = get_up_index(last_x, last_y);
        match index {
            Some(i) => last_y = i,
            None => break,
        };
        if map[last_y][last_x] >= value {
            visible_up = false;
            break;
        }
    }
    if visible_up {
        return true;
    }

    // Check right
    last_x = x;
    last_y = y;
    let mut visible_right = true;
    loop {
        index = get_right_index(last_x, last_y, width);
        match index {
            Some(i) => last_x = i,
            None => break,
        };
        if map[last_y][last_x] >= value {
            visible_right = false;
            break;
        }
    }
    if visible_right {
        return true;
    }

    // Check down
    last_x = x;
    last_y = y;
    let mut visible_down = true;
    loop {
        index = get_down_index(last_x, last_y, height);
        match index {
            Some(i) => last_y = i,
            None => break,
        };
        if map[last_y][last_x] >= value {
            visible_down = false;
            break;
        }
    }
    return visible_down;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["30373", "25512", "65332", "33549", "35390"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_a(&vec1), 21);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let map = parse_input(input);
    let height = map.len();
    let width = map[0].len();
    let mut num = 0;
    for y in 0..height {
        for x in 0..width {
            if is_visible(x, y, &map) {
                num += 1;
            }
        }
    }
    return num;
}

fn get_score(x: usize, y: usize, map: &Vec<Vec<u8>>) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut score = 1;
    // Anything on the edge will be 0
    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        return 0;
    }
    let value = map[y][x];

    // Check left
    let mut last_x = x;
    let mut last_y = y;
    let mut index: Option<usize>;
    let mut num = 0;
    loop {
        index = get_left_index(last_x, last_y);
        match index {
            Some(i) => last_x = i,
            None => break,
        };
        num += 1;
        if map[last_y][last_x] >= value {
            break;
        }
    }
    score *= num;

    // Check up
    last_x = x;
    last_y = y;
    num = 0;
    loop {
        index = get_up_index(last_x, last_y);
        match index {
            Some(i) => last_y = i,
            None => break,
        };
        num += 1;
        if map[last_y][last_x] >= value {
            break;
        }
    }
    score *= num;

    // Check right
    last_x = x;
    last_y = y;
    num = 0;
    loop {
        index = get_right_index(last_x, last_y, width);
        match index {
            Some(i) => last_x = i,
            None => break,
        };
        num += 1;
        if map[last_y][last_x] >= value {
            break;
        }
    }
    score *= num;

    // Check down
    last_x = x;
    last_y = y;
    num = 0;
    loop {
        index = get_down_index(last_x, last_y, height);
        match index {
            Some(i) => last_y = i,
            None => break,
        };
        num += 1;
        if map[last_y][last_x] >= value {
            break;
        }
    }
    return score * num;
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["30373", "25512", "65332", "33549", "35390"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_b(&vec1), 8);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let map = parse_input(input);
    let height = map.len();
    let width = map[0].len();
    let mut scores: Vec<usize> = vec![];
    for y in 0..height {
        for x in 0..width {
            scores.push(get_score(x, y, &map));
        }
    }
    return *scores.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input: Vec<String> = vec!["30373", "25512", "65332", "33549", "35390"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected: Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(parse_input(&input), expected);
    }
}
