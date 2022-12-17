use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinate {
    pub x: usize,
    pub y: usize,
}

impl GridCoordinate {
    pub fn new(x: usize, y: usize) -> GridCoordinate {
        return GridCoordinate { x: x, y: y };
    }
}

impl Display for GridCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl std::ops::Add for GridCoordinate {
    type Output = GridCoordinate;

    fn add(self, other: GridCoordinate) -> GridCoordinate {
        return GridCoordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Ord for GridCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.x.cmp(&self.x).then_with(|| other.y.cmp(&self.y));
    }
}

impl PartialOrd for GridCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinateInf {
    pub x: i32,
    pub y: i32,
}

impl GridCoordinateInf {
    pub fn new(x: i32, y: i32) -> GridCoordinateInf {
        return GridCoordinateInf { x: x, y: y };
    }

    pub fn move_dir(&self, direction: Direction) -> GridCoordinateInf {
        let north_move = GridCoordinateInf::new(0, -1);
        let south_move = GridCoordinateInf::new(0, 1);
        let west_move = GridCoordinateInf::new(-1, 0);
        let east_move = GridCoordinateInf::new(1, 0);

        return *self
            + match direction {
                Direction::NORTH => north_move,
                Direction::EAST => east_move,
                Direction::SOUTH => south_move,
                Direction::WEST => west_move,
                Direction::NORTHEAST => north_move + east_move,
                Direction::NORTHWEST => north_move + west_move,
                Direction::SOUTHEAST => south_move + east_move,
                Direction::SOUTHWEST => south_move + west_move,
            };
    }
}

impl Display for GridCoordinateInf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl std::ops::Add for GridCoordinateInf {
    type Output = GridCoordinateInf;

    fn add(self, other: GridCoordinateInf) -> GridCoordinateInf {
        return GridCoordinateInf {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinateInf64 {
    pub x: i64,
    pub y: i64,
}

impl GridCoordinateInf64 {
    pub fn new(x: i64, y: i64) -> GridCoordinateInf64 {
        return GridCoordinateInf64 { x: x, y: y };
    }

    pub fn move_dir(&self, direction: Direction) -> GridCoordinateInf64 {
        let north_move = GridCoordinateInf64::new(0, -1);
        let south_move = GridCoordinateInf64::new(0, 1);
        let west_move = GridCoordinateInf64::new(-1, 0);
        let east_move = GridCoordinateInf64::new(1, 0);

        return *self
            + match direction {
                Direction::NORTH => north_move,
                Direction::EAST => east_move,
                Direction::SOUTH => south_move,
                Direction::WEST => west_move,
                Direction::NORTHEAST => north_move + east_move,
                Direction::NORTHWEST => north_move + west_move,
                Direction::SOUTHEAST => south_move + east_move,
                Direction::SOUTHWEST => south_move + west_move,
            };
    }
}

impl Display for GridCoordinateInf64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl std::ops::Add for GridCoordinateInf64 {
    type Output = GridCoordinateInf64;

    fn add(self, other: GridCoordinateInf64) -> GridCoordinateInf64 {
        return GridCoordinateInf64 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug)]
pub struct Grid<T: Copy> {
    /* Variable sized Grid.
     *
     * width * height = grid_numbers.len()
     * index by: x + (y * width)
     * essentially top left corner is 0,0, right and down increases.
     */
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, values: Vec<T>) -> Grid<T> {
        assert_eq!(width * height, values.len());
        return Grid {
            width: width,
            height: height,
            values: values,
        };
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn coord_iter(&self) -> GridIter {
        return GridIter {
            cur_x: 0,
            cur_y: 0,
            max_x: self.width,
            max_y: self.height,
            first: true,
        };
    }

    pub fn data_copy(&self) -> Vec<T>
    where
        T: Clone,
    {
        return self.values.clone();
    }
}

pub struct GridIter {
    cur_x: usize,
    cur_y: usize,
    max_x: usize,
    max_y: usize,
    first: bool,
}

impl Iterator for GridIter {
    type Item = GridCoordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(GridCoordinate::new(self.cur_x, self.cur_y));
        }
        self.cur_x += 1;
        if self.cur_x >= self.max_x {
            self.cur_x = self.cur_x % self.max_x;
            self.cur_y += 1;
        }
        if self.cur_y >= self.max_y {
            return None;
        } else {
            return Some(GridCoordinate::new(self.cur_x, self.cur_y));
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NORTHEAST,
    SOUTHEAST,
    SOUTHWEST,
    NORTHWEST,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::NORTH => "NORTH",
            Direction::EAST => "EAST",
            Direction::SOUTH => "SOUTH",
            Direction::WEST => "WEST",
            Direction::NORTHEAST => "NORTHEAST",
            Direction::SOUTHEAST => "SOUTHEAST",
            Direction::SOUTHWEST => "SOUTHWEST",
            Direction::NORTHWEST => "NORTHWEST",
        };
        return write!(f, "{}", s);
    }
}

pub trait GridTraversable {
    type Item;

    fn get_value(&self, pos: GridCoordinate) -> Option<Self::Item>;
    fn set_value(&mut self, pos: GridCoordinate, value: Self::Item);
    fn get_coordinate_by_direction(
        &self,
        pos: GridCoordinate,
        direction: Direction,
    ) -> Option<GridCoordinate>;
    fn get_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate>;
    fn get_diag_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate>;
}

impl<T: Copy> GridTraversable for Grid<T> {
    type Item = T;

    fn get_value(&self, pos: GridCoordinate) -> Option<Self::Item> {
        if pos.y >= self.height || pos.x >= self.width {
            // y cannot exceed height, x cannot exceed width
            return None;
        }
        let pos: usize = pos.x + pos.y * self.width;
        return Some(*(self.values.iter().nth(pos)?));
    }

    fn set_value(&mut self, pos: GridCoordinate, value: Self::Item) {
        if pos.y >= self.height || pos.x >= self.width {
            // y cannot exceed height, x cannot exceed width
            return;
        }
        let pos: usize = pos.x + pos.y * self.width;
        self.values[pos] = value;
    }

    fn get_coordinate_by_direction(
        &self,
        pos: GridCoordinate,
        direction: Direction,
    ) -> Option<GridCoordinate> {
        let mut possible_y: Option<usize> = Some(pos.y);
        let mut possible_x: Option<usize> = Some(pos.x);
        match direction {
            Direction::NORTH => possible_y = pos.y.checked_sub(1),
            Direction::EAST => possible_x = pos.x.checked_add(1),
            Direction::SOUTH => possible_y = pos.y.checked_add(1),
            Direction::WEST => possible_x = pos.x.checked_sub(1),
            Direction::NORTHEAST => {
                possible_x = pos.x.checked_add(1);
                possible_y = pos.y.checked_sub(1);
            }
            Direction::SOUTHEAST => {
                possible_x = pos.x.checked_add(1);
                possible_y = pos.y.checked_add(1);
            }
            Direction::SOUTHWEST => {
                possible_x = pos.x.checked_sub(1);
                possible_y = pos.y.checked_add(1);
            }
            Direction::NORTHWEST => {
                possible_x = pos.x.checked_sub(1);
                possible_y = pos.y.checked_sub(1);
            }
        }
        if let Some(new_x) = possible_x {
            if let Some(new_y) = possible_y {
                if new_x > self.width - 1 || new_y > self.height - 1 {
                    return None;
                }
                return Some(GridCoordinate::new(new_x, new_y));
            }
        }
        return None;
    }

    fn get_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate> {
        let opt_north = self.get_coordinate_by_direction(pos, Direction::NORTH);
        let opt_east = self.get_coordinate_by_direction(pos, Direction::EAST);
        let opt_south = self.get_coordinate_by_direction(pos, Direction::SOUTH);
        let opt_west = self.get_coordinate_by_direction(pos, Direction::WEST);
        let mut result: Vec<GridCoordinate> = Vec::new();
        let options = vec![opt_north, opt_east, opt_south, opt_west];

        for possible_pos in options {
            if let Some(cur_pos) = possible_pos {
                result.push(cur_pos);
            }
        }

        return result;
    }

    fn get_diag_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate> {
        let opt_north_east = self.get_coordinate_by_direction(pos, Direction::NORTHEAST);
        let opt_south_east = self.get_coordinate_by_direction(pos, Direction::SOUTHEAST);
        let opt_south_west = self.get_coordinate_by_direction(pos, Direction::SOUTHWEST);
        let opt_north_west = self.get_coordinate_by_direction(pos, Direction::NORTHWEST);
        let mut result: Vec<GridCoordinate> = Vec::new();
        let options = vec![
            opt_north_east,
            opt_south_east,
            opt_south_west,
            opt_north_west,
        ];

        for possible_pos in options {
            if let Some(cur_pos) = possible_pos {
                result.push(cur_pos);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_make_bad_grid() {
        let nums = vec![1, 2];
        let height = 9;
        let width = 23;
        Grid::new(width, height, nums);
    }

    fn produce_grid() -> Grid<i32> {
        let grid_nums = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let grid: Grid<i32> = Grid::new(10, 5, grid_nums);
        return grid;
    }

    #[test]
    fn test_get_grid_number() {
        let grid = produce_grid();
        assert_eq!(grid.get_value(GridCoordinate::new(0, 0)), Some(2));
        assert_eq!(grid.get_value(GridCoordinate::new(9, 0)), Some(0));
        assert_eq!(grid.get_value(GridCoordinate::new(0, 4)), Some(9));
        assert_eq!(grid.get_value(GridCoordinate::new(9, 4)), Some(8));
        assert_eq!(grid.get_value(GridCoordinate::new(4, 2)), Some(7));
        assert_eq!(grid.get_value(GridCoordinate::new(5, 2)), Some(8));
    }

    #[test]
    fn test_set_grid_number() {
        let mut grid = produce_grid();
        let coord = GridCoordinate::new(3, 3);
        assert_eq!(grid.get_value(coord), Some(7));
        grid.set_value(coord, 99);
        assert_eq!(grid.get_value(coord), Some(99));
    }

    #[test]
    fn test_get_adjacent_coordinates() {
        let grid = produce_grid();
        assert_eq!(
            grid.get_adjacent_coordinates(GridCoordinate::new(0, 0)),
            vec![GridCoordinate::new(1, 0), GridCoordinate::new(0, 1)]
        );
        assert_eq!(
            grid.get_adjacent_coordinates(GridCoordinate::new(9, 0)),
            vec![GridCoordinate::new(9, 1), GridCoordinate::new(8, 0)]
        );
        assert_eq!(
            grid.get_adjacent_coordinates(GridCoordinate::new(0, 4)),
            vec![GridCoordinate::new(0, 3), GridCoordinate::new(1, 4)]
        );
        assert_eq!(
            grid.get_adjacent_coordinates(GridCoordinate::new(9, 4)),
            vec![GridCoordinate::new(9, 3), GridCoordinate::new(8, 4)]
        );
    }

    #[test]
    fn test_get_diag_adjacent_coordinates() {
        let grid = produce_grid();
        assert_eq!(
            grid.get_diag_adjacent_coordinates(GridCoordinate::new(0, 0)),
            vec![GridCoordinate::new(1, 1)]
        );
        assert_eq!(
            grid.get_diag_adjacent_coordinates(GridCoordinate::new(9, 0)),
            vec![GridCoordinate::new(8, 1)]
        );
        assert_eq!(
            grid.get_diag_adjacent_coordinates(GridCoordinate::new(0, 4)),
            vec![GridCoordinate::new(1, 3)]
        );
        assert_eq!(
            grid.get_diag_adjacent_coordinates(GridCoordinate::new(9, 4)),
            vec![GridCoordinate::new(8, 3)]
        );
    }

    #[test]
    fn test_get_width() {
        let grid = produce_grid();
        assert_eq!(grid.get_width(), 10);
    }

    #[test]
    fn test_get_height() {
        let grid = produce_grid();
        assert_eq!(grid.get_height(), 5);
    }

    #[test]
    fn test_coord_iter() {
        let grid = produce_grid();
        let mut iter = grid.coord_iter();
        if let Some(first_v) = iter.next() {
            assert_eq!(first_v, GridCoordinate::new(0, 0));
        } else {
            panic!("No first value found");
        }

        if let Some(second_v) = iter.next() {
            assert_eq!(second_v, GridCoordinate::new(1, 0));
        } else {
            panic!("No second value found");
        }

        let all: Vec<GridCoordinate> = grid.coord_iter().collect();
        assert_eq!(all.len(), 50);
    }

    #[test]
    fn test_add_coords() {
        let a = GridCoordinate::new(3, 5);
        let b = GridCoordinate::new(7, 11);
        let expected = GridCoordinate::new(10, 16);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_move_on_infinite_grid() {
        let start = GridCoordinateInf::new(0, 0);
        let mut cur = start.move_dir(Direction::NORTH);
        assert_eq!(cur, GridCoordinateInf::new(0, -1));
        cur = cur.move_dir(Direction::WEST);
        assert_eq!(cur, GridCoordinateInf::new(-1, -1));
        cur = cur.move_dir(Direction::NORTHWEST);
        assert_eq!(cur, GridCoordinateInf::new(-2, -2));
        cur = cur.move_dir(Direction::NORTHEAST);
        assert_eq!(cur, GridCoordinateInf::new(-1, -3));
        cur = cur.move_dir(Direction::EAST);
        assert_eq!(cur, GridCoordinateInf::new(-0, -3));
        cur = cur.move_dir(Direction::SOUTH);
        assert_eq!(cur, GridCoordinateInf::new(0, -2));
        cur = cur.move_dir(Direction::SOUTHEAST);
        assert_eq!(cur, GridCoordinateInf::new(1, -1));
        cur = cur.move_dir(Direction::SOUTHWEST);
        assert_eq!(cur, GridCoordinateInf::new(0, 0));
    }

    #[test]
    fn test_move_on_infinite_grid64() {
        let start = GridCoordinateInf64::new(0, 0);
        let mut cur = start.move_dir(Direction::NORTH);
        assert_eq!(cur, GridCoordinateInf64::new(0, -1));
        cur = cur.move_dir(Direction::WEST);
        assert_eq!(cur, GridCoordinateInf64::new(-1, -1));
        cur = cur.move_dir(Direction::NORTHWEST);
        assert_eq!(cur, GridCoordinateInf64::new(-2, -2));
        cur = cur.move_dir(Direction::NORTHEAST);
        assert_eq!(cur, GridCoordinateInf64::new(-1, -3));
        cur = cur.move_dir(Direction::EAST);
        assert_eq!(cur, GridCoordinateInf64::new(-0, -3));
        cur = cur.move_dir(Direction::SOUTH);
        assert_eq!(cur, GridCoordinateInf64::new(0, -2));
        cur = cur.move_dir(Direction::SOUTHEAST);
        assert_eq!(cur, GridCoordinateInf64::new(1, -1));
        cur = cur.move_dir(Direction::SOUTHWEST);
        assert_eq!(cur, GridCoordinateInf64::new(0, 0));
    }
}
