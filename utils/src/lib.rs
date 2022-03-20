use std::cmp::*;
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
