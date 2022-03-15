use std::iter;

pub fn part_a(input: &str) -> i64 {
    let message: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();
    iter::successors(Some(message), |inp| Some(fft(inp)))
        .nth(100)
        .unwrap()
        .iter()
        .take(8)
        .fold(0, |acc, d| acc * 10 + d)
}

fn fft(input: &[i64]) -> Vec<i64> {
    let base_pattern = [0, 1, 0, -1];
    (1..=input.len())
        .map(|index| {
            let pattern = base_pattern
                .into_iter()
                .map(|n| iter::repeat(n).take(index))
                .flatten()
                .cycle()
                .skip(1);
            input
                .iter()
                .zip(pattern)
                .map(|(inp, pat)| inp * pat)
                .sum::<i64>()
                .abs()
                % 10
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fft_test() {
        let mut signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        signal = fft(&signal);
        assert_eq!(signal, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        signal = fft(&signal);
        assert_eq!(signal, vec![3, 4, 0, 4, 0, 4, 3, 8]);
        signal = fft(&signal);
        assert_eq!(signal, vec![0, 3, 4, 1, 5, 5, 1, 8]);
        signal = fft(&signal);
        assert_eq!(signal, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

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
    }
}
