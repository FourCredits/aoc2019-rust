mod part_a;
mod part_b;

pub use part_a::part_a;
pub use part_b::part_b;

fn bug_rules(alive: bool, num_neighbours: usize) -> bool {
    (alive && num_neighbours == 1) || (!alive && (num_neighbours == 1 || num_neighbours == 2))
}
