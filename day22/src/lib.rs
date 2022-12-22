extern crate filelib;

pub use filelib::load;
use gridlib::{Direction, Grid, GridCoordinate, GridTraversable};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum BoardTile {
    Solid,
    Open,
}

type Board = Grid<Option<BoardTile>>;

fn parse_input(input: &str) -> (Board, Vec<PathStep>) {
    let mut board_lines = vec![];
    let mut board_done = false;
    let mut board = Grid::new(0, 0, vec![]);
    for line in input.lines() {
        if line == "" && !board_done {
            board_done = true;
            board = parse_board(board_lines.clone());
            continue;
        }
        if board_done {
            return (board, parse_path(line));
        }
        board_lines.push(line);
    }
    unreachable!("Shouldn't ever happen")
}

fn parse_board(lines: Vec<&str>) -> Board {
    // 200 x 151, so should be fine to keep entirely in memory.
    // Board also never has a non-contigious row, which makes this a bit easier to part.
    let mut tiles: Vec<Option<BoardTile>> = vec![];
    let width = lines.iter().map(|l| l.len()).max().unwrap();
    let height = lines.len();
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            let tile = match c {
                ' ' => None,
                '.' => Some(BoardTile::Open),
                '#' => Some(BoardTile::Solid),
                _ => panic!("Input error"),
            };
            tiles.push(tile);
            x += 1;
        }
        for _ in x..width {
            tiles.push(None);
        }
    }

    let grid = Grid::new(width, height, tiles);
    return grid;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PathStep {
    Forward(i32),
    Left,
    Right,
}

impl PathStep {
    fn turn(&self, facing: Direction) -> Direction {
        return match self {
            PathStep::Forward(_) => facing,
            PathStep::Left => {
                if facing == Direction::NORTH {
                    Direction::WEST
                } else if facing == Direction::WEST {
                    Direction::SOUTH
                } else if facing == Direction::SOUTH {
                    Direction::EAST
                } else {
                    Direction::NORTH
                }
            }
            PathStep::Right => {
                if facing == Direction::NORTH {
                    Direction::EAST
                } else if facing == Direction::EAST {
                    Direction::SOUTH
                } else if facing == Direction::SOUTH {
                    Direction::WEST
                } else {
                    Direction::NORTH
                }
            }
        };
    }
}

fn parse_path(line: &str) -> Vec<PathStep> {
    let mut steps = vec![];
    let mut cur_number = "".to_string();
    for c in line.chars() {
        if c != 'L' && c != 'R' {
            cur_number.push(c);
            continue;
        }
        if cur_number.len() > 0 {
            let num = cur_number.parse::<i32>().unwrap();
            steps.push(PathStep::Forward(num));
            cur_number = "".to_string();
        }
        if c == 'L' {
            steps.push(PathStep::Left);
        } else {
            steps.push(PathStep::Right);
        }
    }
    if cur_number.len() > 0 {
        let num = cur_number.parse::<i32>().unwrap();
        steps.push(PathStep::Forward(num));
    }

    return steps;
}

/// Solution to puzzle_a entry point
/// ```
/// let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5";
/// assert_eq!(day22::puzzle_a(&input), 6032);
/// ```
pub fn puzzle_a(input: &str) -> usize {
    let (board, path) = parse_input(input);
    let mut start_coordinate = GridCoordinate::new(0, 0);
    let start_facing = Direction::EAST;
    for coord in board.coord_iter() {
        match board.get_value(coord) {
            Some(option) => match option {
                Some(tile) => match tile {
                    BoardTile::Solid => continue,
                    BoardTile::Open => {
                        start_coordinate = coord;
                        break;
                    }
                },
                None => continue,
            },
            None => panic!("Shouldn't happen"),
        }
    }
    return follow_path(&board, path, start_coordinate, start_facing);
}

// This doesn't check the vlaue, other then its a tile, it just finds the edge
fn find_edge(board: &Board, start: GridCoordinate, direction: Direction) -> GridCoordinate {
    println!("Finding edge from {} to {}", start, direction);
    let mut cur_coordinate = start;
    loop {
        match board.get_value(cur_coordinate) {
            Some(option) => match option {
                Some(_) => {
                    // We found the edge!
                    return cur_coordinate;
                }
                None => {
                    if let Some(coord) =
                        board.get_coordinate_by_direction(cur_coordinate, direction)
                    {
                        cur_coordinate = coord;
                    } else {
                        panic!("Failed to find edge");
                    }
                }
            },
            None => panic!("Failed to find edge"),
        }
    }
}

fn follow_path(
    board: &Board,
    path: Vec<PathStep>,
    start: GridCoordinate,
    start_facing: Direction,
) -> usize {
    println!("Following {:?}", path);
    let mut cur_facing = start_facing;
    let mut cur_coordinate = start;
    for step in path {
        println!("Doing step: {:?}", step);
        cur_facing = step.turn(cur_facing);
        if let PathStep::Forward(forward_steps) = step {
            for _ in 0..forward_steps {
                if let Some(new_coord) =
                    get_next_coord_in_direction(board, cur_coordinate, cur_facing)
                {
                    println!("Moved to {}", new_coord);
                    cur_coordinate = new_coord;
                } else {
                    // Hit a wall, stop moving forward
                    break;
                }
            }
        }
        //print_board(board, cur_coordinate, cur_facing);
    }

    let row_pass = 1000 * (cur_coordinate.y + 1);
    let col_pass = 4 * (cur_coordinate.x + 1);
    let direction_pass = match cur_facing {
        Direction::NORTH => 3,
        Direction::EAST => 0,
        Direction::SOUTH => 1,
        _ => 2,
    };
    return row_pass + col_pass + direction_pass;
}

fn get_next_coord_in_direction(
    board: &Board,
    location: GridCoordinate,
    direction: Direction,
) -> Option<GridCoordinate> {
    let width = board.get_width();
    let height = board.get_height();

    let mut possible_y: Option<usize> = Some(location.y);
    let mut possible_x: Option<usize> = Some(location.x);

    let mut is_wrapped = false;

    let mut wrapped_x_follow = location.x;
    let mut wrapped_y_follow = location.y;
    let wrapped_direction = direction;

    // REASONS WE MIGHT WRAP
    // we are going over the actual constraints of the board.
    if location.y == 0 && direction == Direction::NORTH {
        // Over the top edge, we are wrapping.
        is_wrapped = true;
        wrapped_y_follow = height - 1;
        println!("Top edge wrap detected");
    } else if location.y == height - 1 && direction == Direction::SOUTH {
        is_wrapped = true;
        wrapped_y_follow = 0;
        println!("Bottom edge wrap detected");
    } else if location.x == 0 && direction == Direction::WEST {
        is_wrapped = true;
        wrapped_x_follow = width - 1;
        println!("left edge wrap detected");
    } else if location.x == width - 1 && direction == Direction::EAST {
        is_wrapped = true;
        wrapped_x_follow = 0;
        println!("right edge wrap detected");
    }
    // We are still "on the board", but the next step is an empty space.
    // We will handle this case later.

    if is_wrapped {
        let coord = find_edge(
            board,
            GridCoordinate::new(wrapped_x_follow, wrapped_y_follow),
            wrapped_direction,
        );
        possible_x = Some(coord.x);
        possible_y = Some(coord.y);
        print_board(board, coord, wrapped_direction);
    } else {
        // Didn't wrap, if we should move forward, its a normal move
        match direction {
            Direction::NORTH => possible_y = location.y.checked_sub(1),
            Direction::EAST => possible_x = location.x.checked_add(1),
            Direction::SOUTH => possible_y = location.y.checked_add(1),
            Direction::WEST => possible_x = location.x.checked_sub(1),
            _ => unreachable!(),
        };
    }

    if let Some(new_x) = possible_x {
        if let Some(new_y) = possible_y {
            let coord = GridCoordinate::new(new_x, new_y);
            match board.get_value(coord) {
                Some(inner_option) => {
                    match inner_option {
                        None => {
                            println!("{} is empty, wrapping", coord);
                            // "On the board": but next step is an empty space.
                            match direction {
                                Direction::NORTH => {
                                    wrapped_y_follow = height - 1;
                                }
                                Direction::EAST => {
                                    wrapped_x_follow = 0;
                                }
                                Direction::SOUTH => {
                                    wrapped_y_follow = 0;
                                }
                                Direction::WEST => {
                                    wrapped_x_follow = width - 1;
                                }
                                _ => unreachable!(),
                            };
                            let coord2 = find_edge(
                                board,
                                GridCoordinate::new(wrapped_x_follow, wrapped_y_follow),
                                wrapped_direction,
                            );
                            print_board(board, coord2, wrapped_direction);
                            match board.get_value(coord2).unwrap().unwrap() {
                                BoardTile::Open => return Some(coord2),
                                BoardTile::Solid => return None,
                            }
                        }
                        Some(tile) => match tile {
                            BoardTile::Open => return Some(coord),
                            BoardTile::Solid => return None,
                        },
                    }
                }
                None => panic!("Logic error on get_next_coord_in_direction"),
            }
        }
    }
    return None;
}

fn print_board(b: &Board, m: GridCoordinate, f: Direction) {
    for y in 0..b.get_height() {
        let mut cur_string = "".to_string();
        for x in 0..b.get_width() {
            if m.x == x && m.y == y {
                match f {
                    Direction::NORTH => cur_string.push('^'),
                    Direction::EAST => cur_string.push('>'),
                    Direction::WEST => cur_string.push('<'),
                    Direction::SOUTH => cur_string.push('V'),
                    _ => unreachable!(),
                }
                continue;
            }
            let v = b.get_value(GridCoordinate::new(x, y)).unwrap();
            let c = match v {
                None => ' ',
                Some(tile) => match tile {
                    BoardTile::Solid => '#',
                    BoardTile::Open => '.',
                },
            };
            cur_string.push(c);
        }
        println!("{}", cur_string);
    }
}

/// Solution to puzzle_b entry point
/// ```
/// let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5";
/// assert_eq!(day22::puzzle_b(&input), 2);
/// ```
pub fn puzzle_b(_input: &str) -> i32 {
    return 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path() {
        let input = "10R5L5R10L4R5L5";
        let result: Vec<PathStep> = parse_path(input);
        let expected = vec![
            PathStep::Forward(10),
            PathStep::Right,
            PathStep::Forward(5),
            PathStep::Left,
            PathStep::Forward(5),
            PathStep::Right,
            PathStep::Forward(10),
            PathStep::Left,
            PathStep::Forward(4),
            PathStep::Right,
            PathStep::Forward(5),
            PathStep::Left,
            PathStep::Forward(5),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_board() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let result = parse_board(input.lines().collect());
        assert_eq!(result.get_width(), 16);
        assert_eq!(result.get_height(), 12);
    }

    #[test]
    fn test_parse_input() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5";
        let (result_board, result_path) = parse_input(input);
        assert_eq!(result_board.get_width(), 16);
        assert_eq!(result_board.get_height(), 12);
        let expected_path = vec![
            PathStep::Forward(10),
            PathStep::Right,
            PathStep::Forward(5),
            PathStep::Left,
            PathStep::Forward(5),
            PathStep::Right,
            PathStep::Forward(10),
            PathStep::Left,
            PathStep::Forward(4),
            PathStep::Right,
            PathStep::Forward(5),
            PathStep::Left,
            PathStep::Forward(5),
        ];
        assert_eq!(result_path, expected_path);
    }

    #[test]
    fn test_simple_boards_path() {
        let lines = vec!["                                                  .....#.#..#........#...........#..#....#....#................................#..#..............#...."];
        let board = parse_board(lines);
        let start = GridCoordinate::new(50, 0);
        let facing = Direction::WEST;
        let path = vec![PathStep::Forward(20)];
        let result = follow_path(&board, path, start, facing);
        assert_eq!(result, 1590);
    }
}
