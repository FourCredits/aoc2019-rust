use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> usize {
    let reader = Drone::new(input);
    (0..50)
        .map(|y| (0..50).filter(|&x| reader.is_in_beam(y, x)).count())
        .sum()
}

// Adapted from https://todd.ginsberg.com/post/advent-of-code/2019/day19/ - also has a good
// explanation of how this works
pub fn part_b(input: &str) -> i64 {
    let reader = Drone::new(input);
    let mut x = 0;
    for y in 0.. {
        while !reader.is_in_beam(y + 99, x) {
            x += 1;
        }
        if reader.is_in_beam(y, x + 99) {
            return x * 10_000 + y;
        }
    }
    unreachable!();
}

struct Drone(Vec<i64>);

impl Drone {
    fn new(program: &str) -> Self {
        Self(IntcodeComputer::parse_program(program))
    }

    fn is_in_beam(&self, y: i64, x: i64) -> bool {
        IntcodeComputer::run_program(self.0.clone(), Some(vec![x, y])).output[0] == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 164);
    }

    #[test]
    fn part_b_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 13081049);
    }
}
