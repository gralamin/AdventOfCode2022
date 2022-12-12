extern crate filelib;

pub use filelib::load_no_blanks;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use gridlib::Grid;
use gridlib::GridCoordinate;
use gridlib::GridTraversable;

// S = a
// E = z
const LOWEST_ELEVATION: u8 = 'a' as u8;
const HIGHEST_ELEVATION: u8 = 'z' as u8;
const START_ELEVATION: u8 = LOWEST_ELEVATION;
const END_ELEVATION: u8 = HIGHEST_ELEVATION;

fn parse_input(input: &Vec<String>) -> (Grid<u8>, GridCoordinate, GridCoordinate) {
    let height = input.len();
    let width = input[0].len();
    let mut values: Vec<u8> = vec![];
    let mut start = GridCoordinate::new(0, 0);
    let mut end = GridCoordinate::new(width, height);

    for line in input {
        for c in line.chars() {
            if c != 'S' && c != 'E' {
                values.push(c as u8);
            } else {
                if c == 'S' {
                    values.push(START_ELEVATION);
                    let start_x = (values.len() - 1) % width;
                    let start_y = (values.len() - 1) / width;
                    start = GridCoordinate::new(start_x, start_y);
                } else if c == 'E' {
                    values.push(END_ELEVATION);
                    let end_x = (values.len() - 1) % width;
                    let end_y = (values.len() - 1) / width;
                    end = GridCoordinate::new(end_x, end_y);
                } else {
                    panic!("Unknown char {}", c);
                }
            }
        }
    }
    return (Grid::new(width, height, values), start, end);
}

pub fn example_map() -> Vec<String> {
    return vec!["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct QueueState {
    cost: i32,
    position: GridCoordinate,
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html
impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn pathfind_djikstra(grid: &Grid<u8>, start: GridCoordinate, end: GridCoordinate) -> usize {
    let mut dist = vec![i32::MAX; grid.get_height() * grid.get_width()];
    let mut queue = BinaryHeap::new();
    queue.push(QueueState {
        cost: 0,
        position: start,
    });
    dist[start.y * grid.get_width() + start.x] = 0;

    //println!("Determining path from {}, to {}", start, end);

    while let Some(QueueState { cost, position }) = queue.pop() {
        if position == end {
            return cost.try_into().unwrap();
        }

        if cost > dist[position.y * grid.get_width() + position.x] {
            continue;
        }

        let cur_height = grid.get_value(position).unwrap();

        for possible_pos in grid.get_adjacent_coordinates(position) {
            let possible_height = grid.get_value(possible_pos).unwrap();
            // Skip if too high
            if possible_height > cur_height + 1 {
                continue;
            }

            // Not too high, add it, if we don't have a better path here
            let next = QueueState {
                cost: cost + 1,
                position: possible_pos,
            };
            let next_index = possible_pos.y * grid.get_width() + possible_pos.x;
            if next.cost < dist[next_index] {
                //println!("Found path from {} to {}, cost: {}", position, possible_pos, next.cost);
                //println!("  - height {} vs height {}", cur_height as char, possible_height as char);
                queue.push(next);
                dist[next_index] = next.cost;
            }
        }
    }

    return dist[end.y * grid.get_width() + end.x].try_into().unwrap();
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = day12::example_map();
/// assert_eq!(day12::puzzle_a(&vec1), 31);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let (puzzle, start, end) = parse_input(input);
    return pathfind_djikstra(&puzzle, start, end);
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = day12::example_map();
/// assert_eq!(day12::puzzle_b(&vec1), 29);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let (puzzle, _, end) = parse_input(input);
    let mut starts = vec![];
    for y in 0..puzzle.get_height() {
        for x in 0..puzzle.get_width() {
            let coordinate = GridCoordinate::new(x, y);
            let v = puzzle.get_value(coordinate).unwrap();
            if v == LOWEST_ELEVATION {
                starts.push(coordinate);
            }
        }
    }
    return starts
        .iter()
        .map(|s| pathfind_djikstra(&puzzle, *s, end))
        .min()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let start = GridCoordinate::new(0, 0);
        let end = GridCoordinate::new(3, 3);
        let grid: Grid<u8> = Grid::new(4, 4, vec![1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7]);
        let r = pathfind_djikstra(&grid, start, end);
        assert_eq!(r, 6);
    }
}
