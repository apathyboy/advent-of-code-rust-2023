pub mod template;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3D {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        (self.x - other.x).unsigned_abs()
            + (self.y - other.y).unsigned_abs()
            + (self.z - other.z).unsigned_abs()
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn mul_scalar(&self, scalar: isize) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Point2D {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn mul_scalar(&self, scalar: isize) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

pub fn parse_space_separated<T>(s: &str) -> Vec<T>
where
    T: Default + Clone + std::str::FromStr,
{
    s.split_whitespace()
        .filter_map(|word| word.parse::<T>().ok())
        .collect()
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
