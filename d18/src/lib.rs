/*
TODO: this is what i've got so far:
- I've also not really done a full project test to see if any refactorings have
  broken anything, so I should probably do that
- I've written a new function in utils called parse_grid, for the typical
  pattern of parsing a grid in terms of a position (a V2) and a character. I
  should look if there's other places I can use that
*/

mod key_set;

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fmt::Debug,
};

use utils::{self, v2::V2};

use key_set::KeySet;

pub fn part_1(input: &str) -> usize {
    parse(input, false).best_path()
}

pub fn part_2(input: &str) -> usize {
    parse(input, true).best_path_multiple_bots()
}

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
fn parse(input: &str, transform_needed: bool) -> Graph {
    let (maze, key_positions, all_keys) = if transform_needed {
        parse_maze(&modify_input(input))
    } else {
        parse_maze(input)
    };
    let edges = find_edges(maze, key_positions);
    Graph::new(all_keys, edges)
}

fn modify_input(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let y_mid = grid.len() / 2;
    let x_mid = grid[0].len() / 2;
    grid[y_mid - 1][x_mid - 1] = '@';
    grid[y_mid - 1][x_mid] = '#';
    grid[y_mid - 1][x_mid + 1] = '@';
    grid[y_mid][x_mid - 1] = '#';
    grid[y_mid][x_mid] = '#';
    grid[y_mid][x_mid + 1] = '#';
    grid[y_mid + 1][x_mid - 1] = '@';
    grid[y_mid + 1][x_mid] = '#';
    grid[y_mid + 1][x_mid + 1] = '@';
    grid.iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

// That said, we do still need the maze in the first place. This produces the
// maze, a map of all the positions of the keys, and a set of all the keys
// contained in the maze
fn parse_maze(input: &str) -> (HashMap<V2, Tile>, BTreeMap<char, V2>, KeySet) {
    let mut maze = HashMap::new();
    let mut keys = BTreeMap::new();
    let mut nodes = KeySet::new();
    let mut start_count = 0;
    for (pos, c) in utils::parse_grid(input) {
        let tile = match c {
            '.' => Tile::Blank,
            '#' => Tile::Wall,
            '@' => {
                start_count += 1;
                Tile::Key((start_count + ('0' as u8)) as char)
            }
            c if c.is_ascii_lowercase() => Tile::Key(c),
            c if c.is_ascii_uppercase() => Tile::Door(c.to_ascii_lowercase()),
            c => unreachable!("{:?}", c),
        };
        if let Tile::Key(c) = tile {
            keys.insert(c, pos);
            nodes |= c;
        }
        maze.insert(pos, tile);
    }
    (maze, keys, nodes)
}

// find the edges between keys, and record the distance between those keys,
// and what doors you need to get through to get from one key to the other.
// the (s, d) pair also has a (d, s) pair with the same value
fn find_edges(
    maze: HashMap<V2, Tile>,
    keys: BTreeMap<char, V2>,
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
        self.shortest_path('1', KeySet::new(), 0, &mut HashMap::new())
            .unwrap()
    }

    fn shortest_path(
        &self,
        start: char,
        keys: KeySet,
        distance_travelled: usize,
        visited: &mut HashMap<(char, KeySet), usize>,
    ) -> Option<usize> {
        if let Some(&previous_call) = visited.get(&(start, keys)) {
            if previous_call < distance_travelled {
                // If we're trying to explore from the same place, with the same
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

    fn best_path_multiple_bots(&self) -> usize {
        self.shortest_path_multiple_bots(
            ['1', '2', '3', '4'],
            0,
            KeySet::new(),
            &mut HashMap::new(),
        )
        .unwrap()
    }

    fn shortest_path_multiple_bots(
        &self,
        bots: [char; 4],
        distance_travelled: usize,
        keys: KeySet,
        visited: &mut HashMap<([char; 4], KeySet), usize>,
    ) -> Option<usize> {
        if let Some(&previous_call) = visited.get(&(bots, keys)) {
            if previous_call < distance_travelled {
                return None;
            }
        }
        if keys == self.nodes {
            return Some(distance_travelled);
        }
        visited.insert((bots, keys), distance_travelled);
        self.next_steps_multiple_bots(bots, &keys)
            .filter_map(|(i, neighbour, distance)| {
                let mut new_bots = bots;
                new_bots[i] = neighbour;
                self.shortest_path_multiple_bots(
                    new_bots,
                    distance_travelled + distance,
                    keys | neighbour,
                    visited,
                )
            })
            .min_by(usize::cmp)
    }

    fn next_steps_multiple_bots<'a>(
        &'a self,
        bots: [char; 4],
        keys: &'a KeySet,
    ) -> impl Iterator<Item = (usize, char, usize)> + '_ {
        bots.into_iter().enumerate().flat_map(move |(i, bot)| {
            self.edges[&bot]
                .iter()
                .filter(|(_, _, doors)| doors.is_subset(keys))
                .map(move |&(neighbour, distance, _)| (i, neighbour, distance))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nodes_range(end_node: char) -> KeySet {
        std::iter::once('1').chain('a'..=end_node).collect()
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

    const START: char = '1';
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
            ((START, A), (2, KeySet::new())),
            ((START, B), (4, KeySet::from_iter([A]))),
        ]);
        Graph::new(nodes, connections)
    }

    fn graph_2() -> Graph {
        let nodes = nodes_range(F);
        let connections = HashMap::from([
            ((START, A), (2, KeySet::new())),
            ((START, B), (4, KeySet::from_iter([A]))),
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
            ((START, A), (2, KeySet::new())),
            ((START, B), (22, KeySet::new())),
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
            ((START, A), (3, KeySet::new())),
            ((START, B), (3, KeySet::new())),
            ((START, C), (5, KeySet::new())),
            ((START, D), (5, KeySet::new())),
            ((START, E), (5, KeySet::new())),
            ((START, F), (3, KeySet::new())),
            ((START, G), (3, KeySet::new())),
            ((START, H), (5, KeySet::new())),
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
            ((START, D), (3, KeySet::new())),
            ((START, E), (5, KeySet::new())),
            ((START, F), (7, KeySet::new())),
            ((START, A), (15, KeySet::new())),
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
            assert_eq!(graph_1(), parse(STR_1, false));
        }

        #[test]
        fn example_2() {
            assert_eq!(graph_2(), parse(STR_2, false));
        }

        #[test]
        fn example_3() {
            assert_eq!(graph_3(), parse(STR_3, false));
        }

        #[test]
        fn example_4() {
            assert_eq!(graph_4(), parse(STR_4, false));
        }

        #[test]
        fn example_5() {
            assert_eq!(graph_5(), parse(STR_5, false));
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

    mod part_2 {
        use super::*;

        fn graph_1() -> Graph {
            let nodes = KeySet::from_iter(['1', '2', '3', '4', 'a', 'b', 'c', 'd']);
            let connections = HashMap::from([
                (('1', 'a'), (2, KeySet::new())),
                (('2', 'd'), (2, KeySet::from_iter(['c']))),
                (('3', 'c'), (2, KeySet::from_iter(['b']))),
                (('4', 'b'), (2, KeySet::from_iter(['a']))),
            ]);
            Graph::new(nodes, connections)
        }

        fn graph_2() -> Graph {
            let nodes = KeySet::from_iter(['1', '2', '3', '4', 'a', 'b', 'c', 'd']);
            let connections = HashMap::from([
                (('1', 'd'), (6, KeySet::from_iter(['c', 'b', 'a']))),
                (('2', 'a'), (6, KeySet::new())),
                (('3', 'b'), (6, KeySet::new())),
                (('4', 'c'), (6, KeySet::new())),
            ]);
            Graph::new(nodes, connections)
        }

        const EX_1: &str = "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######";

        const EX_2: &str = "###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";

        #[test]
        fn parse_example_with_transform() {
            let expected = graph_1();
            let actual = parse(EX_1, true);
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_example_without_transform() {
            let expected = graph_2();
            let actual = parse(EX_2, false);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn real_1() {
        let input = include_str!("input.txt");
        assert_eq!(part_1(input), 4544);
    }

    #[test]
    fn real_2() {
        let input = include_str!("input.txt");
        assert_eq!(part_2(input), 1);
    }
}
