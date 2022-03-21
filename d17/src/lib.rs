use std::collections::{HashMap, HashSet};

use intcode::IntcodeComputer;
use utils::V2;

type Position = V2;
type Direction = V2;
type Map = HashMap<Position, i64>;

const LEFT: Direction = V2(0, -1);
const RIGHT: Direction = V2(0, 1);
const UP: Direction = V2(-1, 0);
const DOWN: Direction = V2(1, 0);

#[derive(Debug)]
pub enum Instruction {
    Forward(i64),
    Right,
    Left,
}

pub fn part_a(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let map: Map = make_map(program);
    map.iter()
        .filter_map(|(k, &v)| {
            if is_scaffold(v)
                && k.taxicab_directions()
                    .iter()
                    .filter_map(|adj| map.get(adj).filter(|&&v| is_scaffold(v)))
                    .count()
                    == 4
            {
                let V2(x, y) = k;
                Some(x * y)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let mut program = IntcodeComputer::parse_program(input);
    program[0] = 2;
    let human_instructions = "A,C,C,A,B,A,B,A,B,C\n".to_owned() // main movement routine
        + "R,6,R,6,R,8,L,10,L,4\n"                              // A movement function
        + "L,4,L,12,R,6,L,10\n"                                 // B movement function
        + "R,6,L,10,R,8\n"                                      // C movement function
        + "n\n"; // No continuous video feed
    let intcode_instructions: Vec<i64> = human_instructions.chars().map(|c| c as i64).collect();
    let computer = IntcodeComputer::run_program(program, Some(intcode_instructions));
    *computer.output.last().unwrap()
}

pub fn find_path(map: &Map) -> Vec<Instruction> {
    let (mut position, mut direction) = find_robot(map);
    let is_available_position = |pos: V2, diff: V2| {
        map.get(&(pos + diff))
            .filter(|&&v| is_scaffold(v))
            .is_some()
    };
    let mut result = Vec::new();
    loop {
        let left_turn = left_of(direction);
        let right_turn = right_of(direction);
        if is_available_position(position, direction) {
            let mut n = 0;
            while is_available_position(position, direction) {
                n += 1;
                position += direction;
            }
            result.push(Instruction::Forward(n));
        } else if is_available_position(position, left_turn) {
            direction = left_turn;
            result.push(Instruction::Left);
        } else if is_available_position(position, right_turn) {
            direction = right_turn;
            result.push(Instruction::Right);
        } else {
            return result;
        }
    }
}

fn is_scaffold(space: i64) -> bool {
    space != 87 && space != 46
}

fn right_of(dir: Direction) -> V2 {
    match dir {
        LEFT => UP,
        RIGHT => DOWN,
        UP => RIGHT,
        DOWN => LEFT,
        _ => unreachable!(),
    }
}

fn left_of(dir: Direction) -> V2 {
    match dir {
        LEFT => DOWN,
        RIGHT => UP,
        UP => LEFT,
        DOWN => RIGHT,
        _ => unreachable!(),
    }
}

// Finds the starting position and direction of the robot
fn find_robot(map: &Map) -> (Position, Direction) {
    map.iter()
        .find_map(|(&k, v)| match v {
            60 => Some((k, LEFT)),
            62 => Some((k, RIGHT)),
            94 => Some((k, UP)),
            118 => Some((k, DOWN)),
            _ => None,
        })
        .unwrap_or_else(|| panic!("No robot found: {:?}", map.values().collect::<HashSet<_>>()))
}

pub fn make_map(program: Vec<i64>) -> Map {
    IntcodeComputer::run_program(program, None)
        .output
        .split(|&num| num == 10)
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &e)| (V2(i as i64, j as i64), e))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 5740);
        assert_eq!(part_b(input), 1022165);
    }
}
