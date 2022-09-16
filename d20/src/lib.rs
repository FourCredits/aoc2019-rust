mod portals;
mod recursive;

pub fn part_a(input: &str) -> usize {
    let maze = portals::Maze::new(input);
    let graph = portals::Graph::new(maze);
    graph.solve().unwrap()
}

pub fn part_b(input: &str) -> usize {
    let maze = recursive::Maze::new(input);
    let graph = recursive::Graph::new(maze);
    graph.solve().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_1: &str = include_str!("ex_1.txt");
    const EX_2: &str = include_str!("ex_2.txt");

    #[test]
    fn part_a_test() {
        assert_eq!(part_a(EX_1), 23);
        assert_eq!(part_a(EX_2), 58);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 632);
    }
}
