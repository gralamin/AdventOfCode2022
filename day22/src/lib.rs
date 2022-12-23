extern crate filelib;

pub use filelib::load;
use gridlib::{Direction, Grid, GridCoordinate, GridTraversable};
use std::cmp::min;

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
    //println!("Finding edge from {} to {}", start, direction);
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
    //println!("Following {:?}", path);
    let mut cur_facing = start_facing;
    let mut cur_coordinate = start;
    for step in path {
        //println!("Doing step: {:?}", step);
        cur_facing = step.turn(cur_facing);
        if let PathStep::Forward(forward_steps) = step {
            for _ in 0..forward_steps {
                if let Some(new_coord) =
                    get_next_coord_in_direction(board, cur_coordinate, cur_facing)
                {
                    //println!("Moved to {}", new_coord);
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
        //println!("Top edge wrap detected");
    } else if location.y == height - 1 && direction == Direction::SOUTH {
        is_wrapped = true;
        wrapped_y_follow = 0;
        //println!("Bottom edge wrap detected");
    } else if location.x == 0 && direction == Direction::WEST {
        is_wrapped = true;
        wrapped_x_follow = width - 1;
        //println!("left edge wrap detected");
    } else if location.x == width - 1 && direction == Direction::EAST {
        is_wrapped = true;
        wrapped_x_follow = 0;
        //println!("right edge wrap detected");
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
        //print_board(board, coord, wrapped_direction);
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
                            //println!("{} is empty, wrapping", coord);
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
                            //print_board(board, coord2, wrapped_direction);
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Chunk {
    chunk_id: usize,
    top_left: GridCoordinate,
    bottom_right: GridCoordinate,
    // left edge warp is defined relative to the top left coordinate
    // Warp is: chunk_id, edge direction on the map, and new direction facing
    left_edge_warp: (usize, Direction, Direction),
    // right edge warp is defined relative to the top right coordinate
    right_edge_warp: (usize, Direction, Direction),
    // bottom edge warp is defined relative to the bottom left coordinate
    bottom_edge_warp: (usize, Direction, Direction),
    // bottom edge warp is defined relative to the top left coordinate
    top_edge_warp: (usize, Direction, Direction),
}

impl Chunk {
    fn manual_mapped(chunk_size: usize) -> Vec<Chunk> {
        if chunk_size == 4 {
            // example input
            return vec![
                Chunk {
                    chunk_id: 1,
                    top_left: GridCoordinate::new(8, 0),
                    bottom_right: GridCoordinate::new(11, 3),
                    left_edge_warp: (3, Direction::NORTH, Direction::SOUTH),
                    right_edge_warp: (6, Direction::EAST, Direction::WEST),
                    top_edge_warp: (2, Direction::NORTH, Direction::SOUTH),
                    bottom_edge_warp: (4, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 2,
                    top_left: GridCoordinate::new(0, 4),
                    bottom_right: GridCoordinate::new(3, 7),
                    left_edge_warp: (4, Direction::SOUTH, Direction::NORTH),
                    right_edge_warp: (3, Direction::WEST, Direction::EAST),
                    top_edge_warp: (1, Direction::NORTH, Direction::SOUTH),
                    bottom_edge_warp: (5, Direction::SOUTH, Direction::NORTH),
                },
                Chunk {
                    chunk_id: 3,
                    top_left: GridCoordinate::new(4, 4),
                    bottom_right: GridCoordinate::new(7, 7),
                    left_edge_warp: (2, Direction::EAST, Direction::WEST),
                    right_edge_warp: (4, Direction::WEST, Direction::EAST),
                    top_edge_warp: (1, Direction::WEST, Direction::EAST),
                    bottom_edge_warp: (4, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 4,
                    top_left: GridCoordinate::new(8, 4),
                    bottom_right: GridCoordinate::new(11, 7),
                    left_edge_warp: (3, Direction::EAST, Direction::WEST),
                    right_edge_warp: (6, Direction::NORTH, Direction::SOUTH),
                    top_edge_warp: (1, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (5, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 5,
                    top_left: GridCoordinate::new(8, 8),
                    bottom_right: GridCoordinate::new(11, 11),
                    left_edge_warp: (3, Direction::SOUTH, Direction::NORTH),
                    right_edge_warp: (6, Direction::WEST, Direction::EAST),
                    top_edge_warp: (1, Direction::WEST, Direction::EAST),
                    bottom_edge_warp: (2, Direction::SOUTH, Direction::NORTH),
                },
                Chunk {
                    chunk_id: 6,
                    top_left: GridCoordinate::new(12, 8),
                    bottom_right: GridCoordinate::new(15, 11),
                    left_edge_warp: (5, Direction::EAST, Direction::WEST),
                    right_edge_warp: (1, Direction::EAST, Direction::WEST),
                    top_edge_warp: (4, Direction::EAST, Direction::WEST),
                    bottom_edge_warp: (2, Direction::WEST, Direction::EAST),
                },
            ];
        } else if chunk_size == 50 {
            return vec![
                Chunk {
                    chunk_id: 3,
                    top_left: GridCoordinate::new(50, 50),
                    bottom_right: GridCoordinate::new(99, 99),
                    left_edge_warp: (4, Direction::NORTH, Direction::SOUTH),
                    right_edge_warp: (2, Direction::SOUTH, Direction::NORTH),
                    top_edge_warp: (1, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (5, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 6,
                    top_left: GridCoordinate::new(0, 150),
                    bottom_right: GridCoordinate::new(49, 199),
                    left_edge_warp: (1, Direction::NORTH, Direction::SOUTH),
                    right_edge_warp: (5, Direction::SOUTH, Direction::NORTH),
                    top_edge_warp: (4, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (2, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 4,
                    top_left: GridCoordinate::new(0, 100),
                    bottom_right: GridCoordinate::new(49, 149),
                    left_edge_warp: (1, Direction::WEST, Direction::EAST),
                    right_edge_warp: (5, Direction::WEST, Direction::EAST),
                    top_edge_warp: (3, Direction::WEST, Direction::EAST),
                    bottom_edge_warp: (6, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 5,
                    top_left: GridCoordinate::new(50, 100),
                    bottom_right: GridCoordinate::new(99, 149),
                    left_edge_warp: (4, Direction::EAST, Direction::WEST),
                    right_edge_warp: (2, Direction::EAST, Direction::WEST),
                    top_edge_warp: (3, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (6, Direction::EAST, Direction::WEST),
                },
                Chunk {
                    chunk_id: 1,
                    top_left: GridCoordinate::new(50, 0),
                    bottom_right: GridCoordinate::new(99, 49),
                    left_edge_warp: (4, Direction::WEST, Direction::EAST),
                    right_edge_warp: (2, Direction::WEST, Direction::EAST),
                    top_edge_warp: (6, Direction::WEST, Direction::EAST),
                    bottom_edge_warp: (3, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 2,
                    top_left: GridCoordinate::new(100, 0),
                    bottom_right: GridCoordinate::new(149, 49),
                    left_edge_warp: (1, Direction::EAST, Direction::WEST),
                    right_edge_warp: (5, Direction::EAST, Direction::WEST),
                    top_edge_warp: (6, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (3, Direction::EAST, Direction::WEST),
                },
            ];
        } else if chunk_size == 2 {
            // mini board of real data, for testing purposes
            return vec![
                Chunk {
                    chunk_id: 3,
                    top_left: GridCoordinate::new(2, 2),
                    bottom_right: GridCoordinate::new(3, 3),
                    left_edge_warp: (4, Direction::NORTH, Direction::SOUTH),
                    right_edge_warp: (2, Direction::SOUTH, Direction::NORTH),
                    top_edge_warp: (1, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (5, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 6,
                    top_left: GridCoordinate::new(0, 6),
                    bottom_right: GridCoordinate::new(1, 7),
                    left_edge_warp: (1, Direction::NORTH, Direction::SOUTH),
                    right_edge_warp: (5, Direction::SOUTH, Direction::NORTH),
                    top_edge_warp: (4, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (2, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 4,
                    top_left: GridCoordinate::new(0, 4),
                    bottom_right: GridCoordinate::new(1, 5),
                    left_edge_warp: (1, Direction::WEST, Direction::EAST),
                    right_edge_warp: (5, Direction::WEST, Direction::EAST),
                    top_edge_warp: (3, Direction::WEST, Direction::EAST),
                    bottom_edge_warp: (6, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 5,
                    top_left: GridCoordinate::new(2, 4),
                    bottom_right: GridCoordinate::new(3, 5),
                    left_edge_warp: (4, Direction::EAST, Direction::WEST),
                    right_edge_warp: (2, Direction::EAST, Direction::WEST),
                    top_edge_warp: (3, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (6, Direction::EAST, Direction::WEST),
                },
                Chunk {
                    chunk_id: 1,
                    top_left: GridCoordinate::new(2, 0),
                    bottom_right: GridCoordinate::new(3, 1),
                    left_edge_warp: (4, Direction::WEST, Direction::EAST),
                    right_edge_warp: (2, Direction::WEST, Direction::EAST),
                    top_edge_warp: (6, Direction::WEST, Direction::EAST),
                    bottom_edge_warp: (3, Direction::NORTH, Direction::SOUTH),
                },
                Chunk {
                    chunk_id: 2,
                    top_left: GridCoordinate::new(4, 0),
                    bottom_right: GridCoordinate::new(5, 1),
                    left_edge_warp: (1, Direction::EAST, Direction::WEST),
                    right_edge_warp: (5, Direction::EAST, Direction::WEST),
                    top_edge_warp: (6, Direction::SOUTH, Direction::NORTH),
                    bottom_edge_warp: (3, Direction::EAST, Direction::WEST),
                },
            ];
        } else {
            panic!("Unknown chunk size");
        }
    }
}

/// Solution to puzzle_b entry point
/// ```
/// let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5";
/// assert_eq!(day22::puzzle_b(&input), 5031);
/// ```
pub fn puzzle_b(input: &str) -> usize {
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

    let chunk_size = min(board.get_width() / 3, board.get_height() / 3);
    // Its probably possible to actually fold these somehow, but not sure how to do it
    let chunks = Chunk::manual_mapped(chunk_size);

    return follow_path_chunks(&board, path, start_coordinate, start_facing, &chunks);
}

fn follow_path_chunks(
    board: &Board,
    path: Vec<PathStep>,
    start: GridCoordinate,
    start_facing: Direction,
    chunks: &Vec<Chunk>,
) -> usize {
    //println!("Following {:?}", path);
    let mut cur_facing = start_facing;
    let mut cur_coordinate = start;
    for step in path {
        //println!("Doing step: {:?}", step);
        cur_facing = step.turn(cur_facing);
        if let PathStep::Forward(forward_steps) = step {
            for _ in 0..forward_steps {
                if let Some((new_coord, new_direction)) =
                    get_next_coord_in_direction_chunks(board, cur_coordinate, cur_facing, chunks)
                {
                    //println!("Moved to {}, facing: {}", new_coord, new_direction);
                    cur_coordinate = new_coord;
                    cur_facing = new_direction;
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

fn get_cur_chunk(chunks: &Vec<Chunk>, location: GridCoordinate) -> Chunk {
    //println!("Getting chunk for {}", location);
    return chunks
        .iter()
        .filter(|c| {
            c.top_left.x <= location.x
                && c.bottom_right.x >= location.x
                && c.top_left.y <= location.y
                && c.bottom_right.y >= location.y
        })
        .next()
        .unwrap()
        .clone();
}

fn warp_left(chunks: &Vec<Chunk>, chunk: Chunk, location: GridCoordinate) -> (usize, usize) {
    let next_chunk = chunks
        .iter()
        .filter(|c| c.chunk_id == chunk.left_edge_warp.0)
        .next()
        .unwrap();
    let new_x;
    let new_y;
    match chunk.left_edge_warp.1 {
        Direction::NORTH => {
            // 90 degree turn, the y doesn't change here, but the x should be based on the previous y
            let x_diff = location.y - chunk.top_left.y;
            new_x = next_chunk.top_left.x + x_diff;
            new_y = next_chunk.top_left.y;
        }
        Direction::EAST => {
            // Hey we are just beside each other.
            let y_diff = chunk.bottom_right.y - location.y;
            new_x = next_chunk.bottom_right.x;
            new_y = next_chunk.bottom_right.y - y_diff;
        }
        Direction::SOUTH => {
            // Like North, but other direction
            let x_diff = chunk.bottom_right.y - location.y;
            new_x = next_chunk.top_left.x + x_diff;
            new_y = next_chunk.bottom_right.y;
        }
        Direction::WEST => {
            let y_diff = chunk.bottom_right.y - location.y;
            new_x = next_chunk.top_left.x;
            new_y = next_chunk.top_left.y + y_diff;
        }
        _ => unreachable!(),
    }
    return (new_x, new_y);
}

fn warp_right(chunks: &Vec<Chunk>, chunk: Chunk, location: GridCoordinate) -> (usize, usize) {
    let next_chunk = chunks
        .iter()
        .filter(|c| c.chunk_id == chunk.right_edge_warp.0)
        .next()
        .unwrap();
    let new_x;
    let new_y;
    //println!("Going from {:?} to {:?}", chunk, next_chunk);
    match chunk.right_edge_warp.1 {
        Direction::NORTH => {
            // 90 degree turn, the y doesn't change here, but the x should be based on the previous y
            let x_diff = chunk.bottom_right.y - location.y;
            new_x = next_chunk.top_left.x + x_diff;
            new_y = next_chunk.top_left.y;
        }
        Direction::EAST => {
            // essentially the same as WEST case.
            // TR to BR
            // BR to TR
            let y_diff = chunk.bottom_right.y - location.y;
            new_x = next_chunk.bottom_right.x;
            new_y = next_chunk.top_left.y + y_diff;
        }
        Direction::SOUTH => {
            // Like North, but other direction
            let x_diff = location.y - chunk.top_left.y;
            new_x = next_chunk.top_left.x + x_diff;
            new_y = next_chunk.bottom_right.y;
        }
        Direction::WEST => {
            // Hey we are just beside each other.
            let y_diff = location.y - chunk.top_left.y;
            new_x = next_chunk.top_left.x;
            new_y = next_chunk.top_left.y + y_diff;
        }
        _ => unreachable!(),
    }
    return (new_x, new_y);
}

fn warp_top(chunks: &Vec<Chunk>, chunk: Chunk, location: GridCoordinate) -> (usize, usize) {
    let next_chunk = chunks
        .iter()
        .filter(|c| c.chunk_id == chunk.top_edge_warp.0)
        .next()
        .unwrap();
    let new_x;
    let new_y;
    match chunk.top_edge_warp.1 {
        Direction::NORTH => {
            let x_diff = chunk.bottom_right.x - location.x;
            new_y = next_chunk.top_left.y;
            new_x = next_chunk.top_left.x + x_diff;
        }
        Direction::EAST => {
            // TL on chunk should be BR
            // TR should be TR
            let y_diff = chunk.bottom_right.x - location.x;
            new_y = next_chunk.top_left.y + y_diff;
            new_x = next_chunk.bottom_right.x;
        }
        Direction::SOUTH => {
            // Like North, but other direction
            let x_diff = location.x - chunk.top_left.x;
            new_y = next_chunk.bottom_right.y;
            new_x = next_chunk.top_left.x + x_diff;
        }
        Direction::WEST => {
            // TL on chunk should be TL
            // TR should be BL
            let y_diff = location.x - chunk.top_left.x;
            new_y = next_chunk.top_left.y + y_diff;
            new_x = next_chunk.top_left.x;
        }
        _ => unreachable!(),
    }
    return (new_x, new_y);
}

fn warp_bottom(chunks: &Vec<Chunk>, chunk: Chunk, location: GridCoordinate) -> (usize, usize) {
    let next_chunk = chunks
        .iter()
        .filter(|c| c.chunk_id == chunk.bottom_edge_warp.0)
        .next()
        .unwrap();
    let new_x;
    let new_y;
    //println!("Going from {:?} to {:?}", chunk, next_chunk);
    match chunk.bottom_edge_warp.1 {
        Direction::NORTH => {
            let x_diff = location.x - chunk.top_left.x;
            new_y = next_chunk.top_left.y;
            new_x = next_chunk.top_left.x + x_diff;
        }
        Direction::EAST => {
            // 90 degree turn, the x doesn't change here, but the y should be based on the previous x
            let y_diff = location.x - chunk.top_left.x;
            new_y = next_chunk.top_left.y + y_diff;
            new_x = next_chunk.bottom_right.x;
        }
        Direction::SOUTH => {
            // Like North, but other direction
            let x_diff = chunk.bottom_right.x - location.x;
            new_y = next_chunk.bottom_right.y;
            new_x = next_chunk.top_left.x + x_diff;
        }
        Direction::WEST => {
            // bottom right = top_left
            // bottom left = bottom left
            let y_diff = chunk.bottom_right.x - location.x;
            new_y = next_chunk.top_left.y + y_diff;
            new_x = next_chunk.top_left.x;
        }
        _ => unreachable!(),
    }
    return (new_x, new_y);
}

fn get_next_coord_in_direction_chunks(
    board: &Board,
    location: GridCoordinate,
    direction: Direction,
    chunks: &Vec<Chunk>,
) -> Option<(GridCoordinate, Direction)> {
    let width = board.get_width();
    let height = board.get_height();

    let mut possible_y: Option<usize> = Some(location.y);
    let mut possible_x: Option<usize> = Some(location.x);

    let mut is_wrapped = false;

    let mut wrapped_x_follow = location.x;
    let mut wrapped_y_follow = location.y;
    let mut wrapped_direction = direction;

    let chunk = get_cur_chunk(chunks, location);

    // We will always wrap if we are moving off an edge of a chunk
    if chunk.top_left.x == location.x && direction == Direction::WEST {
        //println!("left Warp detected");
        //print_board(board, location, direction);
        (wrapped_x_follow, wrapped_y_follow) = warp_left(chunks, chunk, location);
        wrapped_direction = chunk.left_edge_warp.2;
        is_wrapped = true;
    } else if chunk.bottom_right.x == location.x && direction == Direction::EAST {
        //println!("right Warp detected");
        //print_board(board, location, direction);
        (wrapped_x_follow, wrapped_y_follow) = warp_right(chunks, chunk, location);
        wrapped_direction = chunk.right_edge_warp.2;
        is_wrapped = true;
    } else if chunk.top_left.y == location.y && direction == Direction::NORTH {
        //println!("top Warp detected");
        //print_board(board, location, direction);
        (wrapped_x_follow, wrapped_y_follow) = warp_top(chunks, chunk, location);
        wrapped_direction = chunk.top_edge_warp.2;
        is_wrapped = true;
    } else if chunk.bottom_right.y == location.y && direction == Direction::SOUTH {
        //println!("bottom Warp detected");
        //print_board(board, location, direction);
        (wrapped_x_follow, wrapped_y_follow) = warp_bottom(chunks, chunk, location);
        wrapped_direction = chunk.bottom_edge_warp.2;
        is_wrapped = true;
    }

    if is_wrapped {
        let coord = find_edge(
            board,
            GridCoordinate::new(wrapped_x_follow, wrapped_y_follow),
            wrapped_direction,
        );
        possible_x = Some(coord.x);
        possible_y = Some(coord.y);
        //print_board(board, coord, wrapped_direction);
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
                            //println!("{} is empty, wrapping", coord);
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
                            //print_board(board, coord2, wrapped_direction);
                            match board.get_value(coord2).unwrap().unwrap() {
                                BoardTile::Open => return Some((coord2, wrapped_direction)),
                                BoardTile::Solid => return None,
                            }
                        }
                        Some(tile) => match tile {
                            BoardTile::Open => return Some((coord, wrapped_direction)),
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

    #[test]
    fn test_warp_left_west_to_east() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(8, 4);
        let mut direction = Direction::WEST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(7, 4));
        assert_eq!(result_direction, direction);

        let bottom_left = GridCoordinate::new(4, 7);
        direction = Direction::WEST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(3, 7));
        assert_eq!(result_direction, direction);

        let input = "  ....\n  ....\n  ..  \n  ..  \n....  \n....  \n..    \n..    ";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(2);
        let top_left = GridCoordinate::new(4, 0);
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(3, 0));
        assert_eq!(result_direction, Direction::WEST);

        let bottom_left = GridCoordinate::new(4, 1);
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(3, 1));
        assert_eq!(result_direction, Direction::WEST);
    }

    #[test]
    fn test_warp_left_west_to_north() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(8, 0);
        let mut direction = Direction::WEST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(4, 4));
        assert_eq!(result_direction, Direction::SOUTH);

        let bottom_left = GridCoordinate::new(8, 3);
        direction = Direction::WEST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(7, 4));
        assert_eq!(result_direction, Direction::SOUTH);

        let input = "  ....\n  ....\n  ..  \n  ..  \n....  \n....  \n..    \n..    ";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(2);
        let top_left = GridCoordinate::new(0, 6);
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(2, 0));
        assert_eq!(result_direction, Direction::SOUTH);

        let bottom_left = GridCoordinate::new(0, 7);
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(3, 0));
        assert_eq!(result_direction, Direction::SOUTH);
    }

    #[test]
    fn test_warp_left_west_to_south() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(8, 8);
        let mut direction = Direction::WEST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(7, 7));
        assert_eq!(result_direction, Direction::NORTH);

        let bottom_left = GridCoordinate::new(8, 11);
        direction = Direction::WEST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(4, 7));
        assert_eq!(result_direction, Direction::NORTH);
    }

    #[test]
    fn test_warp_left_west_to_west() {
        /* chunk size 2 output:
        |   ....
        |   ....
        |   ..
        |   ..
        | ....
        | ....
        | ..
        | ..
        */
        let input = "  ....\n  ....\n  ..  \n  ..  \n....  \n....  \n..    \n..    ";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(2);

        let top_left = GridCoordinate::new(0, 4);
        let mut direction = Direction::WEST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(2, 1));
        assert_eq!(result_direction, Direction::EAST);

        let bottom_left = GridCoordinate::new(0, 5);
        direction = Direction::WEST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(2, 0));
        assert_eq!(result_direction, Direction::EAST);
    }

    #[test]
    fn test_warp_top_north_to_south() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(9, 4);
        let mut direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(9, 3));
        assert_eq!(result_direction, direction);

        let bottom_left = GridCoordinate::new(11, 4);
        direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(11, 3));
        assert_eq!(result_direction, direction);
    }

    #[test]
    fn test_warp_top_north_to_north() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(1, 4);
        let mut direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(10, 0));
        assert_eq!(result_direction, Direction::SOUTH);

        let bottom_left = GridCoordinate::new(2, 4);
        direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(9, 0));
        assert_eq!(result_direction, Direction::SOUTH);
    }

    #[test]
    fn test_warp_top_north_to_east() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(12, 8);
        let mut direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(11, 7));
        assert_eq!(result_direction, Direction::WEST);

        let bottom_left = GridCoordinate::new(14, 8);
        direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(11, 5));
        assert_eq!(result_direction, Direction::WEST);
    }

    #[test]
    fn test_warp_top_north_to_west() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(4, 4);
        let mut direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(8, 0));
        assert_eq!(result_direction, Direction::EAST);

        let bottom_left = GridCoordinate::new(7, 4);
        direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(8, 3));
        assert_eq!(result_direction, Direction::EAST);

        let input = "  ....\n  ....\n  ..  \n  ..  \n....  \n....  \n..    \n..    ";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(2);
        let top_left = GridCoordinate::new(2, 0);
        let mut direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(0, 6));
        assert_eq!(result_direction, Direction::EAST);

        let bottom_left = GridCoordinate::new(3, 0);
        direction = Direction::NORTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(0, 7));
        assert_eq!(result_direction, Direction::EAST);
    }

    #[test]
    fn test_warp_bottom_south_to_north() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(8, 3);
        let mut direction = Direction::SOUTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(8, 4));
        assert_eq!(result_direction, direction);

        let bottom_left = GridCoordinate::new(10, 3);
        direction = Direction::SOUTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(10, 4));
        assert_eq!(result_direction, direction);

        let input = "  ....\n  ....\n  ..  \n  ..  \n....  \n....  \n..    \n..    ";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(2);
        let top_left = GridCoordinate::new(0, 7);
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(4, 0));
        assert_eq!(result_direction, Direction::SOUTH);

        let bottom_left = GridCoordinate::new(1, 7);
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(5, 0));
        assert_eq!(result_direction, Direction::SOUTH);
    }

    #[test]
    fn test_warp_bottom_south_to_south() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(8, 11);
        let mut direction = Direction::SOUTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(3, 7));
        assert_eq!(result_direction, Direction::NORTH);

        let bottom_left = GridCoordinate::new(11, 11);
        direction = Direction::SOUTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(0, 7));
        assert_eq!(result_direction, Direction::NORTH);
    }

    #[test]
    fn test_warp_bottom_south_to_west() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(12, 11);
        let mut direction = Direction::SOUTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(0, 7));
        assert_eq!(result_direction, Direction::EAST);

        let bottom_left = GridCoordinate::new(15, 11);
        direction = Direction::SOUTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(0, 4));
        assert_eq!(result_direction, Direction::EAST);
    }

    #[test]
    fn test_warp_bottom_south_to_east() {
        /* chunk size 2 output:
        |   ....
        |   ....
        |   ..
        |   ..
        | ....
        | ....
        | ..
        | ..
        */
        let input = "  ....\n  ....\n  ..  \n  ..  \n....  \n....  \n..    \n..    ";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(2);

        let top_left = GridCoordinate::new(4, 1);
        let mut direction = Direction::SOUTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(3, 2));
        assert_eq!(result_direction, Direction::WEST);

        let bottom_left = GridCoordinate::new(5, 1);
        direction = Direction::SOUTH;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(3, 3));
        assert_eq!(result_direction, Direction::WEST);
    }

    #[test]
    fn test_warp_right_east_to_west() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(7, 4);
        let mut direction = Direction::EAST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(8, 4));
        assert_eq!(result_direction, direction);

        let bottom_left = GridCoordinate::new(7, 7);
        direction = Direction::EAST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(8, 7));
        assert_eq!(result_direction, direction);
    }

    #[test]
    fn test_warp_right_east_to_east() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(11, 0);
        let mut direction = Direction::EAST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(15, 11));
        assert_eq!(result_direction, Direction::WEST);

        let bottom_left = GridCoordinate::new(11, 3);
        direction = Direction::EAST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(15, 8));
        assert_eq!(result_direction, Direction::WEST);
    }

    #[test]
    fn test_warp_right_east_to_north() {
        let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(4);

        let top_left = GridCoordinate::new(11, 4);
        let mut direction = Direction::EAST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(15, 8));
        assert_eq!(result_direction, Direction::SOUTH);

        let bottom_left = GridCoordinate::new(11, 7);
        direction = Direction::EAST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(12, 8));
        assert_eq!(result_direction, Direction::SOUTH);
    }

    #[test]
    fn test_warp_right_east_to_south() {
        /* chunk size 2 output:
        |   ....
        |   ....
        |   ..
        |   ..
        | ....
        | ....
        | ..
        | ..
        */
        let input = "  ....\n  ....\n  ..  \n  ..  \n....  \n....  \n..    \n..    ";
        let board = parse_board(input.lines().collect());
        let chunks = Chunk::manual_mapped(2);

        let top_left = GridCoordinate::new(3, 2);
        let mut direction = Direction::EAST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, top_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(4, 1));
        assert_eq!(result_direction, Direction::NORTH);

        let bottom_left = GridCoordinate::new(3, 3);
        direction = Direction::EAST;
        let (result_coord, result_direction) =
            get_next_coord_in_direction_chunks(&board, bottom_left, direction, &chunks).unwrap();
        assert_eq!(result_coord, GridCoordinate::new(5, 1));
        assert_eq!(result_direction, Direction::NORTH);
    }
}
