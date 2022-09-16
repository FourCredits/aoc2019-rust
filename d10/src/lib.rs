use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

use utils::v2::V2;

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
    let V2(y, x) = nth_destroyed(station, 199, others).unwrap();
    x * 100 + y
}

fn nth_destroyed(station: V2, n: usize, mut others: Vec<V2>) -> Option<V2> {
    others.sort_by(|&a1, &a2| {
        let magnitude1 = (a1 - station).magnitude();
        let magnitude2 = (a2 - station).magnitude();
        magnitude1.partial_cmp(&magnitude2).unwrap()
    });
    // Using a BTreeMap specifically for how it keeps things in order
    let mut groups: BTreeMap<Wrap, Vec<V2>> = BTreeMap::new();
    for &asteroid in &others {
        groups
            .entry(Wrap((asteroid - station).simplify()))
            .or_default()
            .push(asteroid);
    }
    let asteroids_by_angle: Vec<Vec<V2>> = groups.into_values().collect();
    let stop = asteroids_by_angle.iter().map(Vec::len).max().unwrap();
    (0..stop)
        .flat_map(|i| asteroids_by_angle.iter().filter_map(move |a| a.get(i)))
        .nth(n)
        .copied()
}

fn count_detectable_asteroids(station: V2, asteroids: &[V2]) -> usize {
    asteroids
        .iter()
        .filter_map(|&asteroid| (station != asteroid).then(|| (asteroid - station).simplify()))
        .collect::<HashSet<_>>()
        .len()
}

fn parse_asteroids(input: &str) -> Vec<V2> {
    utils::parse_grid(input)
        .filter_map(|(point, c)| (c == '#').then_some(point))
        .collect()
}

// A wrapper around V2 that has a different `Ord` definition, ordering by that `arg` function
#[derive(PartialEq, Eq)]
struct Wrap(V2);

impl PartialOrd for Wrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.arg().partial_cmp(&other.0.arg())
    }
}

impl Ord for Wrap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
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
}
