extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::GridCoordinateInf64;
use rustc_hash::FxHashMap;

type ShapeCoord = GridCoordinateInf64;
type Shape = Vec<ShapeCoord>;

struct RockPattern {
    shapes: Vec<Shape>,
    heights: Vec<usize>,
    curr: usize,
}

// Rock patterns have 0,0 as top left corner.
impl RockPattern {
    fn new() -> RockPattern {
        let four_row = vec![
            ShapeCoord::new(0, 0),
            ShapeCoord::new(1, 0),
            ShapeCoord::new(2, 0),
            ShapeCoord::new(3, 0),
        ];
        let plus = vec![
            ShapeCoord::new(1, 0),
            ShapeCoord::new(0, 1),
            ShapeCoord::new(1, 1),
            ShapeCoord::new(2, 1),
            ShapeCoord::new(1, 2),
        ];
        let back_upper_l = vec![
            ShapeCoord::new(2, 0),
            ShapeCoord::new(2, 1),
            ShapeCoord::new(0, 2),
            ShapeCoord::new(1, 2),
            ShapeCoord::new(2, 2),
        ];
        let vertical = vec![
            ShapeCoord::new(0, 0),
            ShapeCoord::new(0, 1),
            ShapeCoord::new(0, 2),
            ShapeCoord::new(0, 3),
        ];
        let square = vec![
            ShapeCoord::new(0, 0),
            ShapeCoord::new(1, 0),
            ShapeCoord::new(0, 1),
            ShapeCoord::new(1, 1),
        ];
        // hardcode the heights
        return RockPattern {
            shapes: vec![four_row, plus, back_upper_l, vertical, square],
            curr: 0,
            heights: vec![1, 3, 3, 4, 2],
        };
    }

    // This is off by 1 from how I think about it
    // Since I think of what I just got, instad of whats coming next
    fn get_current_height(&self) -> usize {
        if self.curr == 0 {
            return self.heights[self.heights.len() - 1];
        }
        return self.heights[self.curr - 1];
    }

    fn get_max_height(&self) -> usize {
        return *self.heights.iter().max().unwrap();
    }
}

impl Iterator for RockPattern {
    type Item = Shape;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.shapes[self.curr].clone();

        self.curr += 1;
        self.curr %= self.shapes.len();

        // No endpoint, so never return None
        return Some(current);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum JetStream {
    Left,
    Right,
}

impl JetStream {
    fn move_pattern(&self, pattern: Vec<ShapeCoord>) -> Vec<ShapeCoord> {
        let translator = match self {
            JetStream::Left => ShapeCoord::new(-1, 0),
            JetStream::Right => ShapeCoord::new(1, 0),
        };
        return pattern
            .iter()
            .map(|coord| coord.clone() + translator)
            .collect();
    }
}

struct JetStreamPattern {
    pattern: Vec<JetStream>,
    curr: usize,
}

impl JetStreamPattern {
    fn new(pattern: Vec<JetStream>) -> JetStreamPattern {
        return JetStreamPattern {
            pattern: pattern,
            curr: 0,
        };
    }

    fn pattern_len(&self) -> usize {
        return self.pattern.len();
    }
}

impl Iterator for JetStreamPattern {
    type Item = JetStream;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.pattern[self.curr];

        self.curr += 1;
        self.curr %= self.pattern.len();

        // No endpoint, so never return None
        return Some(current);
    }
}

fn parse_jetstream(line: &str) -> JetStreamPattern {
    let pattern = line
        .chars()
        .map(|c| match c {
            '>' => JetStream::Right,
            '<' => JetStream::Left,
            _ => panic!("Bad input"),
        })
        .collect();
    return JetStreamPattern::new(pattern);
}

fn drop_blocks(p: &mut JetStreamPattern, max: usize) -> usize {
    let mut rocks = RockPattern::new();
    // Figure out the starting floor
    // tallest rock is 4 vertical, so floor can never be more than 4*max
    // Then we spawn 3 above that

    // these are for part b
    // 1000 skips found to work for the example, but not my output...
    let skip_deltas = 1000;
    // Make sure we do all the wind at least twice, to look for weird patterns
    // in case the wind has a large effect. And at least 10000 times.
    let num_simu = 10000.max(p.pattern_len() * 2) + skip_deltas;
    let mut deltas = vec![];
    let mut prev_height;

    let tallest = rocks.get_max_height();
    let floor_y = max.min(num_simu) * tallest + 3 + 1;
    let mut cur_spawn;
    let width = 7;
    let start_x = 2;
    // Symbols for state
    let rock = 1;
    let floor = 2;
    // for Part b, state needs to be a sparse map instead.
    // 2022 * 7 * 4 + 4 is still well within memory requirements.
    let mut state: FxHashMap<(usize, usize), usize> = FxHashMap::default();
    for i in 0..width {
        state.insert((i, floor_y), floor);
    }
    let mut rock_pos: Vec<ShapeCoord>;
    let mut jet;
    let mut potential_spot;
    let mut start_trans;
    let fall_trans = ShapeCoord::new(0, 1);
    let mut last_floor = floor_y;

    for _ in 0..max.min(num_simu) {
        let cur_rock = rocks.next().unwrap();
        // bottom is 3 above lowest rock / floor.
        //println!("current_height is {}", rocks.get_current_height());
        cur_spawn = last_floor - (3 + rocks.get_current_height());
        start_trans = ShapeCoord::new(start_x, cur_spawn.try_into().unwrap());
        rock_pos = cur_rock.iter().map(|c| c.clone() + start_trans).collect();
        //println!("Starting at: {:?}", rock_pos.clone());
        loop {
            jet = p.next().unwrap();
            potential_spot = jet.move_pattern(rock_pos.clone());
            // to check spot:
            if potential_spot.iter().any(|v| {
                if v.x < 0 {
                    return true;
                }
                let y_usize: usize = v.y.try_into().unwrap();
                let x_usize: usize = v.x.try_into().unwrap();
                return x_usize >= width.try_into().unwrap()
                    || state.contains_key(&(x_usize, y_usize));
            }) {
                // Bad spot, don't move
            } else {
                // Good spot
                rock_pos = potential_spot;
            }
            // Next, try falling!

            potential_spot = rock_pos
                .iter()
                .map(|coord| coord.clone() + fall_trans)
                .collect();
            if potential_spot.iter().any(|v| {
                if v.x < 0 {
                    return true;
                }
                let y_usize: usize = v.y.try_into().unwrap();
                let x_usize: usize = v.x.try_into().unwrap();
                return y_usize >= floor_y
                    || x_usize >= width.try_into().unwrap()
                    || state.contains_key(&(x_usize, y_usize));
            }) {
                // Bad spot, we are done!
                for pos in rock_pos.iter() {
                    let y_usize: usize = pos.y.try_into().unwrap();
                    let x_usize: usize = pos.x.try_into().unwrap();
                    //println!("Freezing ({},{})", x_usize, y_usize);
                    state.insert((x_usize, y_usize), rock);
                }
                break;
            } else {
                // Good spot
                rock_pos = potential_spot;
            }
            // Loop, next jet and next fall!
        }
        prev_height = last_floor;
        // determine new spawn point:
        for pos in rock_pos {
            last_floor = last_floor.min(pos.y.try_into().unwrap());
        }
        deltas.push(prev_height - last_floor);
    }

    // Part b
    if max.min(num_simu) != max {
        // skip the first few deltas to gret rid of the floor effect
        let mut pattern_len = 0;
        let pattern_search = &deltas[skip_deltas..];
        for possible_len in 1..=pattern_search.len() / 2 {
            let pattern = &pattern_search[0..possible_len];
            let mut found = true;
            for i in 0..pattern_search.len() - possible_len {
                if pattern_search[i + possible_len] != pattern[i % possible_len] {
                    // not the pattern
                    found = false;
                    break;
                }
            }
            if found {
                pattern_len = possible_len;
                break;
            }
        }
        // We found the pattern! Now to compute the pattern
        let pattern = &pattern_search[0..pattern_len];
        let pattern_sum: usize = pattern.iter().sum();
        // Use an bunch of the recorded deltas to make sure the floor has been accounted for
        let initial_deltas = &deltas[0..deltas.len() / 4];
        let initial_sum: usize = initial_deltas.iter().sum();

        let num_patterns = (max - initial_deltas.len()) / pattern.len();
        let num_leftover = (max - initial_deltas.len()) % pattern.len();
        let leftover_sum: usize = pattern[0..num_leftover].iter().sum();

        return initial_sum + pattern_sum * num_patterns + leftover_sum;
        // 1595988538654 is too low...
    } else {
        // Simulation done, cur_spawn should be height.
        //println!("floor: {}, last_floor: {}", floor_y, last_floor);
        return floor_y - last_floor;
    }
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day17::puzzle_a(&vec1), 3068);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let num_rocks = 2022;
    let mut p = parse_jetstream(&input[0]);
    return drop_blocks(&mut p, num_rocks);
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day17::puzzle_b(&vec1), 1514285714288);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let num_rocks = 1000000000000;
    let mut p = parse_jetstream(&input[0]);
    return drop_blocks(&mut p, num_rocks);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patterns() {
        let mut pattern = RockPattern::new();
        let mut cur = pattern.next().unwrap();
        assert_eq!(cur.len(), 4);
        cur = pattern.next().unwrap();
        assert_eq!(cur.len(), 5);
        cur = pattern.next().unwrap();
        assert_eq!(cur.len(), 5);
        cur = pattern.next().unwrap();
        assert_eq!(cur.len(), 4);
        cur = pattern.next().unwrap();
        assert_eq!(cur.len(), 4);
        cur = pattern.next().unwrap();
        // loop!
        assert_eq!(cur.len(), 4);
        cur = pattern.next().unwrap();
        assert_eq!(cur.len(), 5);
    }

    #[test]
    fn test_jetstream_parse() {
        let s = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut p = parse_jetstream(s);
        let mut v = vec![];
        let size_plus = s.len() + 1;
        for _ in 0..size_plus {
            v.push(p.next().unwrap());
        }
        let mut expected = vec![
            JetStream::Right,
            JetStream::Right,
            JetStream::Right,
            JetStream::Left,
            JetStream::Left,
            JetStream::Right,
            JetStream::Left,
            JetStream::Right,
            JetStream::Right,
            JetStream::Left,
            JetStream::Left,
            JetStream::Left,
            JetStream::Right,
            JetStream::Right,
            JetStream::Left,
            JetStream::Right,
            JetStream::Right,
            JetStream::Right,
            JetStream::Left,
            JetStream::Left,
            JetStream::Left,
            JetStream::Right,
            JetStream::Right,
            JetStream::Right,
            JetStream::Left,
            JetStream::Left,
            JetStream::Left,
            JetStream::Right,
            JetStream::Left,
            JetStream::Left,
            JetStream::Left,
            JetStream::Right,
            JetStream::Right,
            JetStream::Left,
            JetStream::Right,
            JetStream::Right,
            JetStream::Left,
            JetStream::Left,
            JetStream::Right,
            JetStream::Right,
        ];
        // Loop
        expected.push(JetStream::Right);
        assert_eq!(v, expected);
    }

    #[test]
    fn test_drop_blocks_full_example() {
        let s = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut p = parse_jetstream(s);

        assert_eq!(drop_blocks(&mut p, 10), 17);
    }

    #[test]
    fn test_drop_blocks_1_example() {
        let s = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut p = parse_jetstream(s);

        assert_eq!(drop_blocks(&mut p, 1), 1);
    }

    #[test]
    fn test_drop_blocks_2_example() {
        let s = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut p = parse_jetstream(s);

        assert_eq!(drop_blocks(&mut p, 2), 4);
    }

    #[test]
    fn test_drop_blocks_3_example() {
        let s = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut p = parse_jetstream(s);

        assert_eq!(drop_blocks(&mut p, 3), 6);
    }

    #[test]
    fn test_drop_blocks_4_example() {
        let s = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut p = parse_jetstream(s);

        assert_eq!(drop_blocks(&mut p, 4), 7);
    }
}
