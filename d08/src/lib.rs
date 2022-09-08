pub fn part_a(width: usize, height: usize, input: &str) -> usize {
    let count = |c1, layer: &[u8]| layer.iter().filter(|&&c2| c1 == c2).count();
    input
        .as_bytes()
        .chunks(width * height)
        .min_by_key(|layer| count(b'0', layer))
        .map(|layer| count(b'1', layer) * count(b'2', layer))
        .unwrap()
}

pub fn part_b(width: usize, height: usize, input: &str) -> String {
    let layers: Vec<_> = input.as_bytes().chunks(width * height).collect();
    (0..width * height)
        .map(|i| {
            layers
                .iter()
                .map(|layer| layer[i])
                .find(|&ch| ch != b'2')
                .map(|ch| if ch == b'0' { " " } else { "#" })
                .unwrap()
                .to_owned()
                + (if (i + 1) % width == 0 { "\n" } else { "" })
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(25, 6, input), 0);
        let output = include_str!("output.txt");
        assert_eq!(part_b(25, 6, input), output);
    }
}
