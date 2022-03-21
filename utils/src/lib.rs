use std::cmp::*;
use std::fmt;
use std::ops::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct V3(pub i64, pub i64, pub i64);

impl V3 {
    pub fn signum(self) -> V3 {
        Self(self.0.signum(), self.1.signum(), self.2.signum())
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct V2(pub i64, pub i64);

impl V2 {
    pub fn taxicab_directions(self) -> [V2; 4] {
        let V2(x, y) = self;
        [V2(x + 1, y), V2(x - 1, y), V2(x, y - 1), V2(x, y + 1)]
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

pub fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(2, 5), 1);
        assert_eq!(gcd(2, 0), 2);
        assert_eq!(gcd(0, 2), 2);
        assert_eq!(gcd(-2, 0), 2);
        assert_eq!(gcd(0, -2), 2);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(2, 1), 2);
        assert_eq!(lcm(2, 8), 8);
        assert_eq!(lcm(2, 0), 0);
        assert_eq!(lcm(0, 2), 0);
    }
}
