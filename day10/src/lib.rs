extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::{GridCoordinate, GridTraversable};
pub use std::fmt::{Display, Formatter};

pub fn get_example_input() -> Vec<String> {
    return vec![
        "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
        "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1",
        "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1", "addx 16",
        "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop", "addx -3", "addx 9",
        "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop", "noop", "noop", "noop",
        "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop", "addx 2", "addx 6", "noop",
        "noop", "noop", "noop", "noop", "addx 1", "noop", "noop", "addx 7", "addx 1", "noop",
        "addx -13", "addx 13", "addx 7", "noop", "addx 1", "addx -33", "noop", "noop", "noop",
        "addx 2", "noop", "noop", "noop", "addx 8", "noop", "addx -1", "addx 2", "addx 1", "noop",
        "addx 17", "addx -9", "addx 1", "addx 1", "addx -3", "addx 11", "noop", "noop", "addx 1",
        "noop", "addx 1", "noop", "noop", "addx -13", "addx -19", "addx 1", "addx 3", "addx 26",
        "addx -30", "addx 12", "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9",
        "addx 18", "addx 1", "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1",
        "addx 2", "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22",
        "addx -6", "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop",
        "addx 20", "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Addx,
    Noop,
}

impl Operation {
    pub fn new(s: &str) -> Operation {
        return match s {
            "addx" => Operation::Addx,
            "noop" => Operation::Noop,
            _ => panic!("Unknown"),
        };
    }

    pub fn get_clock_cycles(&self) -> usize {
        return match *self {
            Operation::Addx => 2,
            Operation::Noop => 1,
        };
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operation::Addx => "addx",
            Operation::Noop => "noop",
        };
        return write!(f, "{}", s);
    }
}

#[derive(Debug)]
struct State {
    x: i32,
    clock: usize,
    screen_state: gridlib::Grid<bool>,
    // 1 = width 3 (left, middle, right).
    // 2 = width 5 (left1, left2, middle, right1, right2)
    sprite_pixels_adjacent: usize,
}

impl State {
    pub fn new() -> State {
        let all_off = vec![false; 40 * 6];
        let grid = gridlib::Grid::new(40, 6, all_off);
        return State {
            x: 1,
            clock: 1,
            screen_state: grid,
            sprite_pixels_adjacent: 1,
        };
    }

    pub fn do_op(&mut self, op: Operation, v: i32) {
        let sprite_radius: i32 = self.sprite_pixels_adjacent.try_into().unwrap();
        let width: i32 = self.screen_state.get_width().try_into().unwrap();

        let mut cycles = op.get_clock_cycles();
        while cycles > 0 {
            let clocki32: i32 = self.clock.try_into().unwrap();
            let col: i32 = (clocki32 - 1) % width;
            let min = self.x - sprite_radius;
            let max = self.x + sprite_radius;
            //println!("c: {}, x: {}", self.clock, self.x);
            //println!("min: {}, max: {}", min, max);
            let coordinate_x = (self.clock - 1) % self.screen_state.get_width();
            let coordinate_y = (self.clock - 1) / self.screen_state.get_width();
            let coordinate = GridCoordinate::new(coordinate_x, coordinate_y);
            let value = col >= min && col <= max;
            //println!("Storing {} to {}", value, coordinate);
            self.screen_state.set_value(coordinate, value);

            cycles -= 1;
            self.clock += 1;
        }
        match op {
            Operation::Addx => self.x += v,
            Operation::Noop => (),
        };
        //println!("c: {}, x: {}", self.clock, self.x);
    }

    pub fn get_signal_strength(&self, offset: usize) -> i32 {
        let clock_on_cycle: i32 = (self.clock + offset).try_into().unwrap();
        let s = clock_on_cycle * self.x;
        //println!("c: {}, x: {} = {}", clock_on_cycle, self.x, s);
        return s;
    }

    pub fn will_pass_cycle_mod(&self, op: Operation, cycle: usize) -> bool {
        let c = (self.clock + op.get_clock_cycles()) % cycle;
        return c != 0 && self.clock % cycle > c;
    }

    pub fn should_get_signal_strengh(&self, cycle: usize) -> bool {
        return self.clock % cycle == 0;
    }

    pub fn printscreen(&self) -> String {
        let mut s = "\n".to_string();
        let width = self.screen_state.get_width();
        let height = self.screen_state.get_height();
        for y in 0..height {
            for x in 0..width {
                if self
                    .screen_state
                    .get_value(GridCoordinate::new(x, y))
                    .unwrap()
                {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        return s;
    }
}

fn parse_input(input: &Vec<String>) -> Vec<(Operation, i32)> {
    return input
        .iter()
        .map(|line| {
            let mut v = 0;
            let op: Operation;
            if line.starts_with("n") {
                op = Operation::new(line);
            } else {
                let (a, b) = line.split_once(" ").unwrap();
                op = Operation::new(a);
                v = b.parse::<i32>().unwrap();
            }
            return (op, v);
        })
        .collect();
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = day10::get_example_input();
/// assert_eq!(day10::puzzle_a(&vec1), 13140);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> i32 {
    let mut s = State::new();
    let parsed = parse_input(input);
    let cycle_offset = 40;
    let cycle_v = 20;
    let mut cycle_i = 0;
    let mut signal_strength = 0;
    for (op, value) in parsed {
        //println!("{} {}", op, value);
        if s.will_pass_cycle_mod(op, cycle_v + cycle_i * cycle_offset) {
            signal_strength += s.get_signal_strength(1);
            cycle_i += 1;
        }
        s.do_op(op, value);
        if s.should_get_signal_strengh(cycle_v + cycle_i * cycle_offset) {
            signal_strength += s.get_signal_strength(0);
            cycle_i += 1;
        }
        if s.clock >= 220 {
            break;
        }
    }
    return signal_strength;
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = day10::get_example_input();
/// let expected = "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";
/// assert_eq!(day10::puzzle_b(&vec1), expected);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> String {
    let mut s = State::new();
    let parsed = parse_input(input);
    for (op, value) in parsed {
        s.do_op(op, value);
    }
    return s.printscreen();
}
