// The version where labels lead to recursive copies of the maze, aka part 2.

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use utils::v2::V2;

pub struct Graph {
    start: V2,
    end: V2,
    edges: HashMap<V2, HashMap<V2, (usize, DepthChange)>>,
}

impl Graph {
    pub fn new(maze: Maze) -> Self {
        let mut graph = Self {
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
        let mut inner_labels = HashMap::new();
        let mut outer_labels = HashMap::new();
        let mut labels = HashSet::new();
        for (portal_key, portal, goes_deeper) in maze.labels() {
            labels.insert(portal);
            if goes_deeper {
                inner_labels.insert(portal_key, portal);
            } else {
                outer_labels.insert(portal_key, portal);
            }
        }
        for (label, in_pos) in inner_labels {
            let out_pos = outer_labels[&label];
            self.add_edge(in_pos, out_pos, 1, DepthChange::Deeper);
            self.add_edge(out_pos, in_pos, 1, DepthChange::Shallower);
        }
        self.start = outer_labels[&('A', 'A')];
        self.end = outer_labels[&('Z', 'Z')];
        labels
    }

    fn explore_out(&mut self, start: V2, labels: &HashSet<V2>, maze: &Maze) {
        let mut visited = HashSet::from([start]);
        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((pos, distance)) = queue.pop_front() {
            if labels.contains(&pos) && pos != start {
                self.add_edge(start, pos, distance, DepthChange::NoChange);
            }
            for neighbour in pos.taxicab_neighbours() {
                if maze.has_point(&neighbour) && visited.insert(neighbour) {
                    queue.push_back((neighbour, distance + 1));
                }
            }
        }
    }

    fn add_edge(&mut self, from: V2, to: V2, distance: usize, change: DepthChange) {
        self.edges
            .entry(from)
            .or_default()
            .insert(to, (distance, change));
    }

    pub fn solve(&self) -> Option<usize> {
        let mut to_visit = BinaryHeap::from([State::new(self.start, 0, 0)]);
        let mut visited = HashSet::new();
        while let Some(State {
            position,
            distance,
            depth,
        }) = to_visit.pop()
        {
            if !visited.insert(position) {
                continue;
            }
            if position == self.end && depth == 0 {
                return Some(distance);
            }
            for (&neighbour, (cost, change)) in &self.edges[&position] {
                if depth != 0 && (neighbour == self.end || neighbour == self.start) {
                    continue;
                }
                let new_depth = match change {
                    DepthChange::Deeper => depth + 1,
                    DepthChange::NoChange => depth,
                    DepthChange::Shallower => depth - 1,
                };
                let new_distance = distance + cost;
                to_visit.push(State::new(neighbour, new_distance, new_depth));
            }
        }
        None
    }
}

enum DepthChange {
    Deeper,
    NoChange,
    Shallower,
}

pub struct Maze {
    maze: HashMap<V2, char>,
    bottom_right: V2,
}

impl Maze {
    pub fn new(input: &str) -> Maze {
        let maze: HashMap<_, _> = utils::parse_grid(input)
            .filter(|&(_, c)| c == '.' || c.is_ascii_alphabetic())
            .collect();
        let bottom_right = utils::parse_grid(input)
            .map(|(pos, _)| pos)
            .max_by_key(|pos| pos.manhattan_distance(V2(0, 0)))
            .unwrap();
        Self { maze, bottom_right }
    }

    fn labels(&self) -> impl Iterator<Item = ((char, char), V2, bool)> + '_ {
        self.maze
            .iter()
            .filter(|(_, &c)| c != '.')
            .filter_map(|(pos, _)| self.node_at_point(*pos))
    }

    fn node_at_point(&self, pos: V2) -> Option<((char, char), V2, bool)> {
        let above = pos - V2(1, 0);
        let below = pos + V2(1, 0);
        let left = pos - V2(0, 1);
        let right = pos + V2(0, 1);
        let f = |pos: V2| self.maze.get(&pos);
        let p = |c1: char, c2: char| c1.is_ascii_alphabetic() && c2.is_ascii_alphabetic();
        match (f(above), f(left), f(pos), f(right), f(below)) {
            (Some(&c1), _, Some(&c2), _, Some(&'.')) if p(c1, c2) => {
                Some(((c1, c2), below, self.is_inner_portal(above)))
            }
            (Some(&'.'), _, Some(&c1), _, Some(&c2)) if p(c1, c2) => {
                Some(((c1, c2), above, self.is_inner_portal(below)))
            }
            (_, Some(&c1), Some(&c2), Some(&'.'), _) if p(c1, c2) => {
                Some(((c1, c2), right, self.is_inner_portal(left)))
            }
            (_, Some(&'.'), Some(&c1), Some(&c2), _) if p(c1, c2) => {
                Some(((c1, c2), left, self.is_inner_portal(right)))
            }
            _ => None,
        }
    }

    fn is_inner_portal(&self, V2(y, x): V2) -> bool {
        !(y == 0 || x == 0 || y == self.bottom_right.0 || x == self.bottom_right.1)
    }

    pub fn has_point(&self, k: &V2) -> bool {
        self.maze.contains_key(k)
    }
}

struct State {
    position: V2,
    distance: usize,
    depth: u32,
}

impl State {
    fn new(position: V2, distance: usize, depth: u32) -> Self {
        Self {
            position,
            distance,
            depth,
        }
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
    const EX_3: &str = include_str!("ex_3.txt");

    #[test]
    fn inner_vs_outer() {
        let maze = Maze::new(EX_1);
        assert!(!maze.is_inner_portal(V2(0, 9)));
        assert!(maze.is_inner_portal(V2(8, 9)));
        assert!(!maze.is_inner_portal(V2(18, 13)));
    }

    #[test]
    fn example_1() {
        let maze = Maze::new(EX_1);
        let graph = Graph::new(maze);
        let actual = graph.solve();
        assert_eq!(actual, Some(26));
    }

    #[test]
    fn example_2() {
        let maze = Maze::new(EX_2);
        let graph = Graph::new(maze);
        let actual = graph.solve();
        assert_eq!(actual, None);
    }

    #[test]
    fn example_3() {
        let maze = Maze::new(EX_3);
        let graph = Graph::new(maze);
        let actual = graph.solve();
        assert_eq!(actual, Some(396));
    }
}
