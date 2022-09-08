use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::ops::Sub;

// TODO: replace with the utils V2

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct V2(i64, i64);

impl V2 {
    fn magnitude(&self) -> f64 {
        ((self.0 * self.0 + self.1 * self.1) as f64).sqrt()
    }

    fn arg(&self) -> f64 {
        let angle = f64::atan2(self.0 as f64, -self.1 as f64);
        if angle < 0.0 {
            std::f64::consts::PI.mul_add(2.0, angle)
        } else {
            angle
        }
    }

    const fn simplify(&self) -> Self {
        let gcd = utils::gcd(self.0, self.1);
        Self(self.0 / gcd, self.1 / gcd)
    }
}

impl Sub for V2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl PartialOrd for V2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.arg().partial_cmp(&other.arg())
    }
}

impl Ord for V2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.arg().partial_cmp(&other.arg()).unwrap()
    }
}

pub fn part_a(input: &str) -> usize {
    let asteroids = parse_asteroids(input);
    asteroids
        .iter()
        .map(|&asteroid| count_detectable_asteroids(asteroid, &asteroids))
        .max()
        .unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let asteroids = parse_asteroids(input);
    let station = *asteroids
        .iter()
        .max_by_key(|&&asteroid| count_detectable_asteroids(asteroid, &asteroids))
        .unwrap();
    let others: Vec<_> = asteroids
        .into_iter()
        .filter(|&asteroid| asteroid != station)
        .collect();
    let V2(x, y) = destroyed_order(station, others)[199];
    x * 100 + y
}

fn destroyed_order(station: V2, mut others: Vec<V2>) -> Vec<V2> {
    others.sort_by(|&a1, &a2| {
        (a1 - station)
            .magnitude()
            .partial_cmp(&(a2 - station).magnitude())
            .unwrap()
    });
    let mut groups = BTreeMap::new();
    for &asteroid in &others {
        groups
            .entry((asteroid - station).simplify())
            .or_insert_with(Vec::new)
            .push(asteroid);
    }
    round_robin(&groups.values().cloned().collect::<Vec<_>>())
}

fn count_detectable_asteroids(station: V2, asteroids: &[V2]) -> usize {
    asteroids
        .iter()
        .filter(|&&asteroid| station != asteroid)
        .map(|&asteroid| (asteroid - station).simplify())
        .collect::<HashSet<_>>()
        .len()
}

fn parse_asteroids(input: &str) -> Vec<V2> {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(V2(x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

// goes through the collections, taking one element from each at a time
fn round_robin<T: Copy>(vals: &[Vec<T>]) -> Vec<T> {
    let mut result = Vec::new();
    let stop = vals.iter().map(Vec::len).max().unwrap();
    let mut i = 0;
    while i < stop {
        for val in vals {
            if let Some(v) = val.get(i) {
                result.push(*v);
            }
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = ".#..#
.....
#####
....#
...##";
        assert_eq!(part_a(input), 8);
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        assert_eq!(part_a(input), 33);
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        assert_eq!(part_a(input), 35);
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        assert_eq!(part_a(input), 41);
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(part_a(input), 210);
    }

    #[test]
    fn worked_example() {
        let input = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let asteroids = parse_asteroids(input);
        let station = asteroids
            .iter()
            .map(|&asteroid| (asteroid, count_detectable_asteroids(asteroid, &asteroids)))
            .max_by_key(|&(_, n)| n)
            .unwrap()
            .0;
        let others: Vec<_> = asteroids
            .into_iter()
            .filter(|&asteroid| asteroid != station)
            .collect();
        let t = destroyed_order(station, others);
        assert_eq!(
            t.into_iter().take(9).collect::<Vec<_>>(),
            vec![
                V2(8, 1),
                V2(9, 0),
                V2(9, 1),
                V2(10, 0),
                V2(9, 2),
                V2(11, 1),
                V2(12, 1),
                V2(11, 2),
                V2(15, 1)
            ]
        );
    }

    #[test]
    fn example_b() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(part_b(input), 802);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 309);
        assert_eq!(part_b(input), 416);
    }

    #[test]
    fn round_robin_test() {
        let input = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(round_robin(&input), vec![1, 3, 2, 4]);
        let input = vec![vec![1, 2], vec![3, 4, 5]];
        assert_eq!(round_robin(&input), vec![1, 3, 2, 4, 5]);
    }
}
