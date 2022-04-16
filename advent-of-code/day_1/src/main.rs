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
    
    let values = read_input_file_values(file_path);

    let mut measurement = -1;
    let mut increase_measurements = 0;
    for current_measurement in values {
        if measurement != -1 && current_measurement > measurement { 
            increase_measurements += 1;
        } 
        measurement = current_measurement;
    }
    
    println!("First puzzle result: {}", increase_measurements);
}

fn second_puzzle() {
    let mut file_path = env::current_dir()
        .expect("Current dir is available");
    file_path.push("input_2.txt");
    
    let values = read_input_file_values(file_path);

    let mut measurement = -1;
    let mut increase_measurements = 0;
    for i in 0..values.len()-2 {
        let current_measurement = values[i] + values[i + 1] + values[i + 2];
        if measurement != -1 && current_measurement > measurement {
            increase_measurements += 1;
        } 
        measurement = current_measurement;
    }

    println!("First puzzle result: {}", increase_measurements);
}

fn read_input_file_values(file_path: std::path::PathBuf) -> Vec<i32> {
    let file = File::open(file_path)
        .expect("no such file");
    let buffer = BufReader::new(file);

    return buffer.lines()
        .map(|line| line
            .expect("cannot read line")
            .parse()
            .expect("cannot parse value"))
        .collect();
}