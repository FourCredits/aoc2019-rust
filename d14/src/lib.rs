use std::collections::HashMap;

type Reactions<'a> = HashMap<&'a str, (i64, Vec<(&'a str, i64)>)>;

pub fn part_a(input: &str) -> i64 {
    ore_needed(&parse(input), 1)
}

pub fn part_b(input: &str) -> i64 {
    // binary search
    let reactions = parse(input);
    let target = 1_000_000_000_000;
    let (mut low, mut high) = find_bounds(&reactions, target);
    loop {
        let mid = (low + high) / 2;
        let ore = ore_needed(&reactions, mid);
        if low >= (high - 1) || ore == target {
            return mid;
        } else if ore < target {
            low = mid;
        } else if ore > target {
            high = mid;
        }
    }
}

fn find_bounds(reactions: &Reactions, target: i64) -> (i64, i64) {
    let low = target / ore_needed(reactions, 1);
    let mut high = low * 10;
    while ore_needed(reactions, high) < target {
        high *= 10;
    }
    (low, high)
}

fn ore_needed(reactions: &Reactions, amount: i64) -> i64 {
    let mut needed: HashMap<&str, i64> = HashMap::from([("FUEL", amount)]);
    let mut leftovers = HashMap::new();
    while let Some((&c, &a)) = needed.iter().filter(|&(&c, _a)| c != "ORE").next() {
        needed.remove(c);
        for (c, a) in chemicals_needed(reactions, c, a, &mut leftovers) {
            *needed.entry(c).or_insert(0) += a;
        }
    }
    needed["ORE"]
}

fn chemicals_needed<'a>(
    reactions: &Reactions<'a>,
    chemical: &'a str,
    amount: i64,
    leftovers: &mut HashMap<&'a str, i64>,
) -> HashMap<&'a str, i64> {
    let (amount_produced, inputs) = reactions[&chemical].clone();
    let starting_amount = *leftovers.get(&chemical).unwrap_or(&0);
    let num_reactions =
        (((amount - starting_amount) as f64) / amount_produced as f64).ceil() as i64;
    let needed: HashMap<&str, i64> = inputs
        .iter()
        .map(|(c, a)| (*c, a * num_reactions))
        .collect();
    *leftovers.entry(chemical).or_insert(0) += (num_reactions * amount_produced) - amount;
    needed
}

fn parse(input: &str) -> Reactions {
    input
        .lines()
        .map(|line| {
            let (inputs, output) = line.split_once(" => ").unwrap();
            let (out_chemical, out_amount) = parse_element(output);
            let inputs = inputs.split(", ").map(parse_element).collect();
            (out_chemical, (out_amount, inputs))
        })
        .collect()
}

fn parse_element(s: &str) -> (&str, i64) {
    let (amount, chemical) = s.split_once(" ").unwrap();
    (chemical, amount.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        assert_eq!(part_a(input), 165);
    }

    #[test]
    fn test() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 1582325);
        assert_eq!(part_b(input), 2267486);
    }
}
