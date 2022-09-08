use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct V2(pub i64, pub i64);

impl V2 {
    // TODO: rename to taxicab_neighbours
    pub const fn taxicab_directions(self) -> [Self; 4] {
        let Self(x, y) = self;
        [
            Self(x + 1, y),
            Self(x - 1, y),
            Self(x, y - 1),
            Self(x, y + 1),
        ]
    }

    pub fn taxicab_distance(self, other: Self) -> i64 {
        let diff = self - other;
        diff.0.abs() + diff.1.abs()
    }
}

impl fmt::Debug for V2 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("V2").field(&self.0).field(&self.1).finish()
    }
}

impl Add for V2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for V2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Sub for V2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for V2 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}
