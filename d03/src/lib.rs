use std::collections::HashSet;

use utils::v2::V2;

pub fn part_a(input: &str) -> i64 {
    let (wire1, wire2) = input.trim().split_once('\n').unwrap();
    common_points(&make_path(wire1), &make_path(wire2))
        .iter()
        .map(|&pos| V2(0, 0).manhattan_distance(pos))
        .min()
        .unwrap()
}

pub fn part_b(input: &str) -> usize {
    let (wire1, wire2) = input.trim().split_once('\n').unwrap();
    let (wire1, wire2) = (make_path(wire1), make_path(wire2));
    common_points(&wire1, &wire2)
        .iter()
        .map(|&pos| {
            wire1.iter().position(|&p| p == pos).unwrap()
                + wire2.iter().position(|&p| p == pos).unwrap()
                + 2
        })
        .min()
        .unwrap()
}

fn common_points(wire1: &Vec<V2>, wire2: &Vec<V2>) -> HashSet<V2> {
    let set1: HashSet<_> = wire1.iter().collect();
    let set2: HashSet<_> = wire2.iter().collect();
    set1.intersection(&set2).map(|p| **p).collect()
}

fn make_path(path: &str) -> Vec<V2> {
    let mut position = V2(0, 0);
    let mut result = Vec::new();
    for line in path.split(',') {
        let direction = match line.as_bytes()[0] {
            b'R' => V2(0, 1),
            b'L' => V2(0, -1),
            b'U' => V2(1, 0),
            b'D' => V2(-1, 0),
            _ => unreachable!(),
        };
        for _ in 0..line[1..].parse::<u64>().unwrap() {
            position += direction;
            result.push(position);
        }
    }
    result
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
