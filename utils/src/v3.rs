use std::ops::{AddAssign, Sub};

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
