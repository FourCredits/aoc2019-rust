use std::collections::HashMap;

use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> usize {
    get_screen(&mut IntcodeComputer::run_program(
        IntcodeComputer::parse_program(input),
        None,
    ))
    .values()
    .filter(|&&n| n == 2)
    .count()
}

pub fn part_b(input: &str) -> i64 {
    let mut program = IntcodeComputer::parse_program(input);
    program[0] = 2;
    let mut computer = IntcodeComputer::new(program, None);
    loop {
        computer.run_until_needs_input();
        let screen = get_screen(&mut computer);
        if computer.halted {
            return screen[&(-1, 0)];
        }
        let ball_pos = screen.iter().find(|&(_k, &v)| v == 4).unwrap().0;
        let paddle_pos = screen.iter().find(|&(_k, &v)| v == 3).unwrap().0;
        let joystick_position = (ball_pos.0 - paddle_pos.0).signum();
        computer.add_input(joystick_position);
    }
}

fn get_screen(computer: &mut IntcodeComputer) -> HashMap<(i64, i64), i64> {
    computer
        .output
        .chunks(3)
        .map(|instruction| ((instruction[0], instruction[1]), instruction[2]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 265);
        assert_eq!(part_b(input), 13331);
    }
}
