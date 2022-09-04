/* TODO: I don't have the patience to do this, but here are my ideas at the moment
 * - I'm almost sure the theory of my method is sound, but it's frightfully inefficient. As such,
 *   What I'm thinking needs to happen is coming up with a more efficient access pattern. `test`
 *   has some of my ideas on this.
 * - I think what might be useful would be to keep a priority queue of points, sorting them by how
 *   close they are to some line (I'm thinking 45 degrees is a good start). We preferentially
 *   choose elements from there (falling back on exhaustive searching) as the potential 'top left'
 *   of the square.
 * - While searching a square, we can add the nodes we *do* care about to the priority queue. This
 *   should help to keep us focused, and avoid deviating too far from the line.
 * - It might be useful to be able to explore in squares:
 *
 *   ```
 *   123
 *   223
 *   333
 *   ```
 *
 * - Finally, rather than completely exhausive searching, we could use the above search pattern,
 *   expanding one layer at a time. We add the on nodes to the queue, and the off ones to a
 *   different queue. Then if we found anything, we go back to searching via the priority queue. If
 *   not, we expand another layer and keep searching.
 * - Also worth reminding, in case you forget, that the way to do a priority queue in Rust is use a
 *   BinaryHeap with elements that have a custom Ord implementation.
 */

mod rectangle;

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    iter,
};

use intcode::IntcodeComputer;
use utils::v2::V2;

use rectangle::Rectangle;

pub fn part_a(input: &str) -> i64 {
    TractorBeamReadings::new(input).sum_square(V2(0, 0), V2(50, 50))
}

pub fn part_b(input: &str) -> i64 {
    let reading_generator = TractorBeamReadings::new(input);
    let santa_ship_size = V2(100, 100);
    nearest_points(V2(0, 0))
        .filter(|&pos| reading_generator.take_reading(pos) == 1)
        .find(|&pos| reading_generator.whole_square(pos, pos + santa_ship_size))
        .map(|V2(y, x)| (x * 10_000) + y)
        .unwrap()
}

fn test(input: &str) -> i64 {
    let reading_generator = TractorBeamReadings::new(input);
    let mut positions = (0..).zip(0..).map(|(y, x)| V2(y, x));
    let mut queue = VecDeque::new();
    'outer: while let Some(position) = queue.pop_front().or_else(|| positions.next()) {
        if reading_generator.take_reading(position) == 1 {
            // search for the square
            let mut solution_found = true;
            for pos in Rectangle::new(position, position + V2(100, 100)).iter_nearest() {
                if reading_generator.take_reading(pos) == 1 {
                    queue.push_back(pos);
                } else {
                    solution_found = false;
                }
            }
            if solution_found {
                return position.1 * 10_000 + position.0;
            }
        } else {
            let V2(y, x) = position;
            // search upwards
            for i in (0..y).rev() {
                if reading_generator.take_reading(V2(i, x)) == 1 {
                    queue.push_back(V2(i, x));
                    continue 'outer;
                }
            }
            // search leftwards
            for i in (0..x).rev() {
                if reading_generator.take_reading(V2(y, i)) == 1 {
                    queue.push_back(V2(y, i));
                    continue 'outer;
                }
            }
        }
    }
    unreachable!();
}

// This is a version of Rectangle::iter_nearest, but you with no upper bound
fn nearest_points(start: V2) -> impl Iterator<Item = V2> {
    iter::successors(Some(0), |n| Some(n + 1)).flat_map(move |distance| {
        iter::successors(Some(V2(distance, 0)), |&position| {
            (position.0 != 0).then_some(position + V2(-1, 1))
        })
        .map(move |position| position + start)
    })
}

struct TractorBeamReadings {
    program: Vec<i64>,
    cache: RefCell<HashMap<V2, i64>>,
}

impl TractorBeamReadings {
    fn new(program: &str) -> TractorBeamReadings {
        TractorBeamReadings {
            program: IntcodeComputer::parse_program(program),
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn take_reading(&self, pos @ V2(y, x): V2) -> i64 {
        *self.cache.borrow_mut().entry(pos).or_insert_with(|| {
            IntcodeComputer::run_program(self.program.clone(), Some(vec![x, y])).output[0]
        })
    }

    fn sum_square(&self, start: V2, end: V2) -> i64 {
        Rectangle::new(start, end)
            .into_iter()
            .map(|pos| self.take_reading(pos))
            .sum()
    }

    fn whole_square(&self, start: V2, end: V2) -> bool {
        Rectangle::new(start, end)
            .iter_nearest()
            .all(|pos| self.take_reading(pos) == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_points_test() {
        let actual: Vec<_> = nearest_points(V2(0, 0)).take(10).collect();
        let expected = vec![
            V2(0, 0),
            V2(1, 0),
            V2(0, 1),
            V2(2, 0),
            V2(1, 1),
            V2(0, 2),
            V2(3, 0),
            V2(2, 1),
            V2(1, 2),
            V2(0, 3),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_a_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 164);
    }

    #[test]
    fn part_b_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 1);
    }
}
