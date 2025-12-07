use core::fmt;
use std::fmt::Write;

use crate::{coordinate::Coordinate, direction::Direction, Visited};

#[derive(Clone, Default)]
pub struct Grid<T> {
    data: Vec<T>,
    row_len: usize,
    row_count: usize,
}

impl<T> Grid<T> {
    pub fn new() -> Self
    where
        T: Default,
    {
        Self::default()
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        self.row_len
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        self.row_count
    }

    pub fn push_row(&mut self, new_row: Vec<T>) {
        if self.data.is_empty() {
            self.row_len = new_row.len();
        }

        assert!(self.row_len == new_row.len());
        self.row_count += 1;
        self.data.extend(new_row);
    }

    #[inline]
    pub fn get(&self, coord: Coordinate) -> &T {
        &self.data[coord.y * self.row_len + coord.x]
    }

    #[inline]
    pub fn set(&mut self, coord: Coordinate, value: T) {
        self.data[coord.y * self.row_len + coord.x] = value;
    }

    #[inline]
    pub fn get_all_neighbours(
        &self,
        location: Coordinate,
    ) -> impl Iterator<Item = Neighbour<'_, T>> {
        self.get_neighbours(location, Direction::get_all())
    }

    #[inline]
    pub fn get_cardinal_neighbours(
        &self,
        location: Coordinate,
    ) -> impl Iterator<Item = Neighbour<'_, T>> {
        self.get_neighbours(location, Direction::get_cardinals())
    }

    #[inline]
    pub fn get_neighbours<'this>(
        &'this self,
        location: Coordinate,
        directions: impl IntoIterator<Item = Direction> + 'this,
    ) -> impl Iterator<Item = Neighbour<'this, T>> + 'this {
        directions
            .into_iter()
            .filter_map(move |direction| self.get_neighbour(location, direction))
    }

    pub fn get_neighbour(
        &self,
        location: Coordinate,
        direction: Direction,
    ) -> Option<Neighbour<'_, T>> {
        let new_x = match direction {
            Direction::East | Direction::NorthEast | Direction::SouthEast => {
                if location.x + 1 < self.row_len {
                    location.x + 1
                } else {
                    return None;
                }
            }
            Direction::West | Direction::NorthWest | Direction::SouthWest => {
                if location.x != 0 {
                    location.x - 1
                } else {
                    return None;
                }
            }
            _ => location.x,
        };

        let new_y = match direction {
            Direction::North | Direction::NorthEast | Direction::NorthWest => {
                if location.y != 0 {
                    location.y - 1
                } else {
                    return None;
                }
            }
            Direction::South | Direction::SouthEast | Direction::SouthWest => {
                if location.y + 1 < self.row_count {
                    location.y + 1
                } else {
                    return None;
                }
            }
            _ => location.y,
        };
        let neighbour_coord = Coordinate { x: new_x, y: new_y };

        Some(Neighbour {
            value: self.get(neighbour_coord),
            location: neighbour_coord,
            direction,
        })
    }

    #[inline]
    pub fn is_valid_coord(&self, coord: Coordinate) -> bool {
        coord.x < self.row_len && coord.y < self.row_count
    }

    #[inline]
    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coordinate, &T)> + Clone {
        self.data.iter().enumerate().map(move |(idx, i)| {
            let position = self.index_to_coord(idx);
            (position, i)
        })
    }

    #[inline]
    pub fn find_item(&self, input: &T) -> Option<Coordinate>
    where
        T: PartialEq,
    {
        self.data
            .iter()
            .position(|x| x == input)
            .map(|index| self.index_to_coord(index))
    }

    #[inline]
    pub fn find_all<'self_>(
        &'self_ self,
        input: &'self_ T,
    ) -> impl Iterator<Item = Coordinate> + 'self_
    where
        T: PartialEq,
    {
        self.indexed_iter()
            .filter(move |x| x.1 == input)
            .map(|x| x.0)
    }

    #[inline]
    pub fn find_all_by<'self_, F>(
        &'self_ self,
        mut function: F,
    ) -> impl Iterator<Item = Coordinate> + 'self_
    where
        T: PartialEq,
        F: FnMut(&T) -> bool + 'self_,
    {
        self.indexed_iter()
            .filter(move |(_coord, t)| function(t))
            .map(|x| x.0)
    }

    #[inline]
    fn index_to_coord(&self, index: usize) -> Coordinate {
        Coordinate {
            x: index % self.row_len,
            y: index / self.row_len,
        }
    }

    #[inline]
    pub fn are_locations_equal(&self, first: Coordinate, second: Coordinate) -> bool
    where
        T: PartialEq,
    {
        self.get(first) == self.get(second)
    }

    #[inline]
    pub fn matches_neighbour(&self, location: Coordinate, direction: Direction) -> bool
    where
        T: PartialEq,
    {
        if let Some(neighbour) = self.get_neighbour(location, direction) {
            return neighbour.value == self.get(location);
        }

        false
    }

    #[inline]
    pub fn get_row(&self, row_index: usize) -> impl Iterator<Item = Coordinate> + '_ {
        //The row starts at row_index*row_length and ends at row_index+1*row_length
        (row_index * self.row_len..(row_index + 1) * self.row_len).map(|x| self.index_to_coord(x))
    }

    #[inline]
    pub fn get_column(&self, column_index: usize) -> impl Iterator<Item = Coordinate> + '_ {
        (column_index..self.data.len())
            .step_by(self.row_len)
            .map(|x| self.index_to_coord(x))
    }

    #[inline]
    pub fn view_from<'a>(
        &'a self,
        coord: &Coordinate,
        direction: Direction,
    ) -> ViewFromIterator<'a, T> {
        ViewFromIterator {
            current_coord: *coord,
            grid: &self,
            direction,
        }
    }

    pub fn create_visited_list(&self) -> Visited {
        Visited::new(self.row_len, self.row_count)
    }

    pub fn is_on_edge(&self, coord: &Coordinate) -> bool {
        coord.x == 0 || coord.y == 0 || coord.x == self.row_len - 1 || coord.y == self.row_count - 1
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = "".to_string();
        for i in 0..self.data.len() {
            if i != 0 && i % self.row_len == 0 {
                output.push('\n');
            }

            output.push_str(&self.data[i].to_string());
        }

        write!(f, "{}", output)
    }
}

impl<T> fmt::Debug for Grid<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        for i in 0..self.data.len() {
            if i != 0 && i % self.row_len == 0 {
                output.push('\n');
            }
            _ = write!(output, "{:?}", &self.data[i]);
            //output.push_str(&self.data[i]);
        }

        write!(f, "{}", output)
    }
}

#[derive(Clone)]
pub struct Neighbour<'a, T> {
    pub value: &'a T,
    pub location: Coordinate,
    pub direction: Direction,
}

pub struct ViewFromIterator<'grid, T> {
    current_coord: Coordinate,
    grid: &'grid Grid<T>,
    direction: Direction,
}

impl<'a, T> Iterator for ViewFromIterator<'a, T> {
    type Item = Neighbour<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbour = self
            .grid
            .get_neighbour(self.current_coord, self.direction)?;
        self.current_coord = neighbour.location;
        Some(neighbour)
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::coordinate::Coordinate;

    use super::*;
    const TESTINPUT: [[i32; 4]; 4] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];

    fn get_test_grid() -> Grid<i32> {
        let mut grid = Grid::new();

        for array in TESTINPUT {
            grid.push_row(array.to_vec());
        }

        grid
    }

    #[test]
    fn is_valid_coord_test() {
        let grid = get_test_grid();

        // Min x and y
        let mut coord = Coordinate { x: 0, y: 0 };
        assert!(grid.is_valid_coord(coord));

        // Valid, on edges
        coord = Coordinate { x: 2, y: 1 };
        assert!(grid.is_valid_coord(coord));

        // Bottom right corner
        coord = Coordinate { x: 3, y: 3 };
        assert!(grid.is_valid_coord(coord));

        // x out of bounds
        coord = Coordinate { x: 4, y: 3 };
        assert!(!grid.is_valid_coord(coord));

        // y out of bounds
        coord = Coordinate { x: 3, y: 4 };
        assert!(!grid.is_valid_coord(coord));

        // Both out of bounds
        coord = Coordinate { x: 4, y: 4 };
        assert!(!grid.is_valid_coord(coord));

        // Both very out of bounds
        coord = Coordinate { x: 412, y: 561 };
        assert!(!grid.is_valid_coord(coord));
    }

    #[test]
    fn get_row_test() {
        let grid = get_test_grid();
        let row: Vec<Coordinate> = grid.get_row(2).collect();

        assert_eq!(row.len(), 4);
        assert_eq!(row[0], Coordinate::new(0, 2));
        assert_eq!(row[1], Coordinate::new(1, 2));
        assert_eq!(row[2], Coordinate::new(2, 2));
        assert_eq!(row[3], Coordinate::new(3, 2));
    }

    #[test]
    fn get_column_test() {
        let grid = get_test_grid();
        let column: Vec<Coordinate> = grid.get_column(2).collect();

        assert_eq!(column.len(), 4);
        assert_eq!(column[0], Coordinate::new(2, 0));
        assert_eq!(column[1], Coordinate::new(2, 1));
        assert_eq!(column[2], Coordinate::new(2, 2));
        assert_eq!(column[3], Coordinate::new(2, 3));
    }

    #[test]
    fn get_view_test_north() {
        let grid = get_test_grid();
        let view: Vec<_> = grid
            .view_from(&Coordinate::new(2, 2), Direction::North)
            .collect();
        assert_eq!(view.len(), 2);
        assert_eq!(view[0].value, &7);
        assert_eq!(view[1].value, &3);
    }

    #[test]
    fn get_view_test_southeast() {
        let grid = get_test_grid();
        let view: Vec<_> = grid
            .view_from(&Coordinate::new(1, 0), Direction::SouthEast)
            .collect();
        assert_eq!(view.len(), 2);
        assert_eq!(view[0].value, &7);
        assert_eq!(view[1].value, &12);
    }
}
