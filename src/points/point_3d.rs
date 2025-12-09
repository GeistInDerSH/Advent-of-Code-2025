use std::ops::{Add, AddAssign};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Point3D<T>
where
    T: num_traits::Signed + num_traits::PrimInt,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3D<T>
where
    T: num_traits::Signed + num_traits::NumCast + num_traits::PrimInt + Copy,
{
    #[allow(unused)]
    pub fn neighbors(&self) -> [Point3D<T>; 27] {
        let mut neighbors = [Point3D::default(); 27];

        let mut index = 0;
        for x_inc in -1..=1 {
            let x = self.x + T::from(x_inc).unwrap();
            for y_inc in -1..=1 {
                let y = self.y + T::from(y_inc).unwrap();
                for z_inc in -1..=1 {
                    neighbors[index] = Point3D {
                        x,
                        y,
                        z: self.z - T::from(z_inc).unwrap(),
                    };
                    index += 1;
                }
            }
        }

        neighbors
    }

    pub fn distance(&self, point: Point3D<T>) -> usize {
        let x = T::to_isize(&(point.x - self.x)).unwrap();
        let y = T::to_isize(&(point.y - self.y)).unwrap();
        let z = T::to_isize(&(point.z - self.z)).unwrap();

        // isize * isize will always be positive
        #[allow(clippy::cast_sign_loss)]
        let dist = (x * x) as usize + (y * y) as usize + (z * z) as usize;
        dist.isqrt()
    }
}

impl<T> Add for Point3D<T>
where
    T: num_traits::Signed + num_traits::NumCast + num_traits::PrimInt + Copy,
{
    type Output = Point3D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign for Point3D<T>
where
    T: num_traits::Signed + num_traits::NumCast + num_traits::PrimInt + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<T> Default for Point3D<T>
where
    T: num_traits::Signed + num_traits::NumCast + num_traits::PrimInt + Copy,
{
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

#[allow(unused)]
impl Point3D<i16> {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            x: i16::try_from(x).unwrap(),
            y: i16::try_from(y).unwrap(),
            z: i16::try_from(z).unwrap(),
        }
    }
}

#[allow(unused)]
impl Point3D<i32> {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            x: i32::try_from(x).unwrap(),
            y: i32::try_from(y).unwrap(),
            z: i32::try_from(z).unwrap(),
        }
    }
}

#[allow(unused)]
impl Point3D<i64> {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            x: i64::try_from(x).unwrap(),
            y: i64::try_from(y).unwrap(),
            z: i64::try_from(z).unwrap(),
        }
    }
}

fn try_from_internal<S, N>(value: S) -> Result<Point3D<N>, &'static str>
where
    S: AsRef<str>,
    N: num_traits::Signed + num_traits::PrimInt + std::str::FromStr,
{
    let mut parts = value.as_ref().split(',');
    let x = parts
        .next()
        .ok_or("Missing x")?
        .parse::<N>()
        .map_err(|_| "Could not parse x as number")?;
    let y = parts
        .next()
        .ok_or("Missing y")?
        .parse::<N>()
        .map_err(|_| "Could not parse y as number")?;
    let z = parts
        .next()
        .ok_or("Missing z")?
        .parse::<N>()
        .map_err(|_| "Could not parse z as number")?;
    Ok(Point3D { x, y, z })
}

impl TryFrom<&str> for Point3D<i16> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<String> for Point3D<i16> {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<&str> for Point3D<i32> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<String> for Point3D<i32> {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<&str> for Point3D<i64> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}

impl TryFrom<String> for Point3D<i64> {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        try_from_internal(value)
    }
}
