use std::iter;

pub fn part_a(input: &str) -> i64 {
    let message = parse(input);
    let matrix = make_pattern_matrix(message.len());
    let final_message = iter::successors(Some(message), |inp| Some(mat_mul(&matrix, inp)))
        .nth(100)
        .unwrap();
    digits_to_num(&final_message[..8])
}

// taken from https://github.com/prscoelho/aoc2019/blob/master/src/aoc16/mod.rs
// and then thoroughly edited by me, although I don't know how it works
pub fn part_b(input: &str) -> i64 {
    let message = parse(input);
    let start = digits_to_num(&message[..7]) as usize;
    let end = message.len() * 10_000;
    let mut current: Vec<_> = (start..end).map(|i| message[i % message.len()]).collect();
    for _ in 0..100 {
        let sums: Vec<_> = std::iter::once(&0)
            .chain(current.iter())
            .scan(0, |total, &c| {
                *total += c;
                Some(*total)
            })
            .collect();
        for (i, c) in current.iter_mut().enumerate() {
            *c = (sums.last().unwrap() - sums[i]) % 10;
        }
    }
    digits_to_num(&current[..8])
}

fn digits_to_num(ns: &[i64]) -> i64 {
    ns.iter().fold(0, |acc, digit| acc * 10 + digit)
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| i64::from(c.to_digit(10).unwrap()))
        .collect()
}

fn mat_mul(matrix: &[Vec<i64>], vector: &[i64]) -> Vec<i64> {
    let v_len = vector.len();
    let m1_len = matrix.len();
    let m2_len = matrix[0].len();
    assert_eq!(m2_len, v_len);
    (0..m1_len)
        .map(|i| {
            vector
                .iter()
                .enumerate()
                .map(|(j, v)| matrix[i][j] * v)
                .sum::<i64>()
                .abs()
                % 10
        })
        .collect()
}

fn make_pattern_matrix(size: usize) -> Vec<Vec<i64>> {
    const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];
    let mut result = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            result[i][j] = BASE_PATTERN[((j + 1) / (i + 1)) % 4];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_example() {
        let inputs = [
            ("80871224585914546619083218645595", 24176176),
            ("19617804207202209144916044189917", 73745418),
            ("69317163492948606335995924319873", 52432133),
        ];
        for (inp, out) in inputs {
            assert_eq!(part_a(inp), out);
        }
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 67481260);
        assert_eq!(part_b(input), 42178738);
    }
}
