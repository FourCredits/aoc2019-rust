use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let computer = IntcodeComputer::run_program(program, Some(vec![1]));
    computer.output[0]
}

pub fn part_b(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let computer = IntcodeComputer::run_program(program, Some(vec![2]));
    computer.output[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 2518058886);
        assert_eq!(part_b(input), 44292);
    }
}
