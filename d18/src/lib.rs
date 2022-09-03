// Taken from https://github.com/prscoelho/aoc2019/blob/master/src/aoc18/mod.rs
// They do things mostly the same as I tried, but I'm not entirely sure why theirs
// works and mine didn't.

use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use utils::v2::V2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Blank,
    Wall,
    Node(char),
}

type Grid = HashMap<V2, Tile>;
type Graph = HashMap<char, HashMap<char, usize>>;

fn parse(input: &str) -> Grid {
    utils::parse_grid(input)
        .map(|(pos, c)| (pos, parse_tile(c)))
        .collect::<Grid>()
}

fn parse_tile(c: char) -> Tile {
    match c {
        '#' => Tile::Wall,
        '.' => Tile::Blank,
        _ => Tile::Node(c),
    }
}

fn build_graph(grid: &Grid) -> Graph {
    let mut graph = HashMap::new();
    for (coord, tile) in grid {
        if let Tile::Node(c) = tile {
            graph.insert(*c, reachable_from(grid, *coord));
        }
    }
    graph
}

fn reachable_from(grid: &Grid, start: V2) -> HashMap<char, usize> {
    let mut visited = HashSet::from([start]);
    let mut result = HashMap::new();
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some((current_pos, steps)) = queue.pop_front() {
        for neighbour in &current_pos.taxicab_directions() {
            if let Some(tile) = grid.get(neighbour) {
                if visited.contains(neighbour) {
                    continue;
                }
                visited.insert(*neighbour);
                match tile {
                    Tile::Blank => {
                        queue.push_back((*neighbour, steps + 1));
                    }
                    Tile::Node(c) => {
                        result.insert(*c, steps + 1);
                    }
                    Tile::Wall => {}
                }
            }
        }
    }
    result
}

#[derive(PartialEq, Eq)]
struct State {
    steps: usize,
    node: char,
    keys: BTreeSet<char>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn count_keys(graph: &Graph) -> usize {
    graph.iter().filter(|(k, _)| k.is_lowercase()).count()
}

fn search(graph: Graph) -> Option<usize> {
    let key_count = count_keys(&graph);
    let mut priority_queue = BinaryHeap::from([State {
        steps: 0,
        node: '@',
        keys: BTreeSet::new(),
    }]);
    let mut best_costs = HashMap::from([(('@', BTreeSet::new()), 0)]);
    let mut cache = HashMap::new();
    while let Some(current) = priority_queue.pop() {
        if current.keys.len() == key_count {
            return Some(current.steps);
        }
        let next_options = cache
            .entry((current.node, current.keys.clone()))
            .or_insert_with(|| search_keys(&graph, &current.keys, current.node));
        for &(next_node, cost) in next_options.iter() {
            let mut next_keys = current.keys.clone();
            next_keys.insert(next_node);
            let next_steps = current.steps + cost;
            let previous_shortest = best_costs
                .entry((next_node, next_keys.clone()))
                .or_insert(usize::max_value());
            if next_steps < *previous_shortest {
                *previous_shortest = next_steps;
                priority_queue.push(State {
                    steps: next_steps,
                    node: next_node,
                    keys: next_keys,
                });
            }
        }
    }
    None
}

#[derive(PartialEq, Eq)]
struct DijkstraState {
    cost: usize,
    node: char,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// adapted from https://doc.rust-lang.org/std/collections/binary_heap/index.html
// dijkstra search for reachable new keys from start node
fn search_keys(graph: &Graph, keys: &BTreeSet<char>, start: char) -> Vec<(char, usize)> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::from([(start, 0)]);
    let mut heap = BinaryHeap::from([DijkstraState {
        cost: 0,
        node: start,
    }]);
    let mut reach = HashSet::new();
    while let Some(DijkstraState { cost, node }) = heap.pop() {
        if node.is_lowercase() && !keys.contains(&node) {
            reach.insert(node);
            continue;
        }
        for (&next_node, &next_cost) in graph[&node].iter() {
            if next_node.is_uppercase() && !keys.contains(&next_node.to_ascii_lowercase()) {
                continue;
            }
            let next = DijkstraState {
                cost: cost + next_cost,
                node: next_node,
            };
            if !matches!(dist.get(&next.node), Some(&d) if next.cost >= d) {
                dist.insert(next.node, next.cost);
                heap.push(next);
            }
        }
    }
    reach.into_iter().map(|node| (node, dist[&node])).collect()
}

pub fn part_1(input: &str) -> usize {
    let grid = parse(input);
    let graph = build_graph(&grid);
    search(graph).unwrap()
}

// modify grid to split map into 4 sections
// add 4 robots on each section
fn four_robots(grid: &mut HashMap<V2, Tile>) {
    let robot_coord = grid
        .iter()
        .find_map(|(k, &v)| (v == Tile::Node('@')).then_some(*k))
        .unwrap();

    grid.insert(robot_coord, Tile::Wall);
    for &neighbour in &robot_coord.taxicab_directions() {
        grid.insert(neighbour, Tile::Wall);
    }
    grid.insert(V2(robot_coord.0 - 1, robot_coord.1 - 1), Tile::Node('@'));
    grid.insert(V2(robot_coord.0 - 1, robot_coord.1 + 1), Tile::Node('='));
    grid.insert(V2(robot_coord.0 + 1, robot_coord.1 + 1), Tile::Node('%'));
    grid.insert(V2(robot_coord.0 + 1, robot_coord.1 - 1), Tile::Node('$'));
}

fn search_four(graph: Graph) -> Option<usize> {
    let key_count = count_keys(&graph);
    let robots = ['@', '=', '%', '$'];
    let mut priority_queue = BinaryHeap::from([FourState {
        steps: 0,
        robots,
        keys: BTreeSet::new(),
    }]);
    let mut best_costs = HashMap::from([((robots, BTreeSet::new()), 0)]);
    let mut cache = HashMap::new();
    while let Some(current) = priority_queue.pop() {
        if current.keys.len() == key_count {
            return Some(current.steps);
        }
        for (robot_number, &robot_location) in current.robots.iter().enumerate() {
            let next_options = cache
                .entry((robot_location, current.keys.clone()))
                .or_insert_with(|| search_keys(&graph, &current.keys, robot_location));
            for &(next_node, cost) in next_options.iter() {
                let mut next_keys = current.keys.clone();
                next_keys.insert(next_node);
                let mut next_robots = current.robots;
                next_robots[robot_number] = next_node;
                let next_steps = current.steps + cost;
                let previous_shortest = best_costs
                    .entry((next_robots, next_keys.clone()))
                    .or_insert(usize::max_value());
                if next_steps < *previous_shortest {
                    *previous_shortest = next_steps;
                    priority_queue.push(FourState {
                        steps: next_steps,
                        robots: next_robots,
                        keys: next_keys,
                    });
                }
            }
        }
    }
    None
}

#[derive(PartialEq, Eq)]
struct FourState {
    steps: usize,
    robots: [char; 4],
    keys: BTreeSet<char>,
}

impl Ord for FourState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for FourState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_2(input: &str) -> usize {
    let mut grid = parse(input);
    four_robots(&mut grid);
    let graph = build_graph(&grid);
    search_four(graph).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first() {
        let input = include_str!("input.txt");
        let expected = 4544;
        assert_eq!(part_1(input), expected);
    }

    #[test]
    fn second() {
        let input = include_str!("input.txt");
        let expected = 1692;
        assert_eq!(part_2(input), expected);
    }
}
