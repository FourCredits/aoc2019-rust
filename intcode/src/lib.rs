#[derive(Debug)]
pub struct IntcodeComputer {
    pub pc: usize,
    pub halted: bool,
    pub data: Vec<i64>,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
}

impl IntcodeComputer {
    pub fn parse_program(input: &str) -> Vec<i64> {
        input
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    }

    pub fn new(data: Vec<i64>, input: Option<Vec<i64>>) -> IntcodeComputer {
        IntcodeComputer {
            pc: 0,
            halted: false,
            data,
            // Allows us to pop from the vector more easily
            input: if let Some(i) = input {
                i.into_iter().rev().collect()
            } else {
                Vec::new()
            },
            output: Vec::new(),
        }
    }

    pub fn run_program(program: Vec<i64>, input: Option<Vec<i64>>) -> IntcodeComputer {
        let mut computer = IntcodeComputer::new(program, input);
        computer.run();
        computer
    }

    fn step(&mut self) {
        match self.data[self.pc] % 100 {
            1 => self.add(),
            2 => self.mult(),
            3 => self.input(),
            4 => self.output(),
            5 => self.jump_if_true(),
            6 => self.jump_if_false(),
            7 => self.less_than(),
            8 => self.equals(),
            99 => self.halt(),
            _ => unimplemented!(),
        }
    }

    fn add(&mut self) {
        let opcode = self.data[self.pc];
        let parameter1 = match (opcode / 100) % 10 {
            0 => self.data[self.data[self.pc + 1] as usize],
            1 => self.data[self.pc + 1],
            _ => unreachable!(),
        };
        let parameter2 = match (opcode / 1000) % 10 {
            0 => self.data[self.data[self.pc + 2] as usize],
            1 => self.data[self.pc + 2],
            _ => unreachable!(),
        };
        let parameter3 = match opcode / 10000 {
            0 => self.data[self.pc + 3] as usize,
            _ => unreachable!(),
        };
        self.data[parameter3] = parameter1 + parameter2;
        self.pc += 4;
    }

    fn mult(&mut self) {
        let opcode = self.data[self.pc];
        let parameter1 = match (opcode / 100) % 10 {
            0 => self.data[self.data[self.pc + 1] as usize],
            1 => self.data[self.pc + 1],
            _ => unreachable!(),
        };
        let parameter2 = match (opcode / 1000) % 10 {
            0 => self.data[self.data[self.pc + 2] as usize],
            1 => self.data[self.pc + 2],
            _ => unreachable!(),
        };
        let parameter3 = match opcode / 10000 {
            0 => self.data[self.pc + 3] as usize,
            _ => unreachable!(),
        };
        self.data[parameter3] = parameter1 * parameter2;
        self.pc += 4;
    }

    fn input(&mut self) {
        let opcode = self.data[self.pc];
        let destination = match (opcode / 100) % 10 {
            0 => self.data[self.pc + 1] as usize,
            _ => unreachable!(),
        };
        let input = self.input.pop().expect("No more input!");
        self.data[destination] = input;
        self.pc += 2;
    }

    fn output(&mut self) {
        let opcode = self.data[self.pc];
        let output = match (opcode / 100) % 10 {
            0 => self.data[self.data[self.pc + 1] as usize],
            1 => self.data[self.pc + 1],
            _ => unreachable!(),
        };
        self.output.push(output);
        self.pc += 2;
    }

    fn jump_if_true(&mut self) {
        let opcode = self.data[self.pc];
        let parameter1 = match (opcode / 100) % 10 {
            0 => self.data[self.data[self.pc + 1] as usize],
            1 => self.data[self.pc + 1],
            _ => unreachable!(),
        };
        let parameter2 = match (opcode / 1000) % 10 {
            0 => self.data[self.data[self.pc + 2] as usize],
            1 => self.data[self.pc + 2],
            _ => unreachable!(),
        } as usize;
        if parameter1 != 0 {
            self.pc = parameter2;
        } else {
            self.pc += 3;
        }
    }

    fn jump_if_false(&mut self) {
        let opcode = self.data[self.pc];
        let parameter1 = match (opcode / 100) % 10 {
            0 => self.data[self.data[self.pc + 1] as usize],
            1 => self.data[self.pc + 1],
            _ => unreachable!(),
        };
        let parameter2 = match (opcode / 1000) % 10 {
            0 => self.data[self.data[self.pc + 2] as usize],
            1 => self.data[self.pc + 2],
            _ => unreachable!(),
        } as usize;
        if parameter1 == 0 {
            self.pc = parameter2;
        } else {
            self.pc += 3;
        }
    }

    fn less_than(&mut self) {
        let opcode = self.data[self.pc];
        let parameter1 = match (opcode / 100) % 10 {
            0 => self.data[self.data[self.pc + 1] as usize],
            1 => self.data[self.pc + 1],
            _ => unreachable!(),
        };
        let parameter2 = match (opcode / 1000) % 10 {
            0 => self.data[self.data[self.pc + 2] as usize],
            1 => self.data[self.pc + 2],
            _ => unreachable!(),
        };
        let parameter3 = match opcode / 10000 {
            0 => self.data[self.pc + 3] as usize,
            _ => unreachable!(),
        };
        self.data[parameter3] = if parameter1 < parameter2 { 1 } else { 0 };
        self.pc += 4;
    }

    fn equals(&mut self) {
        let opcode = self.data[self.pc];
        let parameter1 = match (opcode / 100) % 10 {
            0 => self.data[self.data[self.pc + 1] as usize],
            1 => self.data[self.pc + 1],
            _ => unreachable!(),
        };
        let parameter2 = match (opcode / 1000) % 10 {
            0 => self.data[self.data[self.pc + 2] as usize],
            1 => self.data[self.pc + 2],
            _ => unreachable!(),
        };
        let parameter3 = match opcode / 10000 {
            0 => self.data[self.pc + 3] as usize,
            _ => unreachable!(),
        };
        self.data[parameter3] = if parameter1 == parameter2 { 1 } else { 0 };
        self.pc += 4;
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let computer = IntcodeComputer::run_program(vec![1, 0, 4, 5, 99, 0], None);
        assert_eq!(computer.data, vec![1, 0, 4, 5, 99, 100]);
    }

    #[test]
    fn mult() {
        let computer = IntcodeComputer::run_program(vec![2, 2, 3, 5, 99, 0], None);
        assert_eq!(computer.data, vec![2, 2, 3, 5, 99, 15]);
    }

    #[test]
    fn halt() {
        let computer = IntcodeComputer::run_program(vec![99], None);
        assert!(computer.halted);
    }

    #[test]
    fn input() {
        let computer = IntcodeComputer::run_program(vec![3, 0, 99], Some(vec![1]));
        assert_eq!(computer.data, vec![1, 0, 99]);
    }

    #[test]
    fn output() {
        let computer = IntcodeComputer::run_program(vec![4, 3, 99, 25], None);
        assert_eq!(computer.output, vec![25]);
    }

    #[test]
    fn immediate_mode() {
        let computer = IntcodeComputer::run_program(vec![1101, 46, 1, 7, 104, 55, 99, 0], None);
        assert_eq!(computer.data, vec![1101, 46, 1, 7, 104, 55, 99, 47]);
        assert_eq!(computer.output, vec![55]);
    }

    #[test]
    fn less_than() {
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let computer = IntcodeComputer::run_program(program.clone(), Some(vec![5]));
        assert_eq!(computer.output, vec![1]);
        let computer = IntcodeComputer::run_program(program, Some(vec![9]));
        assert_eq!(computer.output, vec![0]);
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let computer = IntcodeComputer::run_program(program.clone(), Some(vec![5]));
        assert_eq!(computer.output, vec![1]);
        let computer = IntcodeComputer::run_program(program, Some(vec![9]));
        assert_eq!(computer.output, vec![0]);
    }

    #[test]
    fn equals() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let computer = IntcodeComputer::run_program(program.clone(), Some(vec![8]));
        assert_eq!(computer.output, vec![1]);
        let computer = IntcodeComputer::run_program(program, Some(vec![-1]));
        assert_eq!(computer.output, vec![0]);
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let computer = IntcodeComputer::run_program(program.clone(), Some(vec![8]));
        assert_eq!(computer.output, vec![1]);
        let computer = IntcodeComputer::run_program(program, Some(vec![7]));
        assert_eq!(computer.output, vec![0]);
    }

    #[test]
    fn jump_if_false() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let computer = IntcodeComputer::run_program(program.clone(), Some(vec![0]));
        assert_eq!(computer.output, vec![0]);
        let computer = IntcodeComputer::run_program(program, Some(vec![1]));
        assert_eq!(computer.output, vec![1]);
    }

    #[test]
    fn jump_if_true() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let computer = IntcodeComputer::run_program(program.clone(), Some(vec![0]));
        assert_eq!(computer.output, vec![0]);
        let computer = IntcodeComputer::run_program(program, Some(vec![1]));
        assert_eq!(computer.output, vec![1]);
    }
}
