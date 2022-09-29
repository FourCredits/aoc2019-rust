use std::collections::HashSet;

use utils::v2::V2;

use crate::bug_rules;

pub fn part_b(input: &str) -> usize {
    let mut grid = parse(input);
    for _ in 0..200 {
        grid = tick(grid);
    }
    grid.len()
}

pub fn tick(grid: HashSet<Point>) -> HashSet<Point> {
    relevant_points(&grid)
        .into_iter()
        .filter(|&pos: &Point| bug_rules(grid.contains(&pos), alive_neighbours(&grid, pos)))
        .collect()
}

fn relevant_points(grid: &HashSet<Point>) -> HashSet<Point> {
    let mut ps = grid.clone();
    for p in grid {
        ps.extend(neighbours(*p));
    }
    ps
}

fn alive_neighbours(grid: &HashSet<Point>, p: Point) -> usize {
    neighbours(p).iter().filter(|&p| grid.contains(p)).count()
}

fn neighbours(p: Point) -> HashSet<Point> {
    let mut result = HashSet::new();
    let depth = p.depth;
    for V2(y, x) in V2(p.y, p.x).taxicab_neighbours() {
        if (y, x) != (2, 2) && (x >= 0) && (y >= 0) && (x <= 4) && (y <= 4) {
            result.insert(Point::new(depth, y, x));
            continue;
        }
        if (y, x) == (2, 2) {
            let news = (0..5).map(|i| match (p.y, p.x) {
                (1, 2) => Point::new(depth + 1, 0, i),
                (2, 1) => Point::new(depth + 1, i, 0),
                (3, 2) => Point::new(depth + 1, 4, i),
                (2, 3) => Point::new(depth + 1, i, 4),
                _ => unreachable!(),
            });
            result.extend(news);
        }
        if x < 0 {
            result.insert(Point::new(p.depth - 1, 2, 1));
        } else if x > 4 {
            result.insert(Point::new(p.depth - 1, 2, 3));
        }
        if y < 0 {
            result.insert(Point::new(p.depth - 1, 1, 2));
        } else if y > 4 {
            result.insert(Point::new(p.depth - 1, 3, 2));
        }
    }
    result
}

pub fn parse(input: &str) -> HashSet<Point> {
    utils::parse_grid(input)
        .filter(|&(_, c)| c == '#')
        .map(|(V2(y, x), _)| Point::new(0, y, x))
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    depth: i64,
    y: i64,
    x: i64,
}

impl Point {
    fn new(depth: i64, y: i64, x: i64) -> Self {
        Self { depth, y, x }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacency_rules() {
        let expected = HashSet::from([
            Point::new(0, 4, 3),
            Point::new(0, 2, 3),
            Point::new(0, 3, 2),
            Point::new(0, 3, 4),
        ]);
        assert_eq!(neighbours(Point::new(0, 3, 3)), expected);
        let expected = HashSet::from([
            Point::new(1, 2, 1),
            Point::new(1, 0, 1),
            Point::new(1, 1, 0),
            Point::new(1, 1, 2),
        ]);
        assert_eq!(neighbours(Point::new(1, 1, 1)), expected);
        let expected = HashSet::from([
            Point::new(1, 0, 4),
            Point::new(1, 0, 2),
            Point::new(1, 1, 3),
            Point::new(0, 1, 2),
        ]);
        assert_eq!(neighbours(Point::new(1, 0, 3)), expected);
        let expected = HashSet::from([
            Point::new(1, 0, 3),
            Point::new(1, 1, 4),
            Point::new(0, 1, 2),
            Point::new(0, 2, 3),
        ]);
        assert_eq!(neighbours(Point::new(1, 0, 4)), expected);
        let expected = HashSet::from([
            Point::new(0, 1, 3),
            Point::new(0, 3, 3),
            Point::new(0, 2, 4),
            Point::new(1, 0, 4),
            Point::new(1, 1, 4),
            Point::new(1, 2, 4),
            Point::new(1, 3, 4),
            Point::new(1, 4, 4),
        ]);
        assert_eq!(neighbours(Point::new(0, 2, 3)), expected);
        let expected = HashSet::from([
            Point::new(1, 1, 3),
            Point::new(1, 3, 3),
            Point::new(1, 2, 4),
            Point::new(2, 0, 4),
            Point::new(2, 1, 4),
            Point::new(2, 2, 4),
            Point::new(2, 3, 4),
            Point::new(2, 4, 4),
        ]);
        assert_eq!(neighbours(Point::new(1, 2, 3)), expected);
    }

    #[test]
    fn example() {
        let text = "....#
#..#.
#..##
..#..
#....";
        let mut grid = parse(text);
        for _ in 0..10 {
            grid = tick(grid);
        }
        assert_eq!(grid.len(), 99);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 1983);
    }
}
