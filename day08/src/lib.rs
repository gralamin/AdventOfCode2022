extern crate filelib;
extern crate gridlib;

pub use filelib::load_no_blanks;

use crate::gridlib::GridTraversable;

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

fn parse_input(input: &Vec<String>) -> gridlib::Grid<u8> {
    let values: Vec<u8> = input
        .iter()
        .map(|line| parse_line(line))
        .flatten()
        .collect();
    return gridlib::Grid::new(input[0].len(), input.len(), values);
}

fn parse_line(line: &str) -> Vec<u8> {
    return line.chars().map(|n| char_to_u8(n)).collect();
}

fn is_visible(coord: gridlib::GridCoordinate, map: &gridlib::Grid<u8>) -> bool {
    let height = map.get_height();
    let width = map.get_width();
    if coord.x == 0 || coord.y == 0 || coord.x == width - 1 || coord.y == height - 1 {
        return true;
    }
    let value = map.get_value(coord).unwrap();

    // Check direction
    let mut last: gridlib::GridCoordinate;
    let mut visible;
    let mut index: Option<gridlib::GridCoordinate>;
    for dir in vec![
        gridlib::Direction::NORTH,
        gridlib::Direction::EAST,
        gridlib::Direction::SOUTH,
        gridlib::Direction::WEST,
    ] {
        last = coord;
        visible = true;
        loop {
            index = map.get_coordinate_by_direction(last, dir);
            match index {
                Some(i) => last = i,
                None => break,
            };
            let cur_value = map.get_value(last).unwrap();
            if cur_value >= value {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }
    }
    return false;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["30373", "25512", "65332", "33549", "35390"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_a(&vec1), 21);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let map = parse_input(input);
    return map.coord_iter().filter(|&c| is_visible(c, &map)).count();
}

fn get_score(coord: gridlib::GridCoordinate, map: &gridlib::Grid<u8>) -> usize {
    let height = map.get_height();
    let width = map.get_width();
    let mut score = 1;
    if coord.x == 0 || coord.y == 0 || coord.x == width - 1 || coord.y == height - 1 {
        return 0;
    }
    let value = map.get_value(coord).unwrap();

    // Check direction
    let mut last: gridlib::GridCoordinate;
    let mut num;
    let mut index: Option<gridlib::GridCoordinate>;
    for dir in vec![
        gridlib::Direction::NORTH,
        gridlib::Direction::EAST,
        gridlib::Direction::SOUTH,
        gridlib::Direction::WEST,
    ] {
        last = coord;
        num = 0;
        loop {
            index = map.get_coordinate_by_direction(last, dir);
            match index {
                Some(i) => last = i,
                None => break,
            };
            num += 1;
            if map.get_value(last).unwrap() >= value {
                break;
            }
        }
        score *= num;
    }
    return score;
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["30373", "25512", "65332", "33549", "35390"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_b(&vec1), 8);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let map = parse_input(input);
    return map.coord_iter().map(|c| get_score(c, &map)).max().unwrap();
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
        let expected: gridlib::Grid<u8> = gridlib::Grid::new(
            5,
            5,
            vec![
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ],
        );
        assert_eq!(parse_input(&input).data_copy(), expected.data_copy());
    }
}
