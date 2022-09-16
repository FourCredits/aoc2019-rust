use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

// bfs {{{1
pub fn bfs<Node, NeighbourFn, DoneFn>(
    neighbours: NeighbourFn,
    done: DoneFn,
    start: Node,
) -> Option<Vec<Node>>
where
    NeighbourFn: Fn(&Node) -> Vec<Node>,
    DoneFn: Fn(&Node) -> bool,
    Node: Hash + Eq + Copy,
{
    let mut queue = VecDeque::from([start]);
    let mut predecessors = HashMap::new();
    let mut visited = HashSet::new();
    while let Some(pos) = queue.pop_front() {
        visited.insert(pos);
        if done(&pos) {
            let mut result = Vec::from([pos]);
            let mut pos = pos;
            while let Some(&pred) = predecessors.get(&pos) {
                result.push(pred);
                pos = pred;
            }
            result.reverse();
            return Some(result);
        }
        for neighbour in neighbours(&pos) {
            if !visited.contains(&neighbour) {
                queue.push_back(neighbour);
                predecessors.insert(neighbour, pos);
            }
        }
    }
    None
}

// tests {{{1
mod tests {

    // test 1 {{{2
    #[test]
    fn bfs_test_1() {
        use super::*;
        use crate::v2::V2;
        let map = [[1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1]];
        let start_pos = V2(1, 1);
        let neighbours = |&pos: &V2| {
            pos.taxicab_neighbours()
                .into_iter()
                .filter(|&V2(x, y)| {
                    x >= 0
                        && y >= 0
                        && x < map.len() as i64
                        && y < map[0].len() as i64
                        && map[x as usize][y as usize] == 0
                })
                .collect()
        };
        let done = |&pos: &V2| pos == V2(1, 3);
        let result = bfs(neighbours, done, start_pos);
        assert_eq!(result, Some(vec![V2(1, 1), V2(1, 2), V2(1, 3)]));
    }

    // test 2 {{{2
    #[test]
    fn bfs_test_2() {
        use super::*;
        use crate::v2::V2;
        let map = [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 1, 1, 1],
        ];
        let start_pos = V2(3, 1);
        let neighbours = |&pos: &V2| {
            pos.taxicab_neighbours()
                .into_iter()
                .filter(|&V2(x, y)| {
                    x >= 0
                        && y >= 0
                        && x < map.len() as i64
                        && y < map[0].len() as i64
                        && map[x as usize][y as usize] == 0
                })
                .collect()
        };
        let done = |&pos: &V2| pos == V2(3, 3);
        let result = bfs(neighbours, done, start_pos);
        assert_eq!(
            result,
            Some(vec![
                V2(3, 1),
                V2(2, 1),
                V2(1, 1),
                V2(1, 2),
                V2(1, 3),
                V2(2, 3),
                V2(3, 3)
            ])
        );
    }

    // test 3 {{{2
    #[test]
    fn bfs_test_3() {
        use super::*;
        use crate::v2::V2;
        let map = [
            [1, 1, 1, 1, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 1, 1, 1],
        ];
        let start_pos = V2(3, 1);
        let neighbours = |&pos: &V2| {
            pos.taxicab_neighbours()
                .into_iter()
                .filter(|&V2(x, y)| {
                    x >= 0
                        && y >= 0
                        && x < map.len() as i64
                        && y < map[0].len() as i64
                        && map[x as usize][y as usize] == 0
                })
                .collect()
        };
        let done = |&pos: &V2| pos == V2(3, 3);
        let result = bfs(neighbours, done, start_pos);
        assert_eq!(result, None);
    }
}
