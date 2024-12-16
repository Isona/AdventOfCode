use core::fmt;
use std::ops::Mul;

use crate::vector::Vector;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }

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

impl Mul for Coordinate {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        Self::new(x, y)
    }
}

impl Mul<usize> for Coordinate {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self::new(x, y)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
