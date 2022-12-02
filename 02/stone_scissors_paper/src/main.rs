use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const OTHER_ROCK: &str = "A";
const OTHER_PAPER: &str = "B";
const OTHER_SCISSORS: &str = "C";

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

static MY_ROCK_VALUE: i32 = 1;
static MY_PAPER_VALUE: i32 = 2;
static MY_SCISSORS_VALUE: i32 = 3;

static WIN_VALUE: i32 = 6;
static DRAW_VALUE: i32 = 3;
static LOSE_VALUE: i32 = 0;

const LOSE_REQUEST: &str = "X";
const DRAW_REQUEST: &str = "Y";
const WIN_REQUEST: &str = "Z";

static OTHER_CHOICE_INDEX: usize = 0;
// static MY_CHOICE_INDEX: usize = 1;
static REQUEST_INDEX: usize = 1;

fn main() {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines("./input-task.txt") {
        for line in lines {
            if let Ok(ip) = line {  
                //println!("sum ({})", ip);                      
                let split = ip.trim().split_whitespace();
                let values = split.collect::<Vec<&str>>();
                
                if values.len() != 2 {
                    panic!("Bad hair day!");
                }

                let my_choice: &str = my_choice(values[OTHER_CHOICE_INDEX], values[REQUEST_INDEX]);

                sum = sum  
                    + my_choice_value(my_choice)
                    + result_value(values[OTHER_CHOICE_INDEX], my_choice);
            }
        }
    }
    
    println!("sum ({})", sum); 
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn result_value(other_choice: &str, my_choice: &str) -> i32 {
    match other_choice {
        OTHER_ROCK => {
            match my_choice {
                MY_ROCK => {DRAW_VALUE},
                MY_PAPER => {WIN_VALUE},
                MY_SCISSORS => {LOSE_VALUE},
                &_ => panic!("Bad hair day!")
            }
        },
        OTHER_PAPER => {
            match my_choice {
                MY_ROCK => {LOSE_VALUE},
                MY_PAPER => {DRAW_VALUE},
                MY_SCISSORS => {WIN_VALUE},
                &_ => panic!("Bad hair day!")
            }
        },
        OTHER_SCISSORS => {
            match my_choice {
                MY_ROCK => {WIN_VALUE},
                MY_PAPER => {LOSE_VALUE},
                MY_SCISSORS => {DRAW_VALUE},
                &_ => panic!("Bad hair day!")
            }
        },
        &_ => panic!("Bad hair day!") 
    }   
}

fn my_choice_value(my_choice: &str) -> i32 { 
    match my_choice {
        MY_ROCK => {MY_ROCK_VALUE},
        MY_PAPER => {MY_PAPER_VALUE},
        MY_SCISSORS => {MY_SCISSORS_VALUE},
        &_ => panic!("Bad hair day!")
    }
}

fn my_choice<'life>(other_choice: &str, request_type: &str) -> &'life str {
    match other_choice {
        OTHER_ROCK => {
            match request_type {
                LOSE_REQUEST => {MY_SCISSORS},
                DRAW_REQUEST => {MY_ROCK},
                WIN_REQUEST => {MY_PAPER},
                &_ => panic!("Bad hair day!")
            }
        },
        OTHER_PAPER => {
            match request_type {
                LOSE_REQUEST => {MY_ROCK},
                DRAW_REQUEST => {MY_PAPER},
                WIN_REQUEST => {MY_SCISSORS},
                &_ => panic!("Bad hair day!")
            }
        },
        OTHER_SCISSORS => {
            match request_type {
                LOSE_REQUEST => {MY_PAPER},
                DRAW_REQUEST => {MY_SCISSORS},
                WIN_REQUEST => {MY_ROCK},
                &_ => panic!("Bad hair day!")
            }
        },
        &_ => panic!("Bad hair day!") 
    }   
}