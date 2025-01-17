use bitvec::prelude::*;

use crate::Coordinate;

pub struct Visited {
    coord_map: BitVec,
    row_len: usize,
}

impl Visited {
    pub fn new(row_len: usize, row_count: usize) -> Visited {
        Self {
            coord_map: bitvec![0; row_len*row_count],
            row_len,
        }
    }

    pub fn is_visited(&self, coordinate: &Coordinate) -> bool {
        self.coord_map[coordinate.y * self.row_len + coordinate.x]
    }

    pub fn set(&mut self, coordinate: &Coordinate, value: bool) -> bool {
        let index = coordinate.y * self.row_len + coordinate.x;
        let original = *self.coord_map.get(index).unwrap();
        self.coord_map.set(index, value);
        original
    }
}
