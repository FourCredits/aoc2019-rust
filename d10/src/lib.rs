use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_a(input: &str) -> usize {
    let asteroids = parse_asteroids(input);
    asteroids
        .iter()
        .map(|&asteroid| count_detectable_asteroids(asteroid, &asteroids))
        .max()
        .unwrap()
}

pub fn part_b(input: &str) -> usize {
    let asteroids = parse_asteroids(input);
    let station = asteroids
        .iter()
        .map(|&asteroid| (asteroid, count_detectable_asteroids(asteroid, &asteroids)))
        .max_by_key(|&(_, n)| n)
        .unwrap()
        .0;
    let mut others: Vec<_> = asteroids
        .into_iter()
        .filter(|&asteroid| asteroid != station)
        .collect();
    others.sort_by_key(|&asteroid| {
        let (dx, dy) = distance_2d(station, asteroid);
        dx * dx + dy * dy
    });
    let mut groups = HashMap::new();
    others.iter().for_each(|&asteroid| {
        groups
            .entry(simplify(distance_2d(station, asteroid)))
            .or_insert_with(VecDeque::new)
            .push_back(asteroid);
    });
    let mut groups: Vec<_> = groups.into_iter().collect();
    groups.sort_by(|angle1, angle2| arg(angle1.0).partial_cmp(&arg(angle2.0)).unwrap());
    let mut groups: Vec<_> = groups.into_iter().map(|pair| pair.1).collect();
    let (x, y) = round_robin(&mut groups)[199];
    x * 100 + y
}

fn count_detectable_asteroids(station: (usize, usize), asteroids: &[(usize, usize)]) -> usize {
    asteroids
        .iter()
        .filter(|&&asteroid| station != asteroid)
        .map(|&asteroid| simplify(distance_2d(station, asteroid)))
        .collect::<HashSet<_>>()
        .len()
}

fn parse_asteroids(input: &str) -> Vec<(usize, usize)> {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect()
}

// finds the angle of a complex number, in the range of 0 to 2pi
fn arg((x, y): (isize, isize)) -> f64 {
    let angle = f64::atan2(x as f64, -y as f64);
    if angle < 0.0 {
        angle + 2.0 * std::f64::consts::PI
    } else {
        angle
    }
}

// goes through the collections, taking one element from each at a time
fn round_robin<T>(vs: &mut [VecDeque<T>]) -> Vec<T> {
    let mut result = Vec::new();
    while !vs.iter().any(|group| group.is_empty()) {
        for group in vs.iter_mut() {
            if let Some(v) = group.pop_front() {
                result.push(v);
            }
        }
    }
    result
}

// simplifies a fraction to its simplest form (or a position vector to it's
// simplest integer form)
fn simplify((dx, dy): (isize, isize)) -> (isize, isize) {
    let gcd = gcd(dx, dy);
    (dx / gcd, dy / gcd)
}

// finds the distance between two 2d positions
fn distance_2d((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> (isize, isize) {
    (
        (x1 as isize - x2 as isize).abs(),
        (y1 as isize - y2 as isize).abs(),
    )
}

// finds the greatest common divisor between two numbers
fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(2, 5), 1);
    }

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
