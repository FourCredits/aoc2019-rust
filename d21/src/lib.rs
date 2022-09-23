use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> Result<i64, String> {
    let jumpscript_program = "OR A T
AND B T
AND C T
NOT T J
AND D J
WALK
";
    let intcode_program = IntcodeComputer::parse_program(input);
    run_jumpscript(intcode_program, jumpscript_program)
}

pub fn part_b(input: &str) -> Result<i64, String> {
    let jumpscript_program = "NOT C J  
AND D J  
NOT H T  
NOT T T  
OR E T
AND T J  
NOT A T  
OR T J
NOT B T  
NOT T T  
OR E T
NOT T T  
OR T J
RUN
";
    let intcode_program = IntcodeComputer::parse_program(input);
    run_jumpscript(intcode_program, jumpscript_program)
}

fn run_jumpscript(intcode: Vec<i64>, jumpscript: &str) -> Result<i64, String> {
    let mut computer = IntcodeComputer::new(intcode, None);
    let mut program = jumpscript.chars().map(from_ascii);
    while let Some(i) = program.next() {
        computer.run_until_needs_input();
        computer.add_input(i);
    }
    computer.run();
    let output = computer.output;
    let damage = output[output.len() - 1];
    if damage >= 128 {
        Ok(damage)
    } else {
        let err: String = output.into_iter().map(to_ascii).collect();
        Err(err)
    }
}

pub fn to_ascii(n: i64) -> char {
    char::from(n as u8)
}

pub fn from_ascii(c: char) -> i64 {
    c as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jumpscript(expected: i64, actual: Result<i64, String>) {
        match actual {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Springdroid fell in hole!\n{}", err),
        }
    }

    #[test]
    fn part_a_test() {
        let input = include_str!("input.txt");
        jumpscript(19355645, part_a(input));
    }

    #[test]
    fn part_b_test() {
        let input = include_str!("input.txt");
        jumpscript(1137899149, part_b(input));
    }
}
