pub fn run_program(mut program: Vec<i64>) -> Vec<i64> {
    let mut pc = 0;
    let mut halted = false;
    while !halted {
        let opcode = program[pc];
        match opcode {
            1 => {
                let source1 = program[pc + 1] as usize;
                let source2 = program[pc + 2] as usize;
                let destination = program[pc + 3] as usize;
                program[destination] = program[source1] + program[source2];
                pc += 4;
            }
            2 => {
                let source1 = program[pc + 1] as usize;
                let source2 = program[pc + 2] as usize;
                let destination = program[pc + 3] as usize;
                program[destination] = program[source1] * program[source2];
                pc += 4;
            }
            99 => halted = true,
            _ => unimplemented!(),
        }
    }
    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            run_program(vec![1, 0, 4, 5, 99, 0]),
            vec![1, 0, 4, 5, 99, 100]
        );
    }

    #[test]
    fn mult() {
        assert_eq!(
            run_program(vec![2, 2, 3, 5, 99, 0]),
            vec![2, 2, 3, 5, 99, 15]
        );
    }

    #[test]
    fn halt() {
        assert_eq!(run_program(vec![99]), vec![99]);
    }
}
