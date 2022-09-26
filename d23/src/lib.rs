use std::{collections::VecDeque, iter};

use intcode::IntcodeComputer;

pub fn part_a(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let mut network = Network::new(program);
    loop {
        network.tick(true);
        if let Some((_, y)) = network.nat {
            return y;
        }
    }
}

pub fn part_b(input: &str) -> i64 {
    let program = IntcodeComputer::parse_program(input);
    let mut network = Network::new(program);
    let mut last_sent = None;
    loop {
        network.tick(false);
        if network.is_idle() {
            if let Some(packet @ (_, y)) = network.nat {
                if last_sent == Some(y) {
                    return y;
                } else {
                    last_sent = Some(y);
                }
                network.buffers[0].push_back(packet);
            }
        }
    }
}

struct Network {
    computers: Vec<IntcodeComputer>,
    buffers: Vec<VecDeque<(i64, i64)>>,
    nat: Option<(i64, i64)>,
}

impl Network {
    fn new(program: Vec<i64>) -> Self {
        Self {
            computers: (0..50)
                .map(|address| IntcodeComputer::new(program.clone(), Some(vec![address])))
                .collect(),
            buffers: (0..50).map(|_| VecDeque::new()).collect(),
            nat: None,
        }
    }

    fn tick(&mut self, stop_after_nat: bool) {
        for (computer, buffer) in iter::zip(self.computers.iter_mut(), self.buffers.iter_mut()) {
            if buffer.is_empty() {
                computer.add_input(-1);
            } else {
                for (x, y) in buffer.drain(..) {
                    computer.add_input(x);
                    computer.add_input(y);
                }
            }
        }
        for computer in self.computers.iter_mut() {
            computer.run_until_needs_input();
            for packet in computer.output.chunks_exact(3) {
                if packet[0] == 255 {
                    self.nat = Some((packet[1], packet[2]));
                    if stop_after_nat {
                        return;
                    }
                } else {
                    self.buffers[packet[0] as usize].push_back((packet[1], packet[2]));
                }
            }
            computer.output.clear();
        }
    }

    fn is_idle(&self) -> bool {
        self.buffers.iter().all(|b| b.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 19530);
        assert_eq!(part_b(input), 12725);
    }
}
