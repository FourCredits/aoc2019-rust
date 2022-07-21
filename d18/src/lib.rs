/*
TODO: this is what i've got so far:
- most of the tests work, except for example_4 and example_5
    - as far as i can see, the parsing of the graph is correct
    - so something in the actual graph traversal is incorrect?
- i'd like to make some of the functions a little less right leaning, do some
  good ol' fashioned extracting
- running tests::real doesn't seem to complete in a reasonable amount of time.
  some optimisation required, once everything else is correct...
- I've also not really done a full project test to see if any refactorings have
  broken anything, so I should probably do that
- I've written a new function in utils called parse_grid, for the typical
  pattern of parsing a grid in terms of a position (a V2) and a character. I
  should look if there's other places I can use that
*/

mod key_set;

use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fmt::Debug,
};

use utils::{parse_grid, v2::V2};

use key_set::KeySet;

#[derive(PartialEq, Eq)]
enum Tile {
    Blank,
    Wall,
    Key(char),
    Door(char),
}

// As far as we're concerned when we're trying to solve the problem, the
// actual maze disappears, and we can represent the whole problem as a
// weighted graph between the keys of in the maze, weighted on how long it
// takes to get there, and the doors between the two keys. As such, parsing
// reduces the problem to this graph form.
fn parse(input: &str) -> Graph {
    let (maze, key_positions, all_keys) = parse_maze(input);
    let edges = find_edges(maze, key_positions);
    Graph::new(all_keys, edges)
}

// That said, we do still need the maze in the first place. This produces the
// maze, a map of all the positions of the keys, and a set of all the keys
// contained in the maze
fn parse_maze(input: &str) -> (HashMap<V2, Tile>, HashMap<char, V2>, KeySet) {
    let mut maze = HashMap::new();
    let mut keys = HashMap::new();
    let mut nodes = KeySet::new();
    for (pos, c) in parse_grid(input) {
        let tile = parse_char(c);
        if let Tile::Key(c) = tile {
            keys.insert(c, pos);
            nodes |= c;
        }
        maze.insert(pos, tile);
    }
    (maze, keys, nodes)
}

fn parse_char(c: char) -> Tile {
    match c {
        '.' => Tile::Blank,
        '#' => Tile::Wall,
        c if c.is_ascii_lowercase() || c == '@' => Tile::Key(c),
        c if c.is_ascii_uppercase() => Tile::Door(c.to_ascii_lowercase()),
        _ => unreachable!(),
    }
}

// find the edges between keys, and record the distance between those keys,
// and what doors you need to get through to get from one key to the other.
// the (s, d) pair also has a (d, s) pair with the same value
fn find_edges(
    maze: HashMap<V2, Tile>,
    keys: HashMap<char, V2>,
) -> HashMap<(char, char), (usize, KeySet)> {
    let mut edges = HashMap::new();
    for (source, pos) in keys {
        explore_outwards(&maze, &mut edges, source, pos);
    }
    edges
}

// a form of bfs: start at the source node, exploring outward. Keep track of
// how many doors you have to move through for a given path, as well as how
// far you are from the source key. Once you reach a key (and the source-
// destination pair hasn't already been discovered the other way round), add
// the pair to the list of edges.
fn explore_outwards(
    maze: &HashMap<V2, Tile>,
    edges: &mut HashMap<(char, char), (usize, KeySet)>,
    source: char,
    pos1: V2,
) {
    let mut queue = VecDeque::from([(pos1, 0, KeySet::new())]);
    let mut visited = HashSet::new();
    while let Some((pos2, distance, keys)) = queue.pop_front() {
        visited.insert(pos2);
        pos2.taxicab_directions()
            .into_iter()
            .filter(|n| !visited.contains(n))
            .for_each(|neighbour| match maze.get(&neighbour) {
                Some(&Tile::Key(destination)) if !edges.contains_key(&(destination, source)) => {
                    edges.insert((source, destination), (distance + 1, keys));
                }
                Some(&Tile::Door(new_key)) => {
                    queue.push_back((neighbour, distance + 1, keys | new_key));
                }
                Some(&Tile::Blank) => {
                    queue.push_back((neighbour, distance + 1, keys));
                }
                _ => {}
            });
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    nodes: KeySet,
    edges: HashMap<char, BTreeSet<(char, usize, KeySet)>>,
}

impl Graph {
    fn new(nodes: KeySet, undirected_edges: HashMap<(char, char), (usize, KeySet)>) -> Graph {
        let mut graph = Graph {
            nodes,
            edges: HashMap::new(),
        };
        for ((node1, node2), (distance, doors)) in undirected_edges {
            assert!(nodes.contains(node1));
            assert!(nodes.contains(node2));
            graph.add_edge(node1, node2, distance, doors);
            graph.add_edge(node2, node1, distance, doors);
        }
        graph
    }

    fn add_edge(&mut self, source: char, destination: char, distance: usize, doors: KeySet) {
        self.edges
            .entry(source)
            .or_insert(BTreeSet::new())
            .insert((destination, distance, doors));
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
        // TODO: remove
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
            return Some(distance_travelled);
        }
        visited.insert((start, keys), distance_travelled);
        self.edges
            .get(&start)?
            .iter()
            .filter(|(_, _, doors)| doors.is_subset(&keys))
            .filter_map(|(neighbour, distance, _)| {
                self.shortest_path(
                    *neighbour,
                    keys | *neighbour,
                    distance_travelled + distance,
                    visited,
                )
            })
            .min_by(usize::cmp)
    }
}

pub fn part_a(input: &str) -> usize {
    parse(input).best_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nodes_range(end_node: char) -> KeySet {
        std::iter::once('@').chain('a'..=end_node).collect()
    }

    const STR_1: &'static str = "#########
#b.A.@.a#
#########";

    const STR_2: &'static str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    const STR_3: &'static str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    const STR_4: &'static str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    const STR_5: &'static str = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    const AT: char = '@';
    const A: char = 'a';
    const B: char = 'b';
    const C: char = 'c';
    const D: char = 'd';
    const E: char = 'e';
    const F: char = 'f';
    const G: char = 'g';
    const H: char = 'h';
    const I: char = 'i';
    const J: char = 'j';
    const K: char = 'k';
    const L: char = 'l';
    const M: char = 'm';
    const N: char = 'n';
    const O: char = 'o';
    const P: char = 'p';

    fn graph_1() -> Graph {
        let nodes = nodes_range(B);
        let connections = HashMap::from([
            ((AT, A), (2, KeySet::new())),
            ((AT, B), (4, KeySet::from_iter([A]))),
        ]);
        Graph::new(nodes, connections)
    }

    fn graph_2() -> Graph {
        let nodes = nodes_range(F);
        let connections = HashMap::from([
            ((AT, A), (2, KeySet::new())),
            ((AT, B), (4, KeySet::from_iter([A]))),
            ((A, C), (4, KeySet::from_iter([B]))),
            ((B, E), (4, KeySet::from_iter([C]))),
            ((C, D), (24, KeySet::new())),
            ((E, F), (6, KeySet::from_iter([D, E]))),
        ]);
        Graph::new(nodes, connections)
    }

    fn graph_3() -> Graph {
        let nodes = nodes_range(G);
        let connections = HashMap::from([
            ((AT, A), (2, KeySet::new())),
            ((AT, B), (22, KeySet::new())),
            ((A, C), (4, KeySet::from_iter([B]))),
            ((B, F), (6, KeySet::from_iter([C, D]))),
            ((C, D), (2, KeySet::new())),
            ((D, E), (4, KeySet::from_iter([A]))),
            ((E, G), (4, KeySet::from_iter([F]))),
        ]);
        Graph::new(nodes, connections)
    }

    fn graph_4() -> Graph {
        let nodes = nodes_range(P);
        let connections = HashMap::from([
            // starts
            ((AT, A), (3, KeySet::new())),
            ((AT, B), (3, KeySet::new())),
            ((AT, C), (5, KeySet::new())),
            ((AT, D), (5, KeySet::new())),
            ((AT, E), (5, KeySet::new())),
            ((AT, F), (3, KeySet::new())),
            ((AT, G), (3, KeySet::new())),
            ((AT, H), (5, KeySet::new())),
            // locked edges
            ((A, K), (5, KeySet::from_iter([E]))),
            ((B, J), (5, KeySet::from_iter([A]))),
            ((C, I), (5, KeySet::from_iter([G]))),
            ((D, L), (5, KeySet::from_iter([F]))),
            ((E, P), (5, KeySet::from_iter([H]))),
            ((F, O), (5, KeySet::from_iter([D]))),
            ((G, N), (5, KeySet::from_iter([B]))),
            ((H, M), (5, KeySet::from_iter([C]))),
            // intermediate edges
            ((A, G), (4, KeySet::new())),
            ((A, D), (6, KeySet::new())),
            ((A, H), (6, KeySet::new())),
            ((B, C), (6, KeySet::new())),
            ((B, E), (6, KeySet::new())),
            ((B, F), (4, KeySet::new())),
            ((C, E), (4, KeySet::new())),
            ((C, F), (6, KeySet::new())),
            ((D, G), (6, KeySet::new())),
            ((D, H), (4, KeySet::new())),
            ((E, F), (6, KeySet::new())),
            ((G, H), (6, KeySet::new())),
        ]);
        Graph::new(nodes, connections)
    }

    fn graph_5() -> Graph {
        let nodes = nodes_range(I);
        let connections = HashMap::from([
            ((AT, D), (3, KeySet::new())),
            ((AT, E), (5, KeySet::new())),
            ((AT, F), (7, KeySet::new())),
            ((AT, A), (15, KeySet::new())),
            ((A, C), (1, KeySet::new())),
            ((A, D), (14, KeySet::new())),
            ((A, E), (12, KeySet::new())),
            ((A, F), (10, KeySet::new())),
            ((B, C), (5, KeySet::from_iter([G, I]))),
            ((D, G), (2, KeySet::from_iter([A]))),
            ((D, E), (4, KeySet::new())),
            ((D, F), (6, KeySet::new())),
            ((E, H), (2, KeySet::from_iter([B]))),
            ((E, F), (4, KeySet::new())),
            ((F, I), (2, KeySet::from_iter([C]))),
        ]);
        Graph::new(nodes, connections)
    }

    mod parsing {
        use super::*;

        #[test]
        fn example_1() {
            assert_eq!(graph_1(), parse(STR_1));
        }

        #[test]
        fn example_2() {
            assert_eq!(graph_2(), parse(STR_2));
        }

        #[test]
        fn example_3() {
            assert_eq!(graph_3(), parse(STR_3));
        }

        #[test]
        fn example_4() {
            assert_eq!(graph_4(), parse(STR_4));
        }

        #[test]
        fn example_5() {
            assert_eq!(graph_5(), parse(STR_5));
        }
    }

    mod graph_traversal {
        use super::*;

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
    }

    #[test]
    fn real() {
        assert_eq!(part_a(include_str!("input.txt")), 1);
    }
}
