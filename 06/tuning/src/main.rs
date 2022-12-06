use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use itertools::Itertools;

const START_MARKER_LENGTH: usize = 4;
const START_MESSAGE_MARKER_LENGTH: usize = 14;


fn main() {
    let lines = lines_from_file("./input-task.txt");
    
    // part 1
    for line in lines.clone() {
        println!("message start index {}", get_start_index(line, START_MARKER_LENGTH));
    }

    // part 2
    for line in lines.clone() {
        println!("message start index {}", get_start_index(line, START_MESSAGE_MARKER_LENGTH));
    }

}

fn get_start_index(line: String, marker_length: usize) -> usize {
    let mut start_marker = Vec::new();
    for start_index in 0..line.len()-marker_length {
        start_marker = line[start_index..start_index+marker_length].chars().collect();
        if start_marker.into_iter().unique().count() == marker_length {
            return start_index + marker_length;
        }
    }

    return 0;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}