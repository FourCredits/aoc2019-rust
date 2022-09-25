use std::num::ParseIntError;

use modexp::{modexp, BigInt};

pub fn part_a(input: &str) -> isize {
    parse(input)
        .unwrap()
        .iter()
        .fold(2019, |pos, shuffle| shuffle.perform(pos, 10_007))
}

// Adapted from https://github.com/tginsberg/advent-2019-kotlin/blob/master/src/main/kotlin/com/ginsberg/advent2019/Day22.kt
// Don't really understand it, but then, neither did they
pub fn part_b(input: &str) -> BigInt {
    let num_cards: BigInt = 119315717514047_isize.into();
    let num_shuffles: BigInt = 101741582076661_isize.into();
    let mut memory0: BigInt = 1.into();
    let mut memory1: BigInt = 0.into();
    let find = 2020;
    for instruction in parse(input).unwrap().iter().rev() {
        match instruction {
            Shuffle::Cut(n) => {
                memory1 = memory1 + n;
            }
            Shuffle::Increment(n) => {
                let t = BigInt::from(*n).modpow(&(&num_cards - 2), &num_cards);
                memory0 = memory0 * &t;
                memory1 = memory1 * t;
            }
            Shuffle::NewStack => {
                memory0 = -memory0;
                memory1 = (memory1 + 1) * -1;
            }
        }
    }
    let power = memory0.modpow(&num_shuffles, &num_cards);
    ((&power * find)
        + ((memory1 * (power + &num_cards - 1))
            * modexp(memory0 - 1, &num_cards - 2, num_cards.clone())))
        % num_cards
}

fn parse(input: &str) -> Result<Vec<Shuffle>, ParseError> {
    input.trim().lines().map(|l| l.try_into()).collect()
}

#[derive(PartialEq, Debug)]
enum Shuffle {
    NewStack,
    Cut(isize),
    Increment(isize),
}

impl Shuffle {
    /// Performs the function, mapping `pos` from its current place in a deck of size
    /// `size` and returns its position in the deck shuffled according to the given rule.
    fn perform(&self, pos: isize, size: isize) -> isize {
        match self {
            Shuffle::NewStack => size - (pos + 1),
            Shuffle::Cut(n) => (pos - n).rem_euclid(size),
            Shuffle::Increment(n) => (pos * n) % size,
        }
    }
}

impl TryFrom<&str> for Shuffle {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "deal into new stack" {
            Ok(Shuffle::NewStack)
        } else if let Some(n) = value.strip_prefix("cut ") {
            Ok(Shuffle::Cut(n.parse()?))
        } else if let Some(n) = value.strip_prefix("deal with increment ") {
            Ok(Shuffle::Increment(n.parse()?))
        } else {
            Err(ParseError::InvalidRule)
        }
    }
}

#[derive(PartialEq, Debug)]
enum ParseError {
    InvalidRule,
    InvalidNumber(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::InvalidNumber(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let text = "cut -6
deal with increment 7
deal into new stack";
        let shuffles = parse(text);
        let expected = Ok(vec![
            Shuffle::Cut(-6),
            Shuffle::Increment(7),
            Shuffle::NewStack,
        ]);
        assert_eq!(shuffles, expected);
    }

    #[test]
    fn part_a_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 4086);
    }

    #[test]
    fn part_b_test() {
        let input = include_str!("input.txt");
        let result = part_b(input);
        println!("{}", result);
        assert_eq!(result, 1041334417227_isize.into());
    }
}
