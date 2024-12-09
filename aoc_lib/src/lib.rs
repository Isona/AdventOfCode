use std::vec;

#[derive(Clone)]
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
        Self {
            data: vec![],
            row_len: 0,
            row_count: 0,
        }
    }

    pub fn push_row(&mut self, new_row: Vec<T>) {
        if self.data.is_empty() {
            self.row_len = new_row.len();
        }

        assert!(self.row_len == new_row.len());
        self.row_count += 1;
        self.data.extend(new_row);
    }

    pub fn get(&self, coord: Coordinate) -> &T {
        &self.data[coord.y * self.row_count + coord.x]
    }

    pub fn set(&mut self, coord: Coordinate, value: T) {
        self.data[coord.y * self.row_count + coord.x] = value;
    }

    pub fn get_all_neighbours(&self, location: Coordinate) -> Vec<Neighbour<T>> {
        self.get_neighbours(&location, &Direction::get_all())
    }

    pub fn get_cardinal_neighbours(&self, location: Coordinate) -> Vec<Neighbour<T>> {
        self.get_neighbours(&location, &Direction::get_cardinals())
    }

    pub fn get_neighbours(
        &self,
        location: &Coordinate,
        directions: &[Direction],
    ) -> Vec<Neighbour<T>> {
        let mut neighbours = Vec::new();
        for direction in directions {
            if let Some(neighbour) = self.get_neighbour(location, direction) {
                neighbours.push(neighbour);
            }
        }

        neighbours
    }

    pub fn get_neighbour(
        &self,
        location: &Coordinate,
        direction: &Direction,
    ) -> Option<Neighbour<T>> {
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
            direction: *direction,
        })
    }

    pub fn is_valid_coord(&self, coord: Coordinate) -> bool {
        coord.x < self.row_len && coord.y < self.data.len()
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.data.iter().enumerate().map(move |(idx, i)| {
            let position = self.index_to_coord(idx);
            (position, i)
        })
    }

    pub fn find_item(&self, input: &T) -> Option<Coordinate>
    where
        T: Eq,
    {
        self.data
            .iter()
            .position(|x| x == input)
            .map(|index| self.index_to_coord(index))
    }

    pub fn find_all(&self, input: &T) -> Vec<Coordinate>
    where
        T: Eq,
    {
        self.indexed_iter()
            .filter(|x| x.1 == input)
            .map(|x| x.0)
            .collect()
    }

    fn index_to_coord(&self, index: usize) -> Coordinate {
        Coordinate {
            x: index % self.row_len,
            y: index / self.row_len,
        }
    }
}

pub struct Neighbour<'a, T> {
    pub value: &'a T,
    pub location: Coordinate,
    pub direction: Direction,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn get_cardinals() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    pub fn get_intercardinals() -> Vec<Direction> {
        vec![
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ]
    }

    pub fn get_all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ]
    }

    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::NorthEast => Direction::SouthEast,
            Direction::NorthWest => Direction::NorthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::NorthEast => Direction::NorthWest,
            Direction::NorthWest => Direction::SouthWest,
            Direction::SouthEast => Direction::NorthEast,
            Direction::SouthWest => Direction::SouthEast,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn get_distance(&self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn add_vec(&self, vector: &Vector) -> Option<Coordinate> {
        let x = self.x.checked_add_signed(vector.x.try_into().unwrap())?;
        let y = self.y.checked_add_signed(vector.y.try_into().unwrap())?;

        Some(Coordinate { x, y })
    }

    pub fn sub_vec(&self, vector: &Vector) -> Option<Coordinate> {
        let x = self
            .x
            .checked_add_signed(vector.x.checked_neg().unwrap().try_into().unwrap())?;
        let y = self
            .y
            .checked_add_signed(vector.y.checked_neg().unwrap().try_into().unwrap())?;

        Some(Coordinate { x, y })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Vector {
    x: i128,
    y: i128,
}

impl Vector {
    pub fn get_difference(a: Coordinate, b: Coordinate) -> Self {
        Vector {
            x: b.x as i128 - a.x as i128,
            y: b.y as i128 - a.y as i128,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_get_difference() {
        let coord_1: Coordinate = Coordinate { x: 12, y: 61 };
        let coord_2: Coordinate = Coordinate { x: 612, y: 18 };
        let vector = Vector::get_difference(coord_1, coord_2);
        assert_eq!(vector, Vector { x: 600, y: -43 });
    }

    #[test]
    fn vector_add_success() {
        let coord: Coordinate = Coordinate { x: 12, y: 61 };
        let vec: Vector = Vector { x: -11, y: -11 };
        assert_eq!(coord.add_vec(&vec).unwrap(), Coordinate { x: 1, y: 50 });
    }

    #[test]
    fn vector_add_failure() {
        let coord: Coordinate = Coordinate { x: 12, y: 61 };
        let vec: Vector = Vector { x: -13, y: -11 };
        assert_eq!(coord.add_vec(&vec), None);
    }

    #[test]
    fn vector_sub_success() {
        let coord: Coordinate = Coordinate { x: 12, y: 61 };
        let vec: Vector = Vector { x: 11, y: 11 };
        assert_eq!(coord.sub_vec(&vec).unwrap(), Coordinate { x: 1, y: 50 });
    }

    #[test]
    fn vector_sub_failure() {
        let coord: Coordinate = Coordinate { x: 12, y: 61 };
        let vec: Vector = Vector { x: 13, y: 11 };
        assert_eq!(coord.sub_vec(&vec), None);
    }
}
