use intcode::IntcodeComputer;
use std::collections::HashMap;

use utils::v2::V2;

pub fn part_a(input: &str) -> usize {
    let painted = run_paint_program(input, false);
    painted.len()
}

pub fn part_b(input: &str) -> String {
    let painted = run_paint_program(input, true);
    let min_y = painted.keys().map(|p| p.0).min().unwrap();
    let max_y = painted.keys().map(|p| p.0).max().unwrap();
    let min_x = painted.keys().map(|p| p.1).min().unwrap();
    let max_x = painted.keys().map(|p| p.1).max().unwrap();
    let mut result = String::new();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let tile = *painted.get(&V2(y, x)).unwrap_or(&0);
            result.push(if tile == 0 { '.' } else { '#' });
        }
        result.push('\n');
    }
    result
}

fn run_paint_program(input: &str, start_on_white: bool) -> HashMap<V2, i64> {
    let program = IntcodeComputer::parse_program(input);
    let mut computer = IntcodeComputer::new(program, None);
    let mut direction = 0;
    let mut position = V2(0, 0);
    let mut painted = if start_on_white {
        HashMap::from([(position, 1)])
    } else {
        HashMap::new()
    };
    loop {
        computer.add_input(*painted.get(&position).unwrap_or(&0));
        computer.run_until_needs_input();
        if computer.halted {
            break;
        }
        let color = computer.output.remove(0);
        let turn = computer.output.remove(0);
        painted.insert(position, color);
        direction = change_direction(turn, direction);
        position = move_position(direction, position);
    }
    painted
}

fn change_direction(turn: i64, current_direction: u8) -> u8 {
    match turn {
        0 => (current_direction + 3) % 4,
        1 => (current_direction + 1) % 4,
        _ => unreachable!(),
    }
}

fn move_position(direction: u8, current_position: V2) -> V2 {
    current_position
        + match direction {
            0 => V2(1, 0),
            1 => V2(0, 1),
            2 => V2(-1, 0),
            3 => V2(0, -1),
            _ => unreachable!(),
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 2082);
        let output = include_str!("letters.txt");
        assert_eq!(part_b(input), output);
    }
}
