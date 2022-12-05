use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const PARCEL_COUNT_INDEX: usize = 1;
const FROM_STACK_INDEX: usize = 3;
const TO_STACK_INDEX: usize = 5;

fn main() {
    let lines = lines_from_file("./input-task.txt");
    let mut stacks = get_stacks(lines.clone());
    let mut commands = get_commands(lines.clone());
    for command in commands {
        // println!("command ({}, {}, {})", command.0, command.1, command.2);
        // for _ in 0..command.0 {
        //     let moved_parcel = stacks[command.1].pop().unwrap();
        //     stacks[command.2].push(moved_parcel);
        // }
        let from_stack_final_length = stacks[command.1].len()- usize::try_from(command.0).unwrap();
        let mut moved_parcels = stacks[command.1].split_off(from_stack_final_length);
        stacks[command.2].append(& mut moved_parcels);
    }

    show_stacks_top(stacks);
}

fn show_stacks_top(stacks: Vec<Vec<char>>) {
    for stack in stacks {
        if stack.len() > 0 {
            print!("{}",stack.last().unwrap());
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_stacks(lines: Vec<String>) -> Vec<Vec<char>> {
    let stack_number_line = lines.clone().into_iter().find(|line| line.trim().starts_with('1')).unwrap();
    println!("stack_number_line {}", stack_number_line);
    let stack_count = stack_number_line.trim().split(' ').filter(|&x| !x.is_empty()).count();
    println!("stack_count {}", stack_count);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    for line in lines {
        if line.trim().starts_with('1') {
            break;
        }
        for stack_index in 0..stack_count {
            let picking_index = stack_index*4+1;
            // println!("picking line {} picking_index {} line.len {}", line, picking_index, line.len());
            let parcel = &line[picking_index..picking_index+1];
            if !parcel.trim().is_empty() {
                stacks[stack_index].push(parcel.chars().next().unwrap());
            }
        }
    }
    for stack_index in 0..stack_count {
        stacks[stack_index].reverse();
    }
    return stacks;
}

fn get_commands(lines: Vec<String>) -> Vec<(u32, usize, usize)> {
    let mut commands = Vec::new();
    for line in lines {
        if line.starts_with("move") {
            let line_params: Vec<&str> = line.split(' ').collect();
            let parcel_count = line_params[PARCEL_COUNT_INDEX].parse::<u32>().unwrap();
            let from_stack_index = line_params[FROM_STACK_INDEX].parse::<usize>().unwrap()-1;
            let to_stack_index = line_params[TO_STACK_INDEX].parse::<usize>().unwrap()-1;
            commands.push((parcel_count, from_stack_index, to_stack_index));
        }
    }
    
    return commands;
}