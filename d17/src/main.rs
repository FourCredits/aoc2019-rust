use std::env;

use intcode::IntcodeComputer;

use d17::{find_path, make_map};

fn main() {
    let input = include_str!("input.txt");
    let program = IntcodeComputer::parse_program(input);
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "map" => print_map(program),
        "path" => print_path(program),
        "both" => {
            print_map(program.clone());
            print_path(program);
        }
        _ => panic!("No other things to do"),
    }
}

#[allow(dead_code)]
fn print_path(program: Vec<i64>) {
    let map = make_map(program);
    let path = find_path(&map);
    println!("{:?}", path);
    println!("{:?}", path.len());
}

#[allow(dead_code)]
fn print_map(program: Vec<i64>) {
    let result: String = IntcodeComputer::run_program(program, None)
        .output
        .iter()
        .map(|&n| char::from_u32(n.try_into().unwrap()).unwrap())
        .collect();
    println!("{}", result);
}
