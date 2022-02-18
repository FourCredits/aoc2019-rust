use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let computer = IntcodeComputer::run_program(program, Some(vec![1]));
    computer.output[computer.output.len() - 1]
}

pub fn part_b(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let computer = IntcodeComputer::run_program(program, Some(vec![5]));
    computer.output[computer.output.len() - 1]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::part_a(include_str!("input.txt")), 7259358);
        assert_eq!(super::part_b(include_str!("input.txt")), 11826654);
    }
}
