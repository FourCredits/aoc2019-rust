use intcode::IntcodeComputer;
use std::collections::HashMap;

pub fn part_a(input: &str) -> usize {
    let program = IntcodeComputer::parse_program(input);
    let mut computer = IntcodeComputer::new(program, None);
    let mut direction = 0;
    let mut position = (0, 0);
    let mut painted: HashMap<(i64, i64), i64> = HashMap::new();
    loop {
        computer.add_input(*painted.get(&position).unwrap_or(&0));
        computer.run_until_needs_input();
        if computer.halted {
            break;
        }
        let color = computer.output.remove(0);
        painted.insert(position, color);
        let turn = computer.output.remove(0);
        match turn {
            0 => {
                direction = (direction + 3) % 4;
            }
            1 => {
                direction = (direction + 1) % 4;
            }
            _ => unreachable!(),
        };
        match direction {
            0 => {
                position.0 += 1;
            }
            1 => {
                position.1 += 1;
            }
            2 => {
                position.0 -= 1;
            }
            3 => {
                position.1 -= 1;
            }
            _ => unreachable!(),
        };
    }
    painted.len()
}

pub fn part_b(input: &str) -> String {
    let program = IntcodeComputer::parse_program(input);
    let mut computer = IntcodeComputer::new(program, None);
    let mut direction = 0;
    let mut position = (0, 0);
    let mut painted: HashMap<(i64, i64), i64> = HashMap::from([(position, 1)]);
    loop {
        computer.add_input(*painted.get(&position).unwrap_or(&0));
        computer.run_until_needs_input();
        if computer.halted {
            break;
        }
        let color = computer.output.remove(0);
        painted.insert(position, color);
        let turn = computer.output.remove(0);
        match turn {
            0 => {
                direction = (direction + 3) % 4;
            }
            1 => {
                direction = (direction + 1) % 4;
            }
            _ => unreachable!(),
        };
        match direction {
            0 => {
                position.0 += 1;
            }
            1 => {
                position.1 += 1;
            }
            2 => {
                position.0 -= 1;
            }
            3 => {
                position.1 -= 1;
            }
            _ => unreachable!(),
        };
    }
    let min_y = painted.keys().map(|p| p.0).min().unwrap();
    let max_y = painted.keys().map(|p| p.0).max().unwrap();
    let min_x = painted.keys().map(|p| p.1).min().unwrap();
    let max_x = painted.keys().map(|p| p.1).max().unwrap();
    let mut result = String::new();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let tile = *painted.get(&(y, x)).unwrap_or(&0);
            result.push(if tile == 0 { '.' } else { '#' });
        }
        result.push('\n');
    }
    result
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
