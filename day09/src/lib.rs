extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::{Direction, GridCoordinateInf};
use std::collections::HashSet;

#[derive(Debug)]
struct State {
    head: GridCoordinateInf,
    tail: GridCoordinateInf,
    tail_locations: HashSet<GridCoordinateInf>,
}

impl State {
    pub fn new(location: GridCoordinateInf) -> State {
        let mut set: HashSet<GridCoordinateInf> = HashSet::new();
        set.insert(location);
        //println!("Inserting {}", location);
        return State {
            head: location,
            tail: location,
            tail_locations: set,
        };
    }

    fn move_step(&mut self, direction: Direction, times: usize) {
        for _ in 0..times {
            self.head = move_head(self.head, direction);
            self.tail = move_tail(self.head, self.tail);
            //println!("Inserting {}, Head: {}", self.tail, self.head);
            self.tail_locations.insert(self.tail);
        }
    }

    fn get_num_tail(&self) -> usize {
        return self.tail_locations.len();
    }
}

fn move_tail(head: GridCoordinateInf, tail: GridCoordinateInf) -> GridCoordinateInf {
    // cardinal directions, if two up, the tail move up one.
    let two_up = GridCoordinateInf::new(0, -2);
    let two_down = GridCoordinateInf::new(0, 2);
    let two_left = GridCoordinateInf::new(-2, 0);
    let two_right = GridCoordinateInf::new(2, 0);

    if (tail + two_up) == head {
        return tail + GridCoordinateInf::new(0, -1);
    } else if (tail + two_down) == head {
        return tail + GridCoordinateInf::new(0, 1);
    } else if (tail + two_left) == head {
        return tail + GridCoordinateInf::new(-1, 0);
    } else if (tail + two_right) == head {
        return tail + GridCoordinateInf::new(1, 0);
    }

    // check if adjacent on diagonals
    let northeast = GridCoordinateInf::new(1, -1);
    let northwest = GridCoordinateInf::new(-1, -1);
    let southeast = GridCoordinateInf::new(1, 1);
    let southwest = GridCoordinateInf::new(-1, 1);
    if (tail + northeast) == head
        || (tail + northwest) == head
        || (tail + southeast) == head
        || (tail + southwest) == head
    {
        return tail;
    }

    if tail.x == head.x || tail.y == head.y {
        return tail;
    }

    // Determine which way to move
    if tail.x < head.x {
        if tail.y < head.y {
            return tail + southeast;
        }
        return tail + northeast;
    } else {
        if tail.y < head.y {
            return tail + southwest;
        }
        return tail + northwest;
    }
}

fn move_head(head: GridCoordinateInf, direction: Direction) -> GridCoordinateInf {
    return head.move_dir(direction);
}

fn parse_input_to_directions(input: &Vec<String>) -> Vec<(Direction, usize)> {
    return input
        .iter()
        .map(|line| {
            let (letter, num_str) = line.split_once(" ").unwrap();
            let num = num_str.parse::<usize>().unwrap();
            if letter == "R" {
                return (Direction::EAST, num);
            } else if letter == "L" {
                return (Direction::WEST, num);
            } else if letter == "U" {
                return (Direction::NORTH, num);
            } else if letter == "D" {
                return (Direction::SOUTH, num);
            } else {
                panic!("Can't parse letter");
            }
        })
        .collect();
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["R 4", "U 4", "L 3", "D 1",
///   "R 4", "D 1", "L 5", "R 2"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_a(&vec1), 13);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let steps = parse_input_to_directions(input);
    let mut state = State::new(GridCoordinateInf::new(0, 0));
    for (dir, num) in steps {
        state.move_step(dir, num);
    }
    return state.get_num_tail();
}

#[derive(Debug)]
struct StateSegmented {
    head: GridCoordinateInf,
    segments: Vec<GridCoordinateInf>,
    tail_locations: HashSet<GridCoordinateInf>,
}

impl StateSegmented {
    pub fn new(location: GridCoordinateInf, num_segments: usize) -> StateSegmented {
        let mut set: HashSet<GridCoordinateInf> = HashSet::new();
        set.insert(location);
        let mut segments = vec![];
        for _ in 0..num_segments {
            segments.push(location);
        }
        //println!("Inserting {}", location);
        return StateSegmented {
            head: location,
            segments: segments,
            tail_locations: set,
        };
    }

    fn move_step(&mut self, direction: Direction, times: usize) {
        for _ in 0..times {
            self.head = move_head(self.head, direction);
            for i in 0..self.segments.len() {
                if i == 0 {
                    self.segments[i] = move_tail(self.head, self.segments[i]);
                } else {
                    self.segments[i] = move_tail(self.segments[i - 1], self.segments[i]);
                }
                //println!("Moved segment {} to {}", i, self.segments[i]);
            }
            //println!("Inserting {}, Head: {}", self.segments[self.segments.len() - 1], self.head);
            self.tail_locations
                .insert(self.segments[self.segments.len() - 1]);
        }
    }

    fn get_num_tail(&self) -> usize {
        return self.tail_locations.len();
    }
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_b(&vec1), 36);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let steps = parse_input_to_directions(input);
    let mut state = StateSegmented::new(GridCoordinateInf::new(0, 0), 9);
    for (dir, num) in steps {
        state.move_step(dir, num);
    }
    return state.get_num_tail();
}
