use crate::coordinate::Coordinate;
use core::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default)]
pub struct Vector {
    pub x: i128,
    pub y: i128,
}

impl Vector {
    pub fn new(x: i128, y: i128) -> Self {
        Vector { x, y }
    }

    pub fn get_difference(a: &Coordinate, b: &Coordinate) -> Self {
        Vector {
            x: b.x as i128 - a.x as i128,
            y: b.y as i128 - a.y as i128,
        }
    }
}

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        Self::new(x, y)
    }
}

impl Mul<i128> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i128) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self::new(x, y)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self::new(x, y)
    }
}

impl Add<i128> for Vector {
    type Output = Vector;
    fn add(self, rhs: i128) -> Self::Output {
        let x = self.x + rhs;
        let y = self.y + rhs;
        Self::new(x, y)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self::new(x, y)
    }
}

impl Sub<i128> for Vector {
    type Output = Vector;
    fn sub(self, rhs: i128) -> Self::Output {
        let x = self.x - rhs;
        let y = self.y - rhs;
        Self::new(x, y)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_get_difference() {
        let coord_1: Coordinate = Coordinate { x: 12, y: 61 };
        let coord_2: Coordinate = Coordinate { x: 612, y: 18 };
        let vector = Vector::get_difference(&coord_1, &coord_2);
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
