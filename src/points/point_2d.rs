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

    #[allow(unused)]
    pub fn distance(&self, point: Self) -> usize {
        let x = T::to_isize(&(point.row - self.row)).unwrap();
        let y = T::to_isize(&(point.col - self.col)).unwrap();

        // isize * isize will always be positive
        #[allow(clippy::cast_sign_loss)]
        let dist = (x * x) as usize + (y * y) as usize;
        dist.isqrt()
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

fn try_from_internal<S, N>(value: S) -> Result<Point2D<N>, &'static str>
where
    S: AsRef<str>,
    N: num_traits::Signed + num_traits::PrimInt + std::str::FromStr,
{
    let (lhs, rhs) = value
        .as_ref()
        .split_once(',')
        .ok_or("Input did not contain ','")?;
    let row = lhs.parse::<N>().map_err(|_| "Could not parse Lhs as i16")?;
    let col = rhs.parse::<N>().map_err(|_| "Could not parse Rhs as i16")?;
    Ok(Point2D { row, col })
}

impl TryFrom<&str> for Point2D<i16> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<String> for Point2D<i16> {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<&str> for Point2D<i32> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<String> for Point2D<i32> {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<&str> for Point2D<i64> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<String> for Point2D<i64> {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}
