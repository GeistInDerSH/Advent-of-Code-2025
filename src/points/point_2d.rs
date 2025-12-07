use std::ops::{Add, AddAssign};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Point2D<T>
where
    T: num_traits::Signed + num_traits::PrimInt,
{
    pub row: T,
    pub col: T,
}

impl<T> Point2D<T>
where
    T: num_traits::Signed + num_traits::NumCast + num_traits::PrimInt + Copy,
{
    pub fn neighbors(&self) -> [Point2D<T>; 9] {
        let mut neighbors = [Point2D::default(); 9];

        let mut index = 0;
        for row_inc in -1..=1 {
            let row = self.row + T::from(row_inc).unwrap();
            neighbors[index] = Point2D {
                row,
                col: self.col - T::one(),
            };
            neighbors[index + 1] = Point2D { row, col: self.col };
            neighbors[index + 2] = Point2D {
                row,
                col: self.col + T::one(),
            };
            index += 3;
        }

        neighbors
    }
}

impl<T> Add for Point2D<T>
where
    T: num_traits::Signed + num_traits::NumCast + num_traits::PrimInt + Copy,
{
    type Output = Point2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl<T> AddAssign for Point2D<T>
where
    T: num_traits::Signed + num_traits::NumCast + num_traits::PrimInt + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.row = self.row + rhs.row;
        self.col = self.col + rhs.col;
    }
}

impl<T> Default for Point2D<T>
where
    T: num_traits::Signed + num_traits::NumCast + num_traits::PrimInt + Copy,
{
    fn default() -> Self {
        Self {
            row: T::zero(),
            col: T::zero(),
        }
    }
}

#[allow(unused)]
impl Point2D<i16> {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row: i16::try_from(row).unwrap(),
            col: i16::try_from(col).unwrap(),
        }
    }
}

#[allow(unused)]
impl Point2D<i32> {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row: i32::try_from(row).unwrap(),
            col: i32::try_from(col).unwrap(),
        }
    }
}

#[allow(unused)]
impl Point2D<i64> {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row: i64::try_from(row).unwrap(),
            col: i64::try_from(col).unwrap(),
        }
    }
}
