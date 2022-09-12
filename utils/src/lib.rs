pub mod bfs;
pub mod v2;
pub mod v3;

pub const fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

pub fn parse_grid(input: &str) -> impl Iterator<Item = (v2::V2, char)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, c)| (v2::V2(y as i64, x as i64), c))
    })
}

pub const fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(2, 5), 1);
        assert_eq!(gcd(2, 0), 2);
        assert_eq!(gcd(0, 2), 2);
        assert_eq!(gcd(-2, 0), 2);
        assert_eq!(gcd(0, -2), 2);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(2, 1), 2);
        assert_eq!(lcm(2, 8), 8);
        assert_eq!(lcm(2, 0), 0);
        assert_eq!(lcm(0, 2), 0);
    }
}
