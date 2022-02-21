pub fn part_a(input: &str) -> i64 {
    let orbits = parse_orbits(input);
    let mut to_process = vec![("COM", 0)];
    let mut total = 0;
    while !to_process.is_empty() {
        let (body, num_orbits) = to_process.pop().unwrap();
        total += num_orbits;
        orbits
            .iter()
            .filter(|(k, _)| *k == body)
            .for_each(|pair| to_process.push((pair.1, num_orbits + 1)));
    }
    total
}

pub fn part_b(input: &str) -> Option<usize> {
    let orbits = parse_orbits(input);
    let source_parents = all_parents(&orbits, "YOU");
    let target_parents = all_parents(&orbits, "SAN");
    let common_parent = target_parents.iter().find(|p| source_parents.contains(p))?;
    let source_to_common = source_parents.iter().position(|p| p == common_parent)?;
    let common_to_target = target_parents.iter().position(|p| p == common_parent)?;
    Some(source_to_common + common_to_target)
}

fn all_parents<'a>(orbits: &'a [(&str, &str)], object: &'a str) -> Vec<&'a str> {
    std::iter::successors(Some(object), |&child| {
        orbits
            .iter()
            .find_map(|&(k, v)| if v == child { Some(k) } else { None })
    })
    .skip(1)
    .collect()
}

fn parse_orbits(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_once(')').unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        assert_eq!(
            part_a(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
            ),
            42
        );
    }

    #[test]
    fn example_b() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!(part_b(input), Some(4));
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 254447);
        assert_eq!(part_b(input), Some(445));
    }
}
