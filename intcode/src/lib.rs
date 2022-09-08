use std::collections::HashMap;

#[derive(Debug)]
pub struct IntcodeComputer {
    pub pc: usize,
    pub relative_base: i64,
    pub halted: bool,
    pub data: HashMap<usize, i64>,
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

    pub fn new(data: Vec<i64>, input: Option<Vec<i64>>) -> Self {
        Self {
            pc: 0,
            relative_base: 0,
            halted: false,
            data: data.into_iter().enumerate().collect(),
            // Allows us to pop from the vector more easily
            input: input.map_or_else(Vec::new, |input| input.into_iter().rev().collect()),
            output: Vec::new(),
        }
    }

    pub fn run_program(program: Vec<i64>, input: Option<Vec<i64>>) -> Self {
        let mut computer = Self::new(program, input);
        computer.run();
        computer
    }

    fn step(&mut self) {
        match self.data[&self.pc] % 100 {
            1 => self.add(),
            2 => self.mult(),
            3 => self.input(),
            4 => self.output(),
            5 => self.jump_if_true(),
            6 => self.jump_if_false(),
            7 => self.less_than(),
            8 => self.equals(),
            9 => self.adjust_relative_base(),
            99 => self.halt(),
            _ => unimplemented!(),
        }
    }

    fn get_mem(&self, address: usize) -> i64 {
        *self.data.get(&address).unwrap_or(&0)
    }

    fn read_from_param(&self, position: u32) -> i64 {
        let opcode = self.get_mem(self.pc);
        let parameter_value = self.get_mem(self.pc + position as usize);
        match (opcode / (10i64.pow(position + 1))) % 10 {
            0 => self.get_mem(parameter_value as usize),
            1 => parameter_value,
            2 => self.get_mem((self.relative_base + parameter_value).try_into().unwrap()),
            _ => unreachable!(),
        }
    }

    fn write_to_param(&mut self, position: u32, value_to_write: i64) {
        let opcode = self.get_mem(self.pc);
        let parameter_value = self.get_mem(self.pc + position as usize);
        let address_to_write_to = match (opcode / (10i64.pow(position + 1))) % 10 {
            0 => parameter_value,
            2 => self.relative_base + parameter_value,
            _ => unreachable!(),
        };
        self.data
            .insert(address_to_write_to as usize, value_to_write);
    }

    fn binary_op<F>(&mut self, f: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        let parameter1 = self.read_from_param(1);
        let parameter2 = self.read_from_param(2);
        self.write_to_param(3, f(parameter1, parameter2));
        self.pc += 4;
    }

    fn add(&mut self) {
        self.binary_op(|x, y| x + y);
    }

    fn mult(&mut self) {
        self.binary_op(|x, y| x * y);
    }

    fn input(&mut self) {
        let input_value = self.input.pop().expect("No more input!");
        self.write_to_param(1, input_value);
        self.pc += 2;
    }

    fn output(&mut self) {
        self.output.push(self.read_from_param(1));
        self.pc += 2;
    }

    fn jump_if_true(&mut self) {
        if self.read_from_param(1) == 0 {
            self.pc += 3;
        } else {
            self.pc = self.read_from_param(2) as usize;
        }
    }

    fn jump_if_false(&mut self) {
        if self.read_from_param(1) == 0 {
            self.pc = self.read_from_param(2) as usize;
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

    fn adjust_relative_base(&mut self) {
        self.relative_base += self.read_from_param(1);
        self.pc += 2;
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
        while !(self.halted || (self.get_mem(self.pc) % 100 == 3 && self.input.is_empty())) {
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
        assert_eq!(computer.get_mem(5), 100);
    }

    #[test]
    fn mult() {
        let computer = IntcodeComputer::run_program(vec![2, 2, 3, 5, 99, 0], None);
        assert_eq!(computer.get_mem(5), 15);
    }

    #[test]
    fn halt() {
        let computer = IntcodeComputer::run_program(vec![99], None);
        assert!(computer.halted);
    }

    #[test]
    fn input() {
        let computer = IntcodeComputer::run_program(vec![3, 0, 99], Some(vec![1]));
        assert_eq!(computer.get_mem(0), 1);
    }

    #[test]
    fn output() {
        let computer = IntcodeComputer::run_program(vec![4, 3, 99, 25], None);
        assert_eq!(computer.output, vec![25]);
    }

    #[test]
    fn immediate_mode() {
        let computer = IntcodeComputer::run_program(vec![1101, 46, 1, 7, 104, 55, 99, 0], None);
        assert_eq!(computer.get_mem(7), 47);
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

    #[test]
    fn relative_parameters() {
        let program = vec![204, 1, 99];
        let computer = IntcodeComputer::run_program(program, None);
        assert_eq!(computer.output, vec![1]);
        let program = vec![109, 1, 204, 1, 99];
        let computer = IntcodeComputer::run_program(program, None);
        assert_eq!(computer.output, vec![204]);
    }

    #[test]
    fn quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let computer = IntcodeComputer::run_program(program.clone(), None);
        assert_eq!(computer.output, program);
    }

    #[test]
    fn large_numbers() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let computer = IntcodeComputer::run_program(program, None);
        assert_eq!(computer.output, vec![1219070632396864]);
        let program = vec![104, 1125899906842624, 99];
        let computer = IntcodeComputer::run_program(program, None);
        assert_eq!(computer.output, vec![1125899906842624]);
    }
}
