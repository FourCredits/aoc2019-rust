use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct V2(pub i64, pub i64);

// Note that, throughout the code base, I've used V2 as a 2D point in (y, x) format

impl V2 {
    pub const fn taxicab_neighbours(self) -> [Self; 4] {
        let Self(x, y) = self;
        [
            Self(x + 1, y),
            Self(x - 1, y),
            Self(x, y - 1),
            Self(x, y + 1),
        ]
    }

    pub const fn manhattan_distance(self, other: Self) -> i64 {
        i64::abs(other.1 - self.1) + i64::abs(other.0 - self.0)
    }

    pub fn magnitude(self) -> f64 {
        ((self.0 * self.0 + self.1 * self.1) as f64).sqrt()
    }

    pub fn arg(&self) -> f64 {
        let angle = f64::atan2(self.1 as f64, -self.0 as f64);
        if angle < 0.0 {
            std::f64::consts::PI.mul_add(2.0, angle)
        } else {
            angle
        }
    }

    // If you're interpreting a V2 as a direction pointing away from the origin, this function
    // puts it into its simplest form - a bit like making a unit vector, using only integers.
    pub const fn simplify(&self) -> Self {
        let gcd = crate::gcd(self.0, self.1);
        Self(self.0 / gcd, self.1 / gcd)
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

impl Mul<i64> for V2 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<V2> for i64 {
    type Output = V2;

    fn mul(self, rhs: V2) -> Self::Output {
        V2(rhs.0 * self, rhs.1 * self)
    }
}

impl MulAssign<i64> for V2 {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
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
