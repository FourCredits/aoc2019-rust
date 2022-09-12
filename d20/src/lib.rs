use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

use utils::v2::V2;

type Maze = HashMap<V2, char>;

pub fn part_a(input: &str) -> usize {
    let maze = make_maze(input);
    let graph = Graph::new(&maze);
    graph.solve().unwrap()
}

fn make_maze(input: &str) -> Maze {
    utils::parse_grid(input)
        .filter(|&(_, c)| c == '.' || c.is_ascii_alphabetic())
        .collect()
}

#[derive(Debug)]
struct Graph {
    start: V2,
    end: V2,
    edges: HashMap<V2, Vec<(V2, usize)>>,
}

impl Graph {
    fn new(maze: &Maze) -> Self {
        let mut graph = Graph {
            start: V2(0, 0),
            end: V2(0, 0),
            edges: HashMap::new(),
        };
        let nodes = graph.discover_nodes(maze);
        for &node in &nodes {
            graph.explore_out(node, &nodes, &maze);
        }
        graph
    }

    fn discover_nodes(&mut self, maze: &Maze) -> HashSet<V2> {
        let mut unmatched_portals = HashMap::new();
        let mut nodes = HashSet::new();
        for (&pos, &c1) in maze {
            if c1 == '.' {
                continue;
            }
            if let Some((c2, portal_entrance)) = node_at_point(maze, pos) {
                nodes.insert(portal_entrance);
                let label = order(c1, c2);
                if label == ('A', 'A') {
                    self.start = portal_entrance;
                } else if label == ('Z', 'Z') {
                    self.end = portal_entrance;
                }
                if let Some(other_portal_entrance) = unmatched_portals.remove(&label) {
                    self.add_portal(portal_entrance, other_portal_entrance);
                } else {
                    unmatched_portals.insert(label, portal_entrance);
                }
            };
        }
        assert_eq!(
            unmatched_portals,
            HashMap::from([(('A', 'A'), self.start), (('Z', 'Z'), self.end)])
        );
        nodes
    }

    fn explore_out(&mut self, start: V2, nodes: &HashSet<V2>, maze: &Maze) {
        let mut visited = HashSet::from([start]);
        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((pos, distance)) = queue.pop_front() {
            visited.insert(pos);
            if nodes.contains(&pos) && pos != start {
                self.edges.entry(start).or_default().push((pos, distance));
            }
            for neighbour in pos.taxicab_directions() {
                if maze.contains_key(&neighbour) && !visited.contains(&neighbour) {
                    queue.push_back((neighbour, distance + 1));
                }
            }
        }
    }

    fn add_portal(&mut self, pos1: V2, pos2: V2) {
        self.edges.entry(pos1).or_default().push((pos2, 1));
        self.edges.entry(pos2).or_default().push((pos1, 1));
    }

    fn solve(&self) -> Option<usize> {
        let mut priority_queue = BinaryHeap::from([State {
            position: self.start,
            distance: 0,
        }]);
        let mut visited = HashSet::new();
        while let Some(state) = priority_queue.pop() {
            visited.insert(state.position);
            if state.position == self.end {
                return Some(state.distance);
            }
            let nexts = self.edges[&state.position]
                .iter()
                .filter(|&(np, _)| !visited.contains(np))
                .map(|(new_pos, d)| State::new(*new_pos, state.distance + d));
            priority_queue.extend(nexts);
        }
        None
    }
}

fn order(a: char, b: char) -> (char, char) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn node_at_point(maze: &Maze, pos: V2) -> Option<(char, V2)> {
    pos.taxicab_directions().into_iter().find_map(|neighbour| {
        let open_tile = pos + (pos - neighbour);
        match (maze.get(&neighbour), maze.get(&open_tile)) {
            (Some(&c), Some('.')) if c.is_ascii_alphabetic() => Some((c, open_tile)),
            _ => None,
        }
    })
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    const EX_1: &str = include_str!("ex_1.txt");
    const EX_2: &str = include_str!("ex_2.txt");

    #[test]
    fn parse_test() {
        let graph = Graph::new(&make_maze(EX_1));
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
    fn portal_test() {
        let maze = make_maze(EX_1);
        let actual = node_at_point(&maze, V2(1, 9));
        let expected = Some(('A', V2(2, 9)));
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
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 632);
    }
}
