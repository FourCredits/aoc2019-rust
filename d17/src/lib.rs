use std::collections::HashMap;

use intcode::IntcodeComputer;
use utils::V2;

pub fn part_a(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let map: HashMap<V2, i64> = make_map(program);
    map.iter()
        .filter_map(|(k, &v)| {
            if v != 46
                && k.taxicab_directions()
                    .iter()
                    .filter_map(|adj| map.get(adj).filter(|&&adj_v| adj_v != 46))
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

fn make_map(program: Vec<i64>) -> HashMap<V2, i64> {
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
    }
}
