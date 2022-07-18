/*
TODO: this is what i've got so far:
- most of the tests work, except for example_4 and example_5
- as far as i can see, the parsing of the graph is correct
- so something in the actual graph traversal is incorrect?
- i'd like to make some of the functions a little less right leaning, do some
  good ol' fashioned extracting
- making the graph shows some holdovers from previous code. it could probably
  just be two free functions, make_map and make_graph
- running tests::real doesn't seem to complete in a reasonable amount of time.
  some optimisation required, once everything else is correct...
- I've also not really done a full project test to see if any refactorings have
  broken anything, so I should probably do that
*/

use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fmt::{self, Debug, Formatter},
    ops::BitOr,
};

use utils::v2::V2;

#[derive(PartialEq, Eq)]
enum Tile {
    Blank,
    Wall,
    Key(char),
    Door(char),
}

struct Map {
    map: HashMap<V2, Tile>,
}

impl Map {
    fn new(input: &str) -> Map {
        let mut map: HashMap<V2, Tile> = HashMap::new();
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = V2(y as i64, x as i64);
                let tile = match c {
                    '.' => Tile::Blank,
                    '#' => Tile::Wall,
                    c if c.is_ascii_lowercase() || c == '@' => Tile::Key(c),
                    c if c.is_ascii_uppercase() => Tile::Door(c.to_ascii_lowercase()),
                    _ => unreachable!(),
                };
                map.insert(pos, tile);
            }
        }
        Map { map }
    }

    fn make_graph(&self) -> Graph {
        let keys: Vec<(V2, char)> = self
            .map
            .iter()
            .filter_map(|(k, v)| {
                if let Tile::Key(key) = v {
                    Some((*k, *key))
                } else {
                    None
                }
            })
            .collect();
        let nodes = KeySet::from_nodes(&keys.iter().map(|&(_, node)| node).collect::<Vec<_>>());
        let mut connections = HashMap::new();
        for (pos1, source) in keys {
            let mut queue = VecDeque::from([(pos1, 0, KeySet::new())]);
            let mut visited = HashSet::new();
            while let Some((pos2, distance, keys)) = queue.pop_front() {
                visited.insert(pos2);
                for neighbour in pos2.taxicab_directions() {
                    if visited.contains(&neighbour) {
                        continue;
                    }
                    match self.map.get(&neighbour) {
                        Some(&Tile::Key(destination))
                            if !connections.contains_key(&(destination, source)) =>
                        {
                            connections.insert((source, destination), (distance + 1, keys));
                        }
                        Some(&Tile::Door(new_key)) => {
                            queue.push_back((neighbour, distance + 1, keys | new_key));
                        }
                        Some(&Tile::Blank) => {
                            queue.push_back((neighbour, distance + 1, keys));
                        }
                        _ => {}
                    }
                }
            }
        }
        let connections: Vec<(char, char, usize, KeySet)> = connections
            .into_iter()
            .map(|((s, d), (dist, ks))| (s, d, dist, ks))
            .collect();
        Graph::new(nodes, connections)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct KeySet {
    internal: u32,
}

impl KeySet {
    fn new() -> KeySet {
        KeySet { internal: 0 }
    }

    fn from_nodes(nodes: &[char]) -> KeySet {
        nodes.iter().fold(KeySet::new(), |acc, &c| acc | c)
    }

    fn nodes_range(end_node: char) -> KeySet {
        KeySet::from_nodes(
            &std::iter::once('@')
                .chain('a'..=end_node)
                .collect::<Vec<_>>(),
        )
    }

    fn index(c: char) -> u32 {
        if c == '@' {
            0
        } else if c.is_ascii_lowercase() {
            c as u32 - 'a' as u32 + 1
        } else {
            unreachable!()
        }
    }

    fn contains(&self, key: char) -> bool {
        (self.internal & (1 << KeySet::index(key))) != 0
    }

    fn is_subset(&self, other: &KeySet) -> bool {
        other.internal > self.internal
    }
}

impl BitOr<char> for KeySet {
    type Output = Self;

    fn bitor(self, rhs: char) -> Self::Output {
        KeySet {
            internal: self.internal | (1 << KeySet::index(rhs)),
        }
    }
}

impl Debug for KeySet {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let keys: Vec<char> = std::iter::once('@').chain('a'..='z').collect();
        fmt.debug_set()
            .entries(keys.iter().filter(|&&c| self.contains(c)))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    nodes: KeySet,
    edges: HashMap<char, BTreeSet<(char, usize, KeySet)>>,
}

impl Graph {
    fn new(nodes: KeySet, connections: Vec<(char, char, usize, KeySet)>) -> Graph {
        let mut edges = HashMap::new();
        for (node1, node2, distance, doors) in connections {
            assert!(nodes.contains(node1));
            assert!(nodes.contains(node2));
            edges
                .entry(node1)
                .or_insert(BTreeSet::new())
                .insert((node2, distance, doors));
            edges
                .entry(node2)
                .or_insert(BTreeSet::new())
                .insert((node1, distance, doors));
        }
        Graph { nodes, edges }
    }

    fn best_path(&self) -> usize {
        self.shortest_path('@', KeySet::new() | '@', 0, &mut HashMap::new())
            .unwrap()
    }

    fn shortest_path(
        &self,
        start: char,
        keys: KeySet,
        distance_travelled: usize,
        visited: &mut HashMap<(char, KeySet), usize>,
    ) -> Option<usize> {
        // println!("calling with start {start} and keys {keys:?}");
        if let Some(&previous_call) = visited.get(&(start, keys)) {
            if previous_call < distance_travelled {
                // If we're trying to explore from the smae place, with the same
                // keys, and there's already a better solution, then we don't
                // need to try this: we know there's a better option
                return None;
            }
        }
        if keys == self.nodes {
            // We've collected all the keys, so we're vinished
            Some(distance_travelled)
        } else {
            visited.insert((start, keys), distance_travelled);
            self.edges[&start]
                .iter()
                .filter_map(|(neighbour, distance, doors)| {
                    if !doors.is_subset(&keys) {
                        None
                    } else {
                        self.shortest_path(
                            *neighbour,
                            keys | *neighbour,
                            distance_travelled + distance,
                            visited,
                        )
                    }
                })
                .min_by(usize::cmp)
        }
    }
}

pub fn part_a(input: &str) -> usize {
    Map::new(input).make_graph().best_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    static STR_1: &'static str = "#########
#b.A.@.a#
#########";

    static STR_2: &'static str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    static STR_3: &'static str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    static STR_4: &'static str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    static STR_5: &'static str = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    static AT: char = '@';
    static A: char = 'a';
    static B: char = 'b';
    static C: char = 'c';
    static D: char = 'd';
    static E: char = 'e';
    static F: char = 'f';
    static G: char = 'g';
    static H: char = 'h';
    static I: char = 'i';
    static J: char = 'j';
    static K: char = 'k';
    static L: char = 'l';
    static M: char = 'm';
    static N: char = 'n';
    static O: char = 'o';
    static P: char = 'p';

    fn graph_1() -> Graph {
        let nodes = KeySet::nodes_range(B);
        let connections = vec![
            (AT, A, 2, KeySet::new()),
            (AT, B, 4, KeySet::from_nodes(&[A])),
        ];
        Graph::new(nodes, connections)
    }

    fn graph_2() -> Graph {
        let nodes = KeySet::nodes_range(F);
        let connections = vec![
            (AT, A, 2, KeySet::new()),
            (AT, B, 4, KeySet::from_nodes(&[A])),
            (A, C, 4, KeySet::from_nodes(&[B])),
            (B, E, 4, KeySet::from_nodes(&[C])),
            (C, D, 24, KeySet::new()),
            (E, F, 6, KeySet::from_nodes(&[D, E])),
        ];
        Graph::new(nodes, connections)
    }

    fn graph_3() -> Graph {
        let nodes = KeySet::nodes_range(G);
        let connections = vec![
            (AT, A, 2, KeySet::new()),
            (AT, B, 22, KeySet::new()),
            (A, C, 4, KeySet::from_nodes(&[B])),
            (B, F, 6, KeySet::from_nodes(&[C, D])),
            (C, D, 2, KeySet::new()),
            (D, E, 4, KeySet::from_nodes(&[A])),
            (E, G, 4, KeySet::from_nodes(&[F])),
        ];
        Graph::new(nodes, connections)
    }

    fn graph_4() -> Graph {
        let nodes = KeySet::nodes_range(P);
        let connections = vec![
            // starts
            (AT, A, 3, KeySet::new()),
            (AT, B, 3, KeySet::new()),
            (AT, C, 5, KeySet::new()),
            (AT, D, 5, KeySet::new()),
            (AT, E, 5, KeySet::new()),
            (AT, F, 3, KeySet::new()),
            (AT, G, 3, KeySet::new()),
            (AT, H, 5, KeySet::new()),
            // locked edges
            (A, K, 5, KeySet::from_nodes(&[E])),
            (B, J, 5, KeySet::from_nodes(&[A])),
            (C, I, 5, KeySet::from_nodes(&[G])),
            (D, L, 5, KeySet::from_nodes(&[F])),
            (E, P, 5, KeySet::from_nodes(&[H])),
            (F, O, 5, KeySet::from_nodes(&[D])),
            (G, N, 5, KeySet::from_nodes(&[B])),
            (H, M, 5, KeySet::from_nodes(&[C])),
            // intermediate edges
            (A, G, 4, KeySet::new()),
            (A, D, 6, KeySet::new()),
            (A, H, 6, KeySet::new()),
            (B, C, 6, KeySet::new()),
            (B, E, 6, KeySet::new()),
            (B, F, 4, KeySet::new()),
            (C, E, 4, KeySet::new()),
            (C, F, 6, KeySet::new()),
            (D, G, 6, KeySet::new()),
            (D, H, 4, KeySet::new()),
            (E, F, 6, KeySet::new()),
            (G, H, 6, KeySet::new()),
        ];
        Graph::new(nodes, connections)
    }

    fn graph_5() -> Graph {
        let nodes = KeySet::nodes_range(I);
        let connections = vec![
            (AT, D, 3, KeySet::new()),
            (AT, E, 5, KeySet::new()),
            (AT, F, 7, KeySet::new()),
            (AT, A, 15, KeySet::new()),
            (A, C, 1, KeySet::new()),
            (A, D, 14, KeySet::new()),
            (A, E, 12, KeySet::new()),
            (A, F, 10, KeySet::new()),
            (B, C, 5, KeySet::from_nodes(&[G, I])),
            (D, G, 2, KeySet::from_nodes(&[A])),
            (D, E, 4, KeySet::new()),
            (D, F, 6, KeySet::new()),
            (E, H, 2, KeySet::from_nodes(&[B])),
            (E, F, 4, KeySet::new()),
            (F, I, 2, KeySet::from_nodes(&[C])),
        ];
        Graph::new(nodes, connections)
    }

    #[test]
    fn make_graph_1() {
        assert_eq!(graph_1(), Map::new(STR_1).make_graph());
    }

    #[test]
    fn make_graph_2() {
        assert_eq!(graph_2(), Map::new(STR_2).make_graph());
    }

    #[test]
    fn make_graph_3() {
        assert_eq!(graph_3(), Map::new(STR_3).make_graph());
    }

    #[test]
    fn make_graph_4() {
        assert_eq!(graph_4(), Map::new(STR_4).make_graph());
    }

    #[test]
    fn make_graph_5() {
        assert_eq!(graph_5(), Map::new(STR_5).make_graph());
    }

    #[test]
    fn example_1() {
        assert_eq!(graph_1().best_path(), 8);
    }

    #[test]
    fn example_2() {
        assert_eq!(graph_2().best_path(), 86);
    }

    #[test]
    fn example_3() {
        assert_eq!(graph_3().best_path(), 132);
    }

    #[test]
    fn example_4() {
        assert_eq!(graph_4().best_path(), 136);
    }

    #[test]
    fn example_5() {
        assert_eq!(graph_5().best_path(), 81);
    }

    #[test]
    fn real() {
        assert_eq!(part_a(include_str!("input.txt")), 1);
    }
}
