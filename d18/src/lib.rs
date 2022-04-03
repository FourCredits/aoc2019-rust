use std::collections::{HashMap, HashSet, VecDeque};

use utils::V2;

type Map = HashMap<V2, char>;

pub fn part_a(input: &str) -> u64 {
    let map: Map = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| (V2(i as i64, j as i64), c))
        })
        .collect();
    let position: V2 = *map.iter().find(|(_, &v)| v == '@').unwrap().0;
    best_path(&map, position)
}

fn best_path(map: &Map, position: V2) -> u64 {
    get_available_keys(map, position)
        .iter()
        .map(|&(key_position, distance)| {
            distance + best_path(&unlock_door(map, position, key_position), key_position)
        })
        .min()
        .unwrap_or(0)
}

fn unlock_door(map: &Map, starting_position: V2, key_position: V2) -> Map {
    assert!(matches!(map.get(&key_position), Some(c) if c.is_lowercase()));
    let mut new_map = map.clone();
    new_map.insert(starting_position, '.');
    new_map.insert(key_position, '@');
    if let Some((&unlocked_door, _)) = map
        .iter()
        .find(|(_, &v)| map[&key_position].to_ascii_uppercase() == v)
    {
        new_map.insert(unlocked_door, '.');
        println!("Unlocking door {}", map[&key_position].to_ascii_uppercase());
    }
    new_map
}

// a breadth first-fill, finding all keys available from the starting position
fn get_available_keys(map: &Map, starting_position: V2) -> Vec<(V2, u64)> {
    let mut result = Vec::new();
    let mut queue = VecDeque::from([(starting_position, 0)]);
    let mut explored = HashSet::new();
    while let Some((position, distance)) = queue.pop_front() {
        explored.insert(position);
        if map[&position].is_lowercase() {
            result.push((position, distance));
        }
        for new_position in position.taxicab_directions() {
            if matches!(map.get(&new_position), Some(&c) if c != '#' && !c.is_uppercase())
                && !explored.contains(&new_position)
            {
                queue.push_back((new_position, distance + 1));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "#########
#b.A.@.a#
#########";
        assert_eq!(part_a(input), 8);
    }

    #[test]
    fn example_2() {
        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        assert_eq!(part_a(input), 86);
    }

    #[test]
    fn example_3() {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        assert_eq!(part_a(input), 132);
    }

    #[test]
    fn example_4() {
        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        assert_eq!(part_a(input), 136);
    }

    #[test]
    fn example_5() {
        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        assert_eq!(part_a(input), 81);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 1);
    }
}
