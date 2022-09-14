use std::{
    cmp,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use utils::v2::V2;

pub fn part_a(input: &str) -> usize {
    let maze = Maze::new(input);
    let graph = Graph::new(maze);
    graph.solve().unwrap()
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    start: V2,
    end: V2,
    edges: HashMap<V2, HashMap<V2, usize>>,
}

impl Graph {
    fn new(maze: Maze) -> Self {
        let mut graph = Graph {
            start: V2(0, 0),
            end: V2(0, 0),
            edges: HashMap::new(),
        };
        let nodes = graph.discover_nodes(&maze);
        for &node in &nodes {
            graph.explore_out(node, &nodes, &maze);
        }
        graph
    }

    fn discover_nodes(&mut self, maze: &Maze) -> HashSet<V2> {
        let mut unmatched_portals = HashMap::new();
        let mut nodes = HashSet::new();
        for (portal_key, portal) in maze.portals() {
            nodes.insert(portal);
            if let Some(other) = unmatched_portals.remove(&portal_key) {
                self.add_edge(portal, other, 1);
                self.add_edge(other, portal, 1);
            } else {
                unmatched_portals.insert(portal_key, portal);
            }
        }
        self.start = unmatched_portals.remove(&('A', 'A')).unwrap();
        self.end = unmatched_portals.remove(&('Z', 'Z')).unwrap();
        nodes
    }

    fn explore_out(&mut self, start: V2, nodes: &HashSet<V2>, maze: &Maze) {
        let mut visited = HashSet::from([start]);
        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((pos, distance)) = queue.pop_front() {
            if nodes.contains(&pos) && pos != start {
                self.add_edge(start, pos, distance);
            }
            for neighbour in pos.taxicab_directions() {
                if maze.has_point(&neighbour) && visited.insert(neighbour) {
                    queue.push_back((neighbour, distance + 1));
                }
            }
        }
    }

    fn add_edge(&mut self, from: V2, to: V2, new_dist: usize) {
        let edges_of_from = self.edges.entry(from).or_default();
        if let Some(dist) = edges_of_from.get_mut(&to) {
            *dist = cmp::min(*dist, new_dist);
        } else {
            edges_of_from.insert(to, new_dist);
        }
    }

    fn solve(&self) -> Option<usize> {
        let mut to_visit = BinaryHeap::from([State::new(self.start, 0)]);
        let mut distances = HashMap::from([(self.start, 0)]);
        let mut visited = HashSet::new();
        while let Some(State { position, distance }) = to_visit.pop() {
            if !visited.insert(position) {
                continue;
            }
            for (neighbour, cost) in &self.edges[&position] {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(neighbour)
                    .map_or(true, |&current| new_distance < current);
                if is_shorter {
                    distances.insert(*neighbour, new_distance);
                    to_visit.push(State::new(*neighbour, new_distance));
                }
            }
        }
        distances.get(&self.end).copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Maze {
    maze: HashMap<V2, char>,
}

impl Maze {
    fn new(input: &str) -> Maze {
        let maze = utils::parse_grid(input)
            .filter(|&(_, c)| c == '.' || c.is_ascii_alphabetic())
            .collect();
        Maze { maze }
    }

    fn node_at_point(&self, pos: V2) -> Option<((char, char), V2)> {
        let above = pos - V2(1, 0);
        let below = pos + V2(1, 0);
        let left = pos - V2(0, 1);
        let right = pos + V2(0, 1);
        let f = |pos: V2| self.maze.get(&pos);
        let p = |c1: char, c2: char| c1.is_ascii_alphabetic() && c2.is_ascii_alphabetic();
        match (f(above), f(left), f(pos), f(right), f(below)) {
            (Some(&c1), _, Some(&c2), _, Some(&'.')) if p(c1, c2) => Some(((c1, c2), below)),
            (Some(&'.'), _, Some(&c1), _, Some(&c2)) if p(c1, c2) => Some(((c1, c2), above)),
            (_, Some(&c1), Some(&c2), Some(&'.'), _) if p(c1, c2) => Some(((c1, c2), right)),
            (_, Some(&'.'), Some(&c1), Some(&c2), _) if p(c1, c2) => Some(((c1, c2), left)),
            _ => None,
        }
    }

    fn portals(&self) -> impl Iterator<Item = ((char, char), V2)> + '_ {
        self.maze
            .iter()
            .filter(|(_, &c)| c != '.')
            .filter_map(|(pos, _)| self.node_at_point(*pos))
    }

    pub fn has_point(&self, k: &V2) -> bool {
        self.maze.contains_key(k)
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    position: V2,
    distance: usize,
}

impl State {
    fn new(position: V2, distance: usize) -> Self {
        Self { position, distance }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl Eq for State {}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_1: &str = include_str!("ex_1.txt");
    const EX_2: &str = include_str!("ex_2.txt");

    #[test]
    fn parse_test() {
        let graph = Graph::new(Maze::new(EX_1));
        let actual: HashSet<V2> = graph.edges.keys().copied().collect();
        let expected = HashSet::from([
            V2(2, 9),
            V2(6, 9),
            V2(8, 2),
            V2(10, 6),
            V2(12, 11),
            V2(13, 2),
            V2(15, 2),
            V2(16, 13),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn priority_queue_test() {
        let s1 = State {
            position: V2(16, 13),
            distance: 26,
        };
        let s2 = State {
            position: V2(12, 11),
            distance: 30,
        };
        let mut priority_queue = BinaryHeap::from([s1, s2]);
        assert_eq!(priority_queue.pop(), Some(s1));
        assert_eq!(priority_queue.pop(), Some(s2));
    }

    #[test]
    fn part_a_test() {
        assert_eq!(part_a(EX_1), 23);
        assert_eq!(part_a(EX_2), 58);
    }

    #[test]
    fn idempotence() {
        let input = include_str!("input.txt");
        let maze1 = Maze::new(input);
        let maze2 = Maze::new(input);
        assert_eq!(maze1, maze2, "mazes are different");
        let graph1 = Graph::new(maze1);
        let graph2 = Graph::new(maze2);
        assert_eq!(graph1, graph2, "graphs are different");
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 632);
    }
}
