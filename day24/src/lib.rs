extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::{Direction, Grid, GridCoordinate, GridTraversable};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

// Your position, and cycle time
// cycle time = round % (height * width);
type CacheKey = (GridCoordinate, usize);
type Cache = FxHashSet<CacheKey>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum ValleyTile {
    Clear,
    Solid,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Blizzard {
    dir: Direction,
    coord: GridCoordinate,
    respawn_coord: GridCoordinate,
}

impl Blizzard {
    fn new(dir: Direction, coord: GridCoordinate, grid: &Grid<ValleyTile>) -> Blizzard {
        let reverse_dir;
        if dir == Direction::EAST {
            reverse_dir = Direction::WEST;
        } else if dir == Direction::SOUTH {
            reverse_dir = Direction::NORTH;
        } else if dir == Direction::WEST {
            reverse_dir = Direction::EAST;
        } else if dir == Direction::NORTH {
            reverse_dir = Direction::SOUTH;
        } else {
            reverse_dir = dir;
        }
        let mut last = coord;
        let respawn_coord;
        loop {
            if let Some(new) = grid.get_coordinate_by_direction(last, reverse_dir) {
                let v = grid.get_value(new).unwrap();
                match v {
                    ValleyTile::Clear => {
                        last = new;
                        continue;
                    }
                    ValleyTile::Solid => {
                        // We hit the wall!
                        respawn_coord = last;
                        break;
                    }
                };
            } else {
                // gone off the map, so just use coord, I guess.
                respawn_coord = coord;
                break;
            }
        }

        return Blizzard {
            dir: dir,
            coord: coord,
            respawn_coord: respawn_coord,
        };
    }

    fn step_clone(&self, grid: &Grid<ValleyTile>) -> Blizzard {
        let new_coord;
        if let Some(coord) = grid.get_coordinate_by_direction(self.coord, self.dir) {
            let v = grid.get_value(coord).unwrap();
            match v {
                ValleyTile::Clear => {
                    new_coord = coord;
                }
                ValleyTile::Solid => {
                    new_coord = self.respawn_coord;
                }
            };
        } else {
            new_coord = self.respawn_coord;
        }

        return Blizzard {
            dir: self.dir,
            coord: new_coord,
            respawn_coord: self.respawn_coord,
        };
    }
}

fn parse_input(input: &Vec<String>) -> (Grid<ValleyTile>, Vec<Blizzard>) {
    let width = input[0].len();
    let height = input.len();
    let mut grid_values = vec![];
    let mut blizzard_builders = vec![];

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => ValleyTile::Solid,
                '.' => ValleyTile::Clear,
                '^' => {
                    blizzard_builders.push((Direction::NORTH, GridCoordinate::new(x, y)));
                    ValleyTile::Clear
                }
                '>' => {
                    blizzard_builders.push((Direction::EAST, GridCoordinate::new(x, y)));
                    ValleyTile::Clear
                }
                '<' => {
                    blizzard_builders.push((Direction::WEST, GridCoordinate::new(x, y)));
                    ValleyTile::Clear
                }
                'v' => {
                    blizzard_builders.push((Direction::SOUTH, GridCoordinate::new(x, y)));
                    ValleyTile::Clear
                }
                _ => unreachable!(),
            };
            grid_values.push(tile);
        }
    }

    let grid = Grid::new(width, height, grid_values);
    let blizzards: Vec<Blizzard> = blizzard_builders
        .iter()
        .map(|(d, c)| Blizzard::new(*d, *c, &grid))
        .collect();

    return (grid, blizzards);
}

fn bfs_through(
    g: &Grid<ValleyTile>,
    start: GridCoordinate,
    end: GridCoordinate,
    blizzards: Vec<Blizzard>,
) -> usize {
    // First solution, imagine blizzards aren't moving
    let mut cached = Cache::default();
    let start_turn = 0;
    let blizzard_cycle_max = (g.get_width() - 1) * (g.get_height() - 1);
    let mut queue = VecDeque::default();
    queue.push_back((vec![], start, start_turn));
    let mut min_time = usize::MAX;
    // Seen stops memory from ballooning, by stoping redunant adds.
    let mut seen = Cache::default();

    let mut cached_blizzard_cycles = vec![];
    let mut last_blizzard_cycle = blizzards.clone();
    let mut first_coord_cache: FxHashSet<GridCoordinate> = FxHashSet::default();
    for cycle in last_blizzard_cycle.iter() {
        first_coord_cache.insert(cycle.coord);
    }
    cached_blizzard_cycles.push(first_coord_cache);
    for _ in 1..blizzard_cycle_max {
        let new_cycle: Vec<Blizzard> = last_blizzard_cycle
            .iter()
            .map(|b| b.step_clone(g))
            .collect();
        let mut coord_cache: FxHashSet<GridCoordinate> = FxHashSet::default();
        for cycle in new_cycle.iter() {
            coord_cache.insert(cycle.coord);
        }
        cached_blizzard_cycles.push(coord_cache);
        last_blizzard_cycle = new_cycle;
    }

    let dirs = vec![
        Direction::NORTH,
        Direction::EAST,
        Direction::SOUTH,
        Direction::WEST,
    ];

    while let Some((cur_path, cur_loc, cur_turn)) = queue.pop_front() {
        if cur_turn > min_time {
            // path that is longer then minimum we've found
            // and all further ones will be as well, by BFS
            break;
        }

        let cycle_spot = cur_turn % blizzard_cycle_max;
        if cached.contains(&(cur_loc, cycle_spot)) {
            // We have already seen this exact state
            // and if we have already been here and this is likely a later spot thats equivalent.
            continue;
        }
        let cur_blizzards = &cached_blizzard_cycles[cycle_spot];
        // if cur_loc is in cur_blizzards, skip this
        if cur_blizzards.contains(&cur_loc) {
            continue;
        }

        // If we are at the end, put us in the cache and stop computing
        if cur_loc == end {
            //println!("Path found of len {}: {:?}", cur_path.len(), cur_path);
            min_time = min_time.min(cur_turn);
            break;
        }
        cached.insert((cur_loc, cycle_spot));

        // try moving
        let new_blizzards = &cached_blizzard_cycles[(cur_turn + 1) % blizzard_cycle_max];
        let mut new_path = cur_path.clone();
        new_path.push(cur_loc);
        for possible_dir in &dirs {
            if let Some(new_coord) = g.get_coordinate_by_direction(cur_loc, *possible_dir) {
                // ensure we aren't on a wall
                //println!("Considering: {} for turn {}", new_coord, cur_turn + 1);
                if g.get_value(new_coord).unwrap() == ValleyTile::Solid {
                    //println!("Thrown out for being a wall");
                    continue;
                }
                if new_coord == start {
                    //println!("Thrown out for being the start");
                    continue;
                }
                if new_blizzards.contains(&new_coord) {
                    //println!("Thrown out for having a blizzard");
                    continue;
                }
                let seen_key = (new_coord, (cur_turn + 1) % blizzard_cycle_max);
                if seen.contains(&seen_key) {
                    continue;
                };
                seen.insert(seen_key);
                let new_state = (new_path.clone(), new_coord, cur_turn + 1);
                queue.push_back(new_state);
            }
        }
        if new_blizzards.contains(&cur_loc) {
            continue;
        }

        // try waiting:
        queue.push_back((new_path.clone(), cur_loc, cur_turn + 1));
    }

    return min_time;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["#.######","#>>.<^<#","#.<..<<#","#>v.><>#","#<^v^^>#","######.#"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day24::puzzle_a(&vec1), 18);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let (grid, blizzards) = parse_input(input);
    let start = GridCoordinate::new(1, 0);
    let end = GridCoordinate::new(grid.get_width() - 2, grid.get_height() - 1);
    return bfs_through(&grid, start, end, blizzards);
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["#.######","#>>.<^<#","#.<..<<#","#>v.><>#","#<^v^^>#","######.#"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day24::puzzle_b(&vec1), 54);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let (grid, blizzards) = parse_input(input);
    let start = GridCoordinate::new(1, 0);
    let end = GridCoordinate::new(grid.get_width() - 2, grid.get_height() - 1);
    let end_first = bfs_through(&grid, start, end, blizzards.clone());
    let blizzards_two = get_blizzards_after_n_turns(&blizzards, end_first, &grid);
    let end_second = bfs_through(&grid, end, start, blizzards_two.clone());
    let blizzards_three = get_blizzards_after_n_turns(&blizzards_two, end_second, &grid);
    let end_third = bfs_through(&grid, start, end, blizzards_three);
    return end_first + end_second + end_third;
}

fn get_blizzards_after_n_turns(
    blizzards: &Vec<Blizzard>,
    n: usize,
    grid: &Grid<ValleyTile>,
) -> Vec<Blizzard> {
    let mut last_blizzard_cycle;
    let mut new_cycle = blizzards.clone();

    for _ in 0..n {
        last_blizzard_cycle = new_cycle;
        new_cycle = last_blizzard_cycle
            .iter()
            .map(|b| b.step_clone(grid))
            .collect();
    }
    return new_cycle;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blizzard_steps() {
        let vec1: Vec<String> = vec![
            "#.#####", "#.....#", "#>....#", "#.....#", "#...v.#", "#.....#", "#####.#",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let (grid, blizzards) = parse_input(&vec1);
        let b = blizzards[1].step_clone(&grid).step_clone(&grid);
        assert_eq!(b.coord, b.respawn_coord);
        assert_eq!(b.coord, GridCoordinate::new(4, 1));
        // #>....#
        //   >   # - after 1
        //    >  # - after 2
        //     > # - after 3
        //      ># - after 4
        //  #>   # - after 5
        let c = blizzards[0]
            .step_clone(&grid)
            .step_clone(&grid)
            .step_clone(&grid)
            .step_clone(&grid)
            .step_clone(&grid);
        assert_eq!(c.coord, c.respawn_coord);
        assert_eq!(c.coord, GridCoordinate::new(1, 2));
    }
}
