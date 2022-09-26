use std::collections::HashSet;

use utils::v2::V2;

pub fn part_a(input: &str) -> usize {
    let mut grid = parse(input);
    let mut seen = HashSet::new();
    loop {
        let rating = biodiversity_rating(&grid);
        if !seen.insert(rating) {
            return rating;
        }
        grid = tick(grid);
    }
}

fn bug_rules(alive: bool, num_neighbours: usize) -> bool {
    (alive && num_neighbours == 1) || (!alive && (num_neighbours == 1 || num_neighbours == 2))
}

fn biodiversity_rating(grid: &HashSet<V2>) -> usize {
    grid.iter().map(|V2(y, x)| 1 << (5 * y + x)).sum()
}

fn tick(grid: HashSet<V2>) -> HashSet<V2> {
    new_grid()
        .filter(|pos: &V2| bug_rules(grid.contains(&pos), neighbours(&grid, *pos)))
        .collect()
}

fn neighbours(grid: &HashSet<V2>, pos: V2) -> usize {
    pos.taxicab_neighbours()
        .into_iter()
        .filter(|neighbour| grid.contains(&neighbour))
        .count()
}

fn parse(input: &str) -> HashSet<V2> {
    utils::parse_grid(input)
        .filter_map(|(pos, c)| (c == '#').then_some(pos))
        .collect()
}

fn new_grid() -> impl Iterator<Item = V2> {
    (0..5).flat_map(|y| (0..5).map(move |x| V2(y, x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biodiversity_test() {
        let text = ".....
.....
.....
#....
.#...";
        let input = parse(text);
        assert_eq!(biodiversity_rating(&input), 2129920);
    }

    #[test]
    fn example() {
        let text = "....#
#..#.
#..##
..#..
#....";
        assert_eq!(part_a(text), 2129920);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 20751345);
    }
}
