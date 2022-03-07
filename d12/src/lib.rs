use utils::V3;

pub fn part_a(input: &str) -> i64 {
    let mut moons: Vec<_> = input.lines().map(parse_line).collect();
    for _ in 0..1000 {
        tick(&mut moons);
    }
    moons.iter().map(Moon::total_energy).sum()
}

pub fn part_b(input: &str) -> usize {
    let mut moons: Vec<_> = input.lines().map(parse_line).collect();
    let axes: Vec<_> = (0..=2).map(|n| get_axis(&moons, n)).collect();
    let mut repetitions = [None; 3];
    let mut n = 0;
    while repetitions.iter().any(|r| r.is_none()) {
        n += 1;
        tick(&mut moons);
        (0..=2).for_each(|i| {
            if repetitions[i].is_none() && get_axis(&moons, i) == axes[i] {
                repetitions[i] = Some(n);
            }
        });
    }
    repetitions
        .iter()
        .map(|n| n.unwrap())
        .reduce(utils::lcm)
        .unwrap()
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Moon {
    position: V3,
    velocity: V3,
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn apply_gravity(&mut self, other: Moon) {
        self.velocity += (other.position - self.position).signum();
    }

    fn potential_energy(&self) -> i64 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn parse_line(line: &str) -> Moon {
    let line = &line[1..(line.len() - 1)];
    let pos = match line
        .split(", ")
        .map(|dim| dim[2..].parse::<i64>().unwrap())
        .collect::<Vec<_>>()[..]
    {
        [x, y, z] => V3(x, y, z),
        _ => unreachable!(),
    };
    Moon {
        position: pos,
        velocity: V3(0, 0, 0),
    }
}

fn tick(moons: &mut [Moon]) {
    let len = moons.len();
    for i in 0..len - 1 {
        for j in i..len {
            let m1 = moons[i];
            let m2 = moons[j];
            moons[i].apply_gravity(m2);
            moons[j].apply_gravity(m1);
        }
    }
    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}

fn get_axis(moons: &[Moon], axis: usize) -> Vec<(i64, i64)> {
    let f: fn(V3) -> i64 = match axis {
        0 => |v| v.0,
        1 => |v| v.1,
        2 => |v| v.2,
        _ => unreachable!(),
    };
    moons
        .iter()
        .map(|moon| (f(moon.position), f(moon.velocity)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        assert_eq!(part_b(input), 2772);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 7077);
        assert_eq!(part_b(input), 402_951_477_454_512);
    }
}
