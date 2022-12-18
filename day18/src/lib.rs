extern crate filelib;

pub use filelib::load_no_blanks;
use std::collections::HashSet;
use std::collections::VecDeque;

type GridInt = i32;
type Grid3 = (GridInt, GridInt, GridInt);

fn parse_cubes(lines: &Vec<String>) -> Vec<Grid3> {
    return lines
        .iter()
        .map(|line| {
            let (first, second_str) = line.split_once(",").unwrap();
            let (second, third) = second_str.split_once(",").unwrap();
            return (
                first.parse::<GridInt>().unwrap(),
                second.parse::<GridInt>().unwrap(),
                third.parse::<GridInt>().unwrap(),
            );
        })
        .collect();
}

fn get_adjacent_coords(x: GridInt, y: GridInt, z: GridInt) -> Vec<Grid3> {
    // 6 adjacent coords
    return vec![
        (x, y, z + 1),
        (x, y, z - 1),
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
    ];
}

fn check_exposed(cubes: &Vec<Grid3>) -> usize {
    let mut exposed = 0;
    for (cube_x, cube_y, cube_z) in cubes {
        let adjacents_coords = get_adjacent_coords(*cube_x, *cube_y, *cube_z);
        let matching: Vec<&Grid3> = cubes
            .iter()
            .filter(|&f| adjacents_coords.contains(f))
            .collect();
        exposed += 6 - matching.len();
    }
    return exposed;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["2,2,2","1,2,2","3,2,2","2,1,2","2,3,2","2,2,1","2,2,3",
/// "2,2,4","2,2,6","1,2,5","3,2,5","2,1,5","2,3,5"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day18::puzzle_a(&vec1), 64);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let cubes = parse_cubes(input);
    return check_exposed(&cubes);
}

fn check_exposed_minus_air_pockets(cubes: &Vec<Grid3>) -> usize {
    let mut exposed = 0;

    // use BFS to find the outside, until we finish a bounding box
    // Add 1 point border to allow a safe start.
    let bounding_low_x: GridInt = cubes.iter().map(|c| c.0).min().unwrap() - 1;
    let bounding_high_x: GridInt = cubes.iter().map(|c| c.0).max().unwrap() + 1;
    let bounding_low_y: GridInt = cubes.iter().map(|c| c.1).min().unwrap() - 1;
    let bounding_high_y: GridInt = cubes.iter().map(|c| c.1).max().unwrap() + 1;
    let bounding_low_z: GridInt = cubes.iter().map(|c| c.2).min().unwrap() - 1;
    let bounding_high_z: GridInt = cubes.iter().map(|c| c.2).max().unwrap() + 1;

    let mut visited: HashSet<Grid3> = HashSet::new();
    let mut queue: VecDeque<Grid3> = VecDeque::new();

    let start = (bounding_low_x, bounding_low_y, bounding_low_z);
    queue.push_front(start);

    while !queue.is_empty() {
        let coords = queue.pop_front().unwrap();
        if visited.contains(&coords) {
            continue;
        }
        visited.insert(coords);

        let neighbours: Vec<Grid3> = get_adjacent_coords(coords.0, coords.1, coords.2)
            .iter()
            .filter(|(x, _, _)| *x >= bounding_low_x && *x <= bounding_high_x)
            .filter(|(_, y, _)| *y >= bounding_low_y && *y <= bounding_high_y)
            .filter(|(_, _, z)| *z >= bounding_low_z && *z <= bounding_high_z)
            .map(|(x, y, z)| (*x, *y, *z))
            .collect();
        for n in neighbours {
            if cubes.iter().any(|&f| f == n) {
                exposed += 1;
                continue;
            }
            if !visited.contains(&n) {
                queue.push_back(n);
            }
        }
    }

    return exposed;
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["2,2,2","1,2,2","3,2,2","2,1,2","2,3,2","2,2,1","2,2,3",
/// "2,2,4","2,2,6","1,2,5","3,2,5","2,1,5","2,3,5"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day18::puzzle_b(&vec1), 58);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let cubes = parse_cubes(input);
    return check_exposed_minus_air_pockets(&cubes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_example() {
        let vec1: Vec<String> = vec!["1,1,1", "2,1,1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(puzzle_a(&vec1), 10);
    }
}
