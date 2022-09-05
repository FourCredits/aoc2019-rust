/* TODO: I don't have the patience to do this at the moment, but here are my ideas.
 *
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
 * - Last thing: to find the distance between a point and line, use this formula:
 *   <https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line>
 */

mod rectangle;

use std::{
    cell::RefCell,
    collections::{BinaryHeap, HashMap, VecDeque},
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

fn distance_from_line(V2(y0, x0): V2) -> f32 {
    let V2(y1, x1) = V2(0, 0);
    let V2(y2, x2) = V2(1, 1);
    let (x0, x1, x2) = (x0 as f32, x1 as f32, x2 as f32);
    let (y0, y1, y2) = (y0 as f32, y1 as f32, y2 as f32);
    let num = ((x2 - x1) * (y1 - y0)) - ((x1 - x0) * (y2 - y1));
    let denom = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    (num / denom).abs()
}

#[derive(PartialEq, Eq)]
struct Pos(V2);

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Pos(s) = *self;
        let Pos(o) = *other;
        distance_from_line(s)
            .partial_cmp(&distance_from_line(o))
            .unwrap()
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn test(input: &str) -> i64 {
    let reader = TractorBeamReadings::new(input);
    let mut in_beam = VecDeque::from([V2(0, 0)]);
    let mut not_in_beam = BinaryHeap::new();
    let square = |i| {
        iter::once(V2(i, i))
            .chain((0..i).map(move |j| V2(i, j)))
            .chain((0..i).map(move |j| V2(j, i)))
    };
    loop {
        if let Some(pos) = in_beam.pop_front() {
            #[allow(clippy::unnecessary_fold)]
            let answer_found = (1..100).all(|i| {
                square(i)
                    // I deliberately *don't* want the short-circuiting of `all`: I want the side
                    // effects of this call.
                    .map(|offset| {
                        let result = reader.take_reading(pos + offset) == 1;
                        if result {
                            not_in_beam.push(Pos(pos));
                        } else {
                            in_beam.push_back(pos);
                        }
                        result
                    })
                    .fold(true, |acc, is_in_beam| acc && is_in_beam)
            });
            if answer_found {
                return pos.1 * 10_000 + pos.0;
            }
        } else if let Some(Pos(pos)) = not_in_beam.pop() {
            #[allow(clippy::unnecessary_fold)]
            (1..).try_for_each(|i| {
                square(i)
                    // I deliberately *don't* want the short-circuiting of `all`: I want the side
                    // effects of this call.
                    .map(|offset| {
                        let result = reader.take_reading(pos + offset) == 1;
                        if result {
                            not_in_beam.push(Pos(pos));
                        } else {
                            in_beam.push_back(pos);
                        }
                        result
                    })
                    .fold(true, |acc, is_in_beam| acc && !is_in_beam)
                    .then_some(())
            });
        }
    }
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
    fn distance_from_line_test() {
        assert_eq!(distance_from_line(V2(0, 0)), 0.0);
        assert_eq!(distance_from_line(V2(5, 5)), 0.0);
        assert_eq!(distance_from_line(V2(0, 1)), 2.0_f32.sqrt() / 2.0);
        assert!(distance_from_line(V2(4, 1)) - 18.0_f32.sqrt() / 2.0 < 0.000001);
    }

    #[test]
    fn part_a_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 164);
    }

    #[test]
    fn part_b_test() {
        let input = include_str!("input.txt");
        assert_eq!(test(input), 1);
    }
}
