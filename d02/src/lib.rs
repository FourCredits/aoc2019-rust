pub fn part_a(input: &str) -> i64 {
    let program = {
        let mut program: Vec<i64> = input
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        program[1] = 12;
        program[2] = 2;
        program
    };
    let finished = intcode::run_program(program);
    println!("{:?}", finished[0]);
    finished[0]
}

pub fn part_b(input: &str) -> Option<i64> {
    let mut start_program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    for noun in 0..=99 {
        start_program[1] = noun;
        for verb in 0..=99 {
            start_program[2] = verb;
            let finished_program = intcode::run_program(start_program.clone());
            if finished_program[0] == 19690720 {
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
