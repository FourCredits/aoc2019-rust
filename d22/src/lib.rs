pub fn part_a(input: &str) -> Option<usize> {
    let shuffles = parse(input)?;
    let deck = run(10_007, shuffles);
    deck.iter().position(|&n| n == 2019)
}

fn run(size: u32, shuffles: Vec<Shuffle>) -> Vec<u32> {
    let mut deck = new_deck(size);
    for shuffle in shuffles {
        shuffle.perform(&mut deck);
    }
    deck
}

fn parse(input: &str) -> Option<Vec<Shuffle>> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<Shuffle> {
    // Is this the best way to do this? Probably not, but my inner functional
    // programmer is very happy at this code
    let option_1 = line
        .strip_prefix("cut ")
        .and_then(|rest| rest.parse().ok())
        .map(|n| Shuffle::Cut(n));
    let option_2 = line
        .strip_prefix("deal with increment ")
        .and_then(|rest| rest.parse().ok())
        .map(|n| Shuffle::Increment(n));
    let option_3 = (line == "deal into new stack").then_some(Shuffle::NewStack);
    option_1.or(option_2).or(option_3)
}

#[derive(PartialEq, Debug)]
enum Shuffle {
    NewStack,
    Cut(i32),
    Increment(usize),
}

impl Shuffle {
    fn perform(self, deck: &mut [u32]) {
        match self {
            Shuffle::NewStack => deck.reverse(),
            Shuffle::Cut(i) => deck.rotate_left(bring_in_range(i, deck.len())),
            Shuffle::Increment(amount) => deal_with_increment(deck, amount),
        }
    }
}

fn deal_with_increment(deck: &mut [u32], increment: usize) {
    let mut new = vec![u32::MAX; deck.len()];
    for (i, card) in deck.iter().enumerate() {
        new[i * increment % deck.len()] = *card;
    }
    debug_assert!(new.iter().all(|&n| n != u32::MAX));
    deck.copy_from_slice(&new);
}

fn bring_in_range(num: i32, denom: usize) -> usize {
    if num < 0 {
        (num + denom as i32) as usize
    } else {
        num as usize
    }
}

fn new_deck(size: u32) -> Vec<u32> {
    (0..size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_test() {
        let deck = run(10, vec![Shuffle::NewStack]);
        assert_eq!(deck, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn cut_test() {
        let deck = run(10, vec![Shuffle::Cut(3)]);
        assert_eq!(deck, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn cut_negative_test() {
        let deck = run(10, vec![Shuffle::Cut(-4)]);
        assert_eq!(deck, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn increment_test() {
        let deck = run(10, vec![Shuffle::Increment(3)]);
        assert_eq!(deck, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn parse_test() {
        let text = "cut -6
deal with increment 7
deal into new stack";
        let shuffles = parse(text);
        let expected = Some(vec![
            Shuffle::Cut(-6),
            Shuffle::Increment(7),
            Shuffle::NewStack,
        ]);
        assert_eq!(shuffles, expected);
    }

    #[test]
    fn examples() {
        let examples = vec![
            "deal with increment 7
deal into new stack
deal into new stack",
            "cut 6
deal with increment 7
deal into new stack",
            "deal with increment 7
deal with increment 9
cut -2",
            "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1",
        ];
        let results = vec![
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
        ];
        for (text, expected) in std::iter::zip(examples, results) {
            let shuffles = parse(text).unwrap();
            let deck = run(10, shuffles);
            assert_eq!(expected, deck);
        }
    }

    // 2808 too low
    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), Some(4086));
    }
}
