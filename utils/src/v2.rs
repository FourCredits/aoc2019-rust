use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
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
