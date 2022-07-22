use std::{
    fmt::{self, Debug, Formatter},
    ops::{BitOr, BitOrAssign},
};

// KeySet is just a wrapper around a bit set. It gives convenient functions, so
// that it functions like a set should. Just easier to pass around then a full
// thing
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct KeySet {
    internal: u32,
}

impl KeySet {
    pub fn new() -> KeySet {
        KeySet { internal: 0 }
    }

    fn index(c: char) -> u32 {
        match c {
            '@' => 0,
            '1'..='4' => c as u32 - '1' as u32 + 27,
            c if c.is_ascii_lowercase() => c as u32 - 'a' as u32 + 1,
            _ => unreachable!(),
        }
    }

    pub fn contains(&self, key: char) -> bool {
        (self.internal & (1 << KeySet::index(key))) != 0
    }

    pub fn is_subset(&self, other: &KeySet) -> bool {
        (self.internal | other.internal) == other.internal
    }
}

impl FromIterator<char> for KeySet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        iter.into_iter().fold(KeySet::new(), |acc, c| acc | c)
    }
}

impl BitOr<char> for KeySet {
    type Output = Self;

    fn bitor(self, rhs: char) -> Self::Output {
        KeySet {
            internal: self.internal | (1 << KeySet::index(rhs)),
        }
    }
}

impl BitOrAssign<char> for KeySet {
    fn bitor_assign(&mut self, rhs: char) {
        *self = *self | rhs;
    }
}

impl Debug for KeySet {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let keys: Vec<char> = std::iter::once('@')
            .chain('a'..='z')
            .chain('1'..='4')
            .collect();
        fmt.debug_set()
            .entries(keys.iter().filter(|&&c| self.contains(c)))
            .finish()
    }
}
