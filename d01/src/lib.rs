pub fn part_a(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|l| fuel(l.parse::<i64>().unwrap()))
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|l| fuel_recursive(l.parse::<i64>().unwrap()))
        .sum()
}

fn fuel(n: i64) -> i64 {
    (n / 3) - 2
}

fn fuel_recursive(n: i64) -> i64 {
    use std::iter::successors;
    successors(Some(n), |&n| {
        let f = fuel(n);
        if f > 0 {
            Some(f)
        } else {
            None
        }
    })
    .skip(1)
    .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_a() {
        assert_eq!(super::part_a("12"), 2);
        assert_eq!(super::part_a("14"), 2);
        assert_eq!(super::part_a("1969"), 654);
        assert_eq!(super::part_a("100756"), 33583);
    }

    #[test]
    fn example_b() {
        assert_eq!(super::part_a("14"), 2);
        assert_eq!(super::part_a("1969"), 654);
        assert_eq!(super::part_b("100756"), 50346);
    }

    #[test]
    fn real() {
        assert_eq!(super::part_a(include_str!("input.txt")), 3216868);
        assert_eq!(super::part_b(include_str!("input.txt")), 4822435);
    }
}
