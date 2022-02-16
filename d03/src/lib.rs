use std::collections::HashSet;

pub fn part_a(input: &str) -> u64 {
    let (wire1, wire2) = input.trim().split_once('\n').unwrap();
    let wire1: HashSet<_> = make_path(wire1).into_iter().collect();
    let wire2: HashSet<_> = make_path(wire2).into_iter().collect();
    wire1
        .intersection(&wire2)
        .map(|&pos| manhattan_distance((0, 0), pos))
        .min()
        .unwrap()
}

pub fn part_b(input: &str) -> usize {
    let (wire1, wire2) = input.trim().split_once('\n').unwrap();
    let (wire1, wire2) = (make_path(wire1), make_path(wire2));
    wire1
        .iter()
        .collect::<HashSet<_>>()
        .intersection(&wire2.iter().collect::<HashSet<_>>())
        .map(|&pos| {
            wire1.iter().position(|p| p == pos).unwrap()
                + wire2.iter().position(|p| p == pos).unwrap()
                + 2
        })
        .min()
        .unwrap()
}

fn manhattan_distance((y1, x1): (i64, i64), (y2, x2): (i64, i64)) -> u64 {
    (i64::abs(y2 - y1) + i64::abs(x2 - x1)) as u64
}

fn make_path(path: &str) -> Vec<(i64, i64)> {
    let (mut y, mut x) = (0, 0);
    let mut res = Vec::new();
    for line in path.split(',') {
        let (dy, dx) = match line.chars().next().unwrap() {
            'R' => (0, 1),
            'L' => (0, -1),
            'U' => (1, 0),
            'D' => (-1, 0),
            _ => unreachable!(),
        };
        for _ in 0..line[1..].parse::<u64>().unwrap() {
            y += dy;
            x += dx;
            res.push((y, x));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

    const EXAMPLE_2: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn example_a() {
        assert_eq!(part_a(EXAMPLE_1), 159);
        assert_eq!(part_a(EXAMPLE_2), 135)
    }

    #[test]
    fn example_b() {
        assert_eq!(part_b(EXAMPLE_1), 610);
        assert_eq!(part_b(EXAMPLE_2), 410);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 1195);
        assert_eq!(part_b(input), 91518);
    }
}
