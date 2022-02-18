use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> i64 {
    let program = {
        let mut program = IntcodeComputer::parse_program(input);
        program[1] = 12;
        program[2] = 2;
        program
    };
    IntcodeComputer::run_program(program, None).data[0]
}

pub fn part_b(input: &str) -> Option<i64> {
    let mut start_program = IntcodeComputer::parse_program(input);
    for noun in 0..=99 {
        start_program[1] = noun;
        for verb in 0..=99 {
            start_program[2] = verb;
            let computer = IntcodeComputer::run_program(start_program.clone(), None);
            if computer.data[0] == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 6730673);
        assert_eq!(part_b(input), Some(3749));
    }
}
