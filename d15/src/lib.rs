use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let mut computer = IntcodeComputer::new(program, None);
    let map = explore(&mut computer);
    shortest_path(&map).unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let mut computer = IntcodeComputer::new(program, None);
    let map = explore(&mut computer);
    time_oxygen_spread(&map)
}

// a breadth-first traversal of the given input space, labelling the oxygen
// system as 0, and then adding 1 as the oxygen travels outwards
fn time_oxygen_spread(map: &HashMap<(i64, i64), i64>) -> i64 {
    let oxygen_system = *map.iter().find(|&(_k, &v)| v == 2).unwrap().0;
    let mut oxygen_spread = HashMap::new();
    let mut queue = VecDeque::from([(oxygen_system, 0)]);
    while let Some((pos, elapsed_time)) = queue.pop_front() {
        oxygen_spread.insert(pos, elapsed_time);
        for new_pos in taxicab_directions(pos) {
            if !oxygen_spread.contains_key(&new_pos) && matches!(map.get(&new_pos), Some(1 | 2)) {
                queue.push_back((new_pos, elapsed_time + 1));
            }
        }
    }
    *oxygen_spread.values().max().unwrap()
}

// finds the shortest path to the oxygen system, via a breadth-first search
fn shortest_path(map: &HashMap<(i64, i64), i64>) -> Option<i64> {
    let start = (0, 0);
    let mut queue: VecDeque<((i64, i64), i64)> = VecDeque::from([(start, 0)]);
    let mut visited: HashSet<(i64, i64)> = HashSet::from([start]);
    while let Some((pos, path_length)) = queue.pop_front() {
        if map[&pos] == 2 {
            return Some(path_length);
        }
        visited.insert(pos);
        for new_pos in taxicab_directions(pos) {
            if !visited.contains(&new_pos) && matches!(map.get(&new_pos), Some(1 | 2)) {
                queue.push_back((new_pos, path_length + 1));
            }
        }
    }
    None
}

fn taxicab_directions(pos: (i64, i64)) -> Vec<(i64, i64)> {
    vec![
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ]
}

// A depth-first traversal, based on what the intcode computer says about the
// input space
fn explore(comp: &mut IntcodeComputer) -> HashMap<(i64, i64), i64> {
    let mut current_pos = (0, 0);
    let mut fully_explored: HashSet<(i64, i64)> = HashSet::new();
    let mut result: HashMap<(i64, i64), i64> = HashMap::from([(current_pos, 1)]);
    let mut predecessor: HashMap<(i64, i64), ((i64, i64), i64)> = HashMap::new();
    loop {
        let mut any_new = false;
        for dir in 1..=4 {
            let new_pos = match dir {
                1 => (current_pos.0 + 1, current_pos.1),
                2 => (current_pos.0 - 1, current_pos.1),
                3 => (current_pos.0, current_pos.1 - 1),
                4 => (current_pos.0, current_pos.1 + 1),
                _ => unreachable!(),
            };
            if let Entry::Vacant(e) = result.entry(new_pos) {
                let response = make_move(comp, dir);
                e.insert(response);
                if response != 0 {
                    predecessor.insert(new_pos, (current_pos, opposite_direction(dir)));
                    current_pos = new_pos;
                    any_new = true;
                }
            }
        }
        if !any_new {
            if current_pos == (0, 0) {
                return result;
            }
            // backtrack: find last node that isn't fully explored
            fully_explored.insert(current_pos);
            while fully_explored.contains(&current_pos) {
                let (previous, direction) = predecessor[&current_pos];
                current_pos = previous;
                make_move(comp, direction);
            }
        }
    }
}

fn opposite_direction(i: i64) -> i64 {
    match i {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => unreachable!(),
    }
}

fn make_move(comp: &mut IntcodeComputer, dir: i64) -> i64 {
    comp.add_input(dir);
    comp.run_until_needs_input();
    comp.output.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_path_test() {
        let input = HashMap::from([
            ((1, 0), 0),
            ((1, 1), 0),
            ((0, -1), 0),
            ((0, 0), 1),
            ((0, 1), 1),
            ((0, 2), 0),
            ((-1, -1), 2),
            ((-1, 0), 1),
            ((-1, 1), 0),
            ((-2, 0), 0),
        ]);
        assert_eq!(shortest_path(&input), Some(2));
    }

    #[test]
    fn time_oxygen_spread_test() {
        let input = HashMap::from([
            ((3, -1), 0),
            ((3, 0), 0),
            ((2, -2), 0),
            ((2, -1), 1),
            ((2, 0), 1),
            ((2, 1), 0),
            ((2, 2), 0),
            ((1, -2), 0),
            ((1, -1), 1),
            ((1, 0), 0),
            ((1, 1), 1),
            ((1, 2), 1),
            ((1, 3), 0),
            ((0, -2), 0),
            ((0, -1), 1),
            ((0, 0), 2),
            ((0, 1), 1),
            ((0, 2), 0),
            ((-1, -1), 0),
            ((-1, 0), 0),
            ((-1, 1), 0),
        ]);
        assert_eq!(time_oxygen_spread(&input), 4);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 252);
        assert_eq!(part_b(input), 350);
    }
}
