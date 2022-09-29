use std::io;

use intcode::IntcodeComputer;

fn main() -> io::Result<()> {
    let input = include_str!("input.txt");
    let program = IntcodeComputer::parse_program(input);
    let mut computer = IntcodeComputer::new(program, None);
    loop {
        computer.run_until_needs_input();
        let stdin = io::stdin();
        for n in computer.output.drain(..) {
            print!("{}", n as u8 as char);
        }
        if computer.halted {
            break;
        }
        let mut string = String::new();
        stdin.read_line(&mut string)?;
        for c in string.chars() {
            computer.add_input(c as i64);
        }
    }
    println!("Computer quit!");
    Ok(())
}
