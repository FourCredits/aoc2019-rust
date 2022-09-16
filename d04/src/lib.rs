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
    (0..(digits.len() - 1)).any(|i| {
        digits[i] == digits[i + 1]
            && (i == 0 || digits[i - 1] != digits[i])
            && digits.get(i + 2).map_or(true, |&a| a != digits[i])
    })
}

fn digits(mut n: u64) -> Vec<u8> {
    let mut result = Vec::new();
    while n > 0 {
        result.push((n % 10) as u8);
        n /= 10;
    }
    result.reverse();
    result
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
