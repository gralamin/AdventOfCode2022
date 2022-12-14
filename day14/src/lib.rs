extern crate filelib;

pub use filelib::load_no_blanks;
use filelib::parse_path_to_coords;

fn parse_input(input: &Vec<String>) -> Vec<Vec<(i32, i32)>> {
    return input
        .iter()
        .map(|line| parse_path_to_coords(line))
        .collect();
}

fn move_sand(paths: &Vec<Vec<(i32, i32)>>, sand_x: i32, sand_y: i32) -> (i32, i32, bool) {
    // preference, check if we can fall down
    if try_move_down(paths, sand_x, sand_y) {
        return (sand_x, sand_y + 1, false);
    }

    for (x, y) in vec![(sand_x - 1, sand_y + 1), (sand_x + 1, sand_y + 1)] {
        if try_diagonal_point(paths, x, y) {
            return (x, y, false);
        }
    }
    return (sand_x, sand_y, true);
}

fn try_move_down(paths: &Vec<Vec<(i32, i32)>>, sand_x: i32, sand_y: i32) -> bool {
    // since paths are either horizontal or vertical lines, we need to check for either:
    // - Horizontal line covering the new x, y
    // - a vertical line starting / ending at the new x,y
    let candidate_x = sand_x;
    let candidate_y = sand_y + 1;

    for path in paths {
        let (mut last_x, mut last_y) = path[0];
        for i in 1..path.len() {
            let (new_x, new_y) = path[i];
            if last_y == new_y {
                // horizontal line
                if candidate_y == new_y && between_straight_line(last_x, new_x, candidate_x) {
                    return false;
                }
            } else if last_x == new_x {
                // vertical line
                if candidate_x == last_x && (candidate_y == new_y || candidate_y == last_y) {
                    return false;
                }
            }

            last_x = new_x;
            last_y = new_y;
        }
    }
    // No collisions
    return true;
}

fn between_straight_line(start: i32, end: i32, point: i32) -> bool {
    return (start <= point && end >= point) || (end <= point && start >= point);
}

fn try_diagonal_point(paths: &Vec<Vec<(i32, i32)>>, sand_x: i32, sand_y: i32) -> bool {
    // So its possible we could now be in a vertical line
    // or in a horizontal line
    // so we need to use the solution for try_move_down, but without the vertical_line special case
    for path in paths {
        let (mut last_x, mut last_y) = path[0];
        for i in 1..path.len() {
            let (new_x, new_y) = path[i];
            if last_y == new_y {
                // horizontal line
                if sand_y == new_y && between_straight_line(last_x, new_x, sand_x) {
                    return false;
                }
            } else if last_x == new_x {
                // vertical line
                if sand_x == last_x && between_straight_line(last_y, new_y, sand_y) {
                    return false;
                }
            }

            last_x = new_x;
            last_y = new_y;
        }
    }
    // No collisions
    return true;
}

fn below_all_paths(paths: &Vec<Vec<(i32, i32)>>) -> i32 {
    // Get the y such that we can assume that we are past all the points
    return paths
        .iter()
        .map(|path| path.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap()
        + 1;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["498,4 -> 498,6 -> 496,6", "503,4 -> 502,4 -> 502,9 -> 494,9"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day14::puzzle_a(&vec1), 24);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let mut paths = parse_input(input);
    let sand_start_x = 500;
    let sand_start_y = 0;
    let mut num_sand: usize = 1;
    let abyss_y = below_all_paths(&paths);

    let mut current_x = sand_start_x;
    let mut current_y = sand_start_y;
    let mut at_rest = false;

    loop {
        //println!("Sand is at: ({}, {})", current_x, current_y);
        if current_y >= abyss_y {
            // throw away this sand, since it hit the abyss.
            num_sand -= 1;
            break;
        }
        if at_rest {
            num_sand += 1;
            paths.push(vec![(current_x, current_y), (current_x, current_y)]);
            current_x = sand_start_x;
            current_y = sand_start_y;
            at_rest = false;
        } else {
            (current_x, current_y, at_rest) = move_sand(&paths, current_x, current_y);
        }
    }

    return num_sand;
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["498,4 -> 498,6 -> 496,6", "503,4 -> 502,4 -> 502,9 -> 494,9"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day14::puzzle_b(&vec1), 93);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let mut paths = parse_input(input);
    let sand_start_x = 500;
    // Start at negative -1.
    let sand_start_y = -1;
    let mut num_sand: usize = 1;
    let floor_y = below_all_paths(&paths) + 1;
    paths.push(vec![(i32::MIN, floor_y), (i32::MAX, floor_y)]);

    let mut current_x = sand_start_x;
    let mut current_y = sand_start_y;
    let mut at_rest = false;

    loop {
        //println!("Sand is at: ({}, {})", current_x, current_y);
        if at_rest {
            num_sand += 1;
            paths.push(vec![(current_x, current_y), (current_x, current_y)]);
            if current_x == sand_start_x && current_y == 0 {
                num_sand -= 1;
                break;
            }

            current_x = sand_start_x;
            current_y = sand_start_y;
            at_rest = false;
        } else {
            (current_x, current_y, at_rest) = move_sand(&paths, current_x, current_y);
        }
    }

    return num_sand;
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(1, 1);
    }
}
*/
