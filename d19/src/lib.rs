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

use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> usize {
    let reader = Drone::new(input);
    (0..50)
        .map(|y| (0..50).filter(|&x| reader.is_in_beam(y, x)).count())
        .sum()
}

// Adapted from https://todd.ginsberg.com/post/advent-of-code/2019/day19/ - also has a good
// explanation of how this works
pub fn part_b(input: &str) -> i64 {
    let reader = Drone::new(input);
    let mut x = 0;
    for y in 0.. {
        while !reader.is_in_beam(y + 99, x) {
            x += 1;
        }
        if reader.is_in_beam(y, x + 99) {
            return x * 10_000 + y;
        }
    }
    unreachable!();
}

struct Drone(Vec<i64>);

impl Drone {
    fn new(program: &str) -> Drone {
        Drone(IntcodeComputer::parse_program(program))
    }

    fn is_in_beam(&self, y: i64, x: i64) -> bool {
        IntcodeComputer::run_program(self.0.clone(), Some(vec![x, y])).output[0] == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 164);
    }

    #[test]
    fn part_b_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 13081049);
    }
}
