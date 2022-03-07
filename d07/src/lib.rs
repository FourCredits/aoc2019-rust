use intcode::IntcodeComputer;

fn permutations<T: Clone>(vals: &[T]) -> Vec<Vec<T>> {
    if vals.is_empty() {
        return vec![vec![]];
    }
    let mut copy = vals.to_owned();
    let mut result = Vec::new();
    for _ in 0..copy.len() {
        if let Some((first, rest)) = copy.split_first() {
            for mut rest_perm in permutations(rest) {
                rest_perm.push(first.clone());
                result.push(rest_perm);
            }
        }
        copy.rotate_left(1);
    }
    result
}

pub fn part_a(input: &str) -> i64 {
    let amplifier_controller_software = IntcodeComputer::parse_program(input);
    permutations(&[0, 1, 2, 3, 4])
        .into_iter()
        .map(|inputs| {
            let mut intermediate_value = 0;
            for phase_setting in inputs {
                let mut computer = IntcodeComputer::new(
                    amplifier_controller_software.clone(),
                    Some(vec![phase_setting, intermediate_value]),
                );
                computer.run();
                intermediate_value = computer.output.pop().unwrap();
            }
            intermediate_value
        })
        .max()
        .unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let software = IntcodeComputer::parse_program(input);
    permutations(&[5, 6, 7, 8, 9])
        .into_iter()
        .map(|permutation| {
            let mut intermediate_value = 0;
            let mut computers: Vec<_> = permutation
                .iter()
                .map(|&input| IntcodeComputer::new(software.clone(), Some(vec![input])))
                .collect();
            while !computers.last().unwrap().halted {
                computers.iter_mut().for_each(|computer| {
                    computer.add_input(intermediate_value);
                    computer.run_until_needs_input();
                    intermediate_value = computer.output.pop().unwrap();
                })
            }
            intermediate_value
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        assert_eq!(
            part_a("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
        assert_eq!(
            part_a("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            54321
        );
        assert_eq!(
            part_a("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
            65210
        );
    }

    #[test]
    fn example_b() {
        assert_eq!(
            part_b("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
            139629729
        );
        assert_eq!(
            part_b("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
            18216
        );
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 47064);
        assert_eq!(part_b(input), 4248984);
    }

    #[test]
    fn permutations_test() {
        assert_eq!(permutations(&[0; 0]), vec![vec![]]);
        assert_eq!(permutations(&[1]), vec![vec![1]]);
        assert_eq!(permutations(&[1, 2]), vec![vec![2, 1], vec![1, 2]]);
        let expected = vec![
            vec![3, 2, 1],
            vec![2, 3, 1],
            vec![1, 3, 2],
            vec![3, 1, 2],
            vec![2, 1, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(permutations(&[1, 2, 3]), expected);
    }
}
