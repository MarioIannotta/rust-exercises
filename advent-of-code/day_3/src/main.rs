use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const BITS_NUMBER: usize = 12;
const FILE_NAME: &str = "input.txt";

fn main() {
    first_puzzle();
    second_puzzle()
}

fn second_puzzle() {

    let mut file_path = env::current_dir().unwrap();
    file_path.push(FILE_NAME);

    let file = File::open(file_path)
        .expect("no such file");
    let buffer = BufReader::new(file);

    let list: Vec<[i8; BITS_NUMBER]> = buffer.lines()
        .map(|line| parse_line(line.unwrap()))
        .collect();

    let oxigen = calculate_parame(&list, true);
    let c02 = calculate_parame(&list, false);

    println!("second puzzle result: {:?}", oxigen * c02);
}

fn calculate_parame(list: &Vec<[i8; BITS_NUMBER]>, take_most: bool) -> i32 {
    let mut filtered_list = list.clone();
    let mut current_index = 0;
    let mut param = 0;
    while current_index < BITS_NUMBER && param == 0 {
        filtered_list = filter_list(&filtered_list, current_index, take_most)
            .into_iter().collect(); // this is required to trim the vector
        current_index += 1;
        if filtered_list.len() == 1 {
            param = bits_to_decimal(filtered_list[0]);
        }
    }
    return param;
}

fn count_ones(list: &Vec<[i8; BITS_NUMBER]>) -> [i32; BITS_NUMBER] {
    let mut ones_count: [i32; BITS_NUMBER] = [0; BITS_NUMBER];
    for row in list {
        for bit_index in 0..ones_count.len() {
            if row[bit_index] == 1 {
                ones_count[bit_index] += 1;
            }
        }
    }
    return ones_count;
}

fn filter_list(list: &Vec<[i8; BITS_NUMBER]>, index: usize, take_most: bool) -> Vec<[i8; BITS_NUMBER]> {
    let ones_count = count_ones(&list);
    let half_list_length: f32 = list.len() as f32 / 2.0;
    let mut filtered_list: Vec<[i8; BITS_NUMBER]> = Vec::new();
    for item in list {
        if take_most {
            if list.len() == 2 && item[index] == 1 {
                filtered_list = vec!(*item);
                break;
            }
            if item[index] == 1 {
                if ones_count[index] as f32 >= half_list_length {
                    filtered_list.push(*item);
                }
            } else {
                if (ones_count[index] as f32) < half_list_length {
                    filtered_list.push(*item);
                }
            }
        } else {
            if list.len() == 2 && item[index] == 0 {
                filtered_list = vec!(*item);
                break;
            }
            if item[index] == 0 {
                if ones_count[index] as f32 >= half_list_length {
                    filtered_list.push(*item);
                }
            } else {
                if (ones_count[index] as f32) < half_list_length {
                    filtered_list.push(*item);
                }
            }
        }
    } 
    return filtered_list;
}

fn first_puzzle() {

    let mut file_path = env::current_dir().unwrap();
    file_path.push(FILE_NAME);

    let file = File::open(file_path)
        .expect("no such file");
    let buffer = BufReader::new(file);

    let  list = buffer.lines()
        .map(|line| parse_line(line.unwrap()));
       
    let mut ones_count: [i32; BITS_NUMBER] = [0; BITS_NUMBER];
    let mut list_size: i32 = 0;
    for row in list {
        for bit_index in 0..ones_count.len() {
            if row[bit_index] == 1 {
                ones_count[bit_index] += 1;
            }
        }
        list_size += 1;
    }

    let mut gamma_values: [i8; BITS_NUMBER] = [0; BITS_NUMBER];
    let mut epsilon_values: [i8; BITS_NUMBER] = [0; BITS_NUMBER];

    for i in 0..ones_count.len() {
        gamma_values[i] = if ones_count[i] > list_size/2 { 1 } else { 0 };
        epsilon_values[i] =  if ones_count[i] < list_size/2 { 1 } else { 0 };
    }
    let gamma = bits_to_decimal(gamma_values);
    let epsilon = bits_to_decimal(epsilon_values);

    println!("first puzzle result: {:?}", gamma * epsilon);
}

fn bits_to_decimal(bits: [i8; BITS_NUMBER]) -> i32 {
    let mut value: i32 = 0;
    for i in 0..bits.len() {
        value += (bits[i] as i32) * (2 as i32).pow((bits.len() - i - 1) as u32);
    }
    return value;
}

fn parse_line(line: String) -> [i8; BITS_NUMBER] {
    let mut result: [i8; BITS_NUMBER] = [0; BITS_NUMBER];
    for i in 0..BITS_NUMBER {
        result[i] = line.chars().nth(i).unwrap().to_digit(10).expect("Cannot get char") as i8
    }
    return result;
}