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

    fn get_input_param(&self, position: u32, parameter: usize) -> i64 {
        let opcode = self.data[self.pc];
        match (opcode / (10i64.pow(position + 1))) % 10 {
            0 => self.data[self.data[self.pc + parameter] as usize],
            1 => self.data[self.pc + parameter],
            _ => unreachable!(),
        }
    }

    fn get_output_param(&self, position: u32, parameter: usize) -> usize {
        let opcode = self.data[self.pc];
        (match (opcode / (10i64.pow(position + 1))) % 10 {
            0 => self.data[self.pc + parameter],
            _ => unreachable!(),
        }) as usize
    }

    fn binary_op<F>(&mut self, f: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        let parameter1 = self.get_input_param(1, 1);
        let parameter2 = self.get_input_param(2, 2);
        let parameter3 = self.get_output_param(3, 3);
        self.data[parameter3] = f(parameter1, parameter2);
        self.pc += 4;
    }

    fn add(&mut self) {
        self.binary_op(|x, y| x + y);
    }

    fn mult(&mut self) {
        self.binary_op(|x, y| x * y);
    }

    fn input(&mut self) {
        let destination = self.get_output_param(1, 1);
        self.data[destination] = self.input.pop().expect("No more input!");
        self.pc += 2;
    }

    fn output(&mut self) {
        self.output.push(self.get_input_param(1, 1));
        self.pc += 2;
    }

    fn jump_if_true(&mut self) {
        if self.get_input_param(1, 1) != 0 {
            self.pc = self.get_input_param(2, 2) as usize;
        } else {
            self.pc += 3;
        }
    }

    fn jump_if_false(&mut self) {
        if self.get_input_param(1, 1) == 0 {
            self.pc = self.get_input_param(2, 2) as usize;
        } else {
            self.pc += 3;
        }
    }

    fn less_than(&mut self) {
        self.binary_op(|x, y| if x < y { 1 } else { 0 });
    }

    fn equals(&mut self) {
        self.binary_op(|x, y| if x == y { 1 } else { 0 });
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    pub fn run(&mut self) {
        while !self.halted {
            // println!("{:?}", self);
            self.step();
        }
    }

    pub fn add_input(&mut self, new_input: i64) {
        self.input.insert(0, new_input);
    }

    pub fn run_until_needs_input(&mut self) {
        while !(self.halted || (self.data[self.pc] % 100 == 3 && self.input.is_empty())) {
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
