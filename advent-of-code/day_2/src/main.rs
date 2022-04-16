use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    first_puzzle();
    second_puzzle();
}

fn first_puzzle() {
    let mut file_path = env::current_dir()
        .expect("Current dir is available");
    file_path.push("input_1.txt");
    
    let values = read_input_file_moves(file_path);
    
    let mut horizional_position: i16 = 0;
    let mut depth: i16 = 0;
    for value in values {
        match value {
            Move::Forward(value) => horizional_position += value as i16,
            Move::Down(value) => depth += value as i16,
            Move::Up(value) => depth -= value as i16,
        }
    }

    let result = horizional_position as i32 * depth as i32;
    println!("First puzzle result: {}", result);
}

fn second_puzzle() {
    let mut file_path = env::current_dir()
        .expect("Current dir is available");
    file_path.push("input_1.txt");

    let values = read_input_file_moves(file_path);

    let mut aim: i32 = 0;
    let mut horizional_position: i32 = 0;
    let mut depth: i32 = 0;
    for value in values {
        match value {
            Move::Forward(value) => {
                horizional_position += value as i32;
                depth += value as i32 * aim as i32;
            },
            Move::Down(value) => aim += value as i32,
            Move::Up(value) => aim -= value as i32,
        }
    }

    let result = horizional_position * depth;
    println!("Second puzzle result: {}", result);
}

fn read_input_file_moves(file_path: std::path::PathBuf) -> Vec<Move> {
    let file = File::open(file_path)
        .expect("no such file");
    let buffer = BufReader::new(file);

    return buffer.lines()
        .map(|line| Move::from_command(line.expect("Cannot parse command")).expect("Cannot parse move"))
        .collect();
}

#[derive(Debug)]
enum Move {
    Forward(i8), 
    Down(i8), 
    Up(i8)
}

impl Move {
    fn from_command(command: String) -> Option<Move> {
        let components: Vec<&str> = command.split(" ").collect();
        if components.len() != 2 {
            return None;
        } else {
            let direction = components[0];
            let value: i8 = components[1].parse().expect("Cannot parse move value");
            return match direction {
                "forward" => return Some(Move::Forward(value)),
                "down" => return Some(Move::Down(value)),
                "up" => return Some(Move::Up(value)),
                _ => None
            }
        }
    }
}