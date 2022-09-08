pub fn part_a(input: &str) -> usize {
    let (lo, hi) = input.split_once('-').unwrap();
    let (lo, hi) = (lo.parse::<u64>().unwrap(), hi.parse::<u64>().unwrap());
    (lo..=hi).filter(|&n| matches_criteria1(n)).count()
}

pub fn part_b(input: &str) -> usize {
    let (lo, hi) = input.split_once('-').unwrap();
    let (lo, hi) = (lo.parse::<u64>().unwrap(), hi.parse::<u64>().unwrap());
    (lo..=hi).filter(|&n| matches_criteria2(n)).count()
}

fn matches_criteria1(n: u64) -> bool {
    let digits = digits(n);
    digits.len() == 6
        && digits.windows(2).all(|pair| pair[0] <= pair[1])
        && digits.windows(2).any(|pair| pair[0] == pair[1])
}

fn matches_criteria2(n: u64) -> bool {
    let digits = digits(n);
    digits.len() == 6 && digits.windows(2).all(|pair| pair[0] <= pair[1]) && group_of_two(&digits)
}

fn group_of_two(digits: &[u8]) -> bool {
    for i in 0..(digits.len() - 1) {
        if digits[i] == digits[i + 1] {
            let left_ok = i == 0 || digits[i - 1] != digits[i];
            let right_ok = digits.get(i + 2).map_or(true, |&a| a != digits[i]);
            if left_ok && right_ok {
                return true;
            }
        }
    }
    false
}

fn digits(mut n: u64) -> Vec<u8> {
    let mut res = Vec::new();
    while n > 0 {
        res.push((n % 10).try_into().unwrap());
        n /= 10;
    }
    res.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = "278384-824795";

    #[test]
    fn matches_criteria1_test() {
        assert!(matches_criteria1(111111));
        assert!(!matches_criteria1(223450));
        assert!(!matches_criteria1(123789));
    }

    #[test]
    fn matches_criteria2_test() {
        assert!(matches_criteria2(112233));
        assert!(!matches_criteria2(123444));
        assert!(matches_criteria2(111122));
    }

    #[test]
    fn real() {
        assert_eq!(part_a(REAL_INPUT), 921);
        assert_eq!(part_b(REAL_INPUT), 603);
    }
}
