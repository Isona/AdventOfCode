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

    pub fn get(&self, coord: Coordinate) -> &T {
        &self.data[coord.y * self.row_count + coord.x]
    }

    pub fn get_all_neighbour_coords(&self, location: Coordinate) -> Vec<Coordinate> {
        let mut neighbours = self.get_cardinal_neighbour_coords(location);

        let intercardinals = vec![
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ];

        for direction in intercardinals {
            if let Some(neighbour) = self.get_neighbour(location, direction) {
                neighbours.push(neighbour)
            }
        }

        neighbours
    }

    pub fn get_cardinal_neighbour_coords(&self, location: Coordinate) -> Vec<Coordinate> {
        let mut neighbours = vec![];
        let cardinals = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        for direction in cardinals {
            if let Some(neighbour) = self.get_neighbour(location, direction) {
                neighbours.push(neighbour);
            }
        }

        neighbours
    }

    pub fn get_all_neighbour_coords_with_direction(
        &self,
        location: Coordinate,
    ) -> Vec<(Coordinate, Direction)> {
        let mut neighbours = self.get_cardinal_neighbour_coords_with_direction(location);

        let intercardinals = vec![
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ];

        for direction in intercardinals {
            if let Some(neighbour) = self.get_neighbour(location, direction) {
                neighbours.push((neighbour, direction));
            }
        }

        neighbours
    }

    pub fn get_cardinal_neighbour_coords_with_direction(
        &self,
        location: Coordinate,
    ) -> Vec<(Coordinate, Direction)> {
        let mut neighbours = vec![];
        let cardinals = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        for direction in cardinals {
            if let Some(neighbour) = self.get_neighbour(location, direction) {
                neighbours.push((neighbour, direction));
            }
        }

        neighbours
    }

    pub fn push_row(&mut self, new_row: Vec<T>) {
        if self.data.is_empty() {
            self.row_len = new_row.len();
        }

        assert!(self.row_len == new_row.len());
        self.row_count += 1;
        self.data.extend(new_row);
    }

    pub fn get_neighbour(&self, location: Coordinate, direction: Direction) -> Option<Coordinate> {
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

        Some(Coordinate { x: new_x, y: new_y })
    }

    pub fn is_valid_coord(&self, coord: Coordinate) -> bool {
        coord.x < self.row_len && coord.y < self.data.len()
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.data.iter().enumerate().map(move |(idx, i)| {
            let position = Coordinate {
                x: idx % self.row_len,
                y: idx / self.row_len,
            };
            (position, i)
        })
    }

    pub fn find_item(&self, input: &T) -> Option<Coordinate>
    where
        T: Eq,
    {
        for (coord, value) in self.indexed_iter() {
            if value == input {
                return Some(coord);
            }
        }

        return None;
    }
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn get_distance(&self, other: Coordinate) -> usize {
        return &self.x.abs_diff(other.x) + &self.y.abs_diff(other.y);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
