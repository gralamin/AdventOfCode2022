extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::{Direction, GridCoordinateInf};
use rustc_hash::{FxHashMap, FxHashSet};

type Coord = GridCoordinateInf;
type SparseSet = FxHashSet<Coord>;

fn check_directions(coord: Coord, directions: &Vec<Direction>, elf_loc: &SparseSet) -> bool {
    for direction in directions {
        let next_coord = coord.move_dir(*direction);
        if elf_loc.contains(&next_coord) {
            return false;
        }
    }
    return true;
}

fn run_simulation(elf_loc: &SparseSet, round_num_start: usize, round_num_end: usize) -> SparseSet {
    let mut cur_map = elf_loc.clone();
    let all_directions = vec![
        Direction::NORTH,
        Direction::EAST,
        Direction::SOUTH,
        Direction::WEST,
        Direction::NORTHEAST,
        Direction::SOUTHEAST,
        Direction::SOUTHWEST,
        Direction::NORTHWEST,
    ];

    // elves = #
    // empty ground = .
    // nothing beside self = do nothing
    // if N, NE, and NW open, elf moves north
    // if S, SE, and SW open, elf moves south
    // if W, NW, and SW open, elf moves west
    // if E, NE, and SE open, elf moves east.
    // if none above, no move
    // After proposing the move, remove all elves trying to go to the same tile
    // moves proposoal are offset by round number (1st N first, 2nd S first, 3, W first)
    // round 10: Count numer of empty ground tiles in the bounding box of elves
    // note grid can grow over time
    // this should prevent the chance of an elf tries to move into a spot that an elf was.
    let directions_to_check = vec![
        vec![Direction::NORTH, Direction::NORTHEAST, Direction::NORTHWEST],
        vec![Direction::SOUTH, Direction::SOUTHEAST, Direction::SOUTHWEST],
        vec![Direction::WEST, Direction::NORTHWEST, Direction::SOUTHWEST],
        vec![Direction::EAST, Direction::NORTHEAST, Direction::SOUTHEAST],
    ];

    for num in round_num_start..round_num_end {
        //println!("cur_map len is: {}", cur_map.len());
        let mut next_map = SparseSet::default();
        let mut possible_moves = FxHashMap::default();
        for elf in cur_map.clone() {
            // Check if we should move at all
            if check_directions(elf, &all_directions, &cur_map) {
                // no one adjacent, don't move
                next_map.insert(elf);
                continue;
            }
            let mut no_move = true;
            for i in 0..directions_to_check.len() {
                let direction_to_check =
                    directions_to_check[(i + num) % directions_to_check.len()].clone();
                if check_directions(elf, &direction_to_check, &cur_map) {
                    let next_coord = elf.move_dir(direction_to_check[0]);
                    let mut coord_vec = possible_moves.entry(next_coord).or_insert(vec![]);
                    coord_vec.push(elf);
                    no_move = false;
                    break;
                }
            }
            if no_move {
                next_map.insert(elf);
            }
        }

        for (key, value) in possible_moves {
            if value.len() == 1 {
                next_map.insert(key);
            } else {
                for v in value {
                    next_map.insert(v);
                }
            }
        }
        cur_map = next_map;
    }

    return cur_map;
}

fn calc_empty(elf_loc: &SparseSet) -> usize {
    let num_elves = elf_loc.len();
    let min_x = elf_loc.iter().map(|c| c.x).min().unwrap();
    let min_y = elf_loc.iter().map(|c| c.y).min().unwrap();

    let max_x = elf_loc.iter().map(|c| c.x).max().unwrap();
    let max_y = elf_loc.iter().map(|c| c.y).max().unwrap();
    let y_len: usize = (max_y - min_y + 1).try_into().unwrap();
    let x_len: usize = (max_x - min_x + 1).try_into().unwrap();
    return (y_len * x_len) - num_elves;
}

fn parse_input(lines: &Vec<String>) -> SparseSet {
    let mut set = SparseSet::default();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    set.insert(Coord::new(x as i32, y as i32));
                    ()
                }
                _ => panic!("Bad input"),
            };
        }
    }
    return set;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["....#..","..###.#","#...#.#",".#...##","#.###..","##.#.##",".#..#.."].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day23::puzzle_a(&vec1), 110);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let i = parse_input(input);
    let after_sim = run_simulation(&i, 0, 10);
    return calc_empty(&after_sim);
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["....#..","..###.#","#...#.#",".#...##","#.###..","##.#.##",".#..#.."].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day23::puzzle_b(&vec1), 20);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let i = parse_input(input);
    let mut last_sim = i.clone();
    let mut round = 1;
    loop {
        let after_sim = run_simulation(&last_sim, round - 1, round);
        //println!("New pos: {:?}", after_sim);
        if last_sim == after_sim {
            return round;
        }
        last_sim = after_sim.clone();
        round += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_empty() {
        let mut map = SparseSet::default();
        map.insert(Coord::new(-3, -1));
        map.insert(Coord::new(-1, -3));
        // 9 - 2 = 7
        assert_eq!(calc_empty(&map), 7);
    }
}
