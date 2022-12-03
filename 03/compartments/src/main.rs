use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut priority_sum: u32 = 0;

    let mut rucksacks_index: usize = 0;
    let mut rucksacks: Vec<HashSet<char>> = Vec::new();
    rucksacks.push(HashSet::new());
    rucksacks.push(HashSet::new());
    rucksacks.push(HashSet::new());
    let mut common_priority_sum: u32 = 0;
    let mut group_count: usize = 0;

    if let Ok(lines) = read_lines("./input-task.txt") {
        for line in lines {
            if let Ok(rucksack) = line {                       
                let trimmed_rucksack = rucksack.trim();

                // part 1 calculations
                if trimmed_rucksack.len() % 2 != 0 {
                    panic!("Not even compartments")
                }
                let (first_compartment, second_compartment) = trimmed_rucksack.split_at(trimmed_rucksack.len()/2);
                let first_compartment_unique_chars: HashSet<_> = 
                    first_compartment.chars().collect::<Vec<char>>().iter().cloned().collect();
                let second_compartment_unique_chars: HashSet<_> = 
                    second_compartment.chars().collect::<Vec<char>>().iter().cloned().collect();
                println!("first second {} {}", first_compartment, second_compartment);
                for item_id in second_compartment_unique_chars.iter() { 
                    if first_compartment_unique_chars.contains(item_id) {
                        priority_sum += get_char_priority_value(item_id.to_owned());
                    } 
                }
                
                // part 2: calculations
                rucksacks[rucksacks_index] = trimmed_rucksack.chars().collect::<Vec<char>>().iter().cloned().collect();

                if rucksacks_index == 2 {
                    let common_values:HashSet<char> = rucksacks[0].intersection(&rucksacks[1]).copied().collect();
                    let common_item: Vec<_> = common_values.intersection(&rucksacks[2]).copied().collect();
                    if common_item.len() != 1 {
                        panic!("Starting selfdestruction sequence!")
                    }
                    rucksacks_index = 0;
                    common_priority_sum = common_priority_sum + get_char_priority_value(common_item[0]);
                    group_count += 1;
                } else {
                    rucksacks_index += 1;
                }

            }
        }
    }
    println!("priority_sum ({})", priority_sum);   
    println!("common_priority_sum group_count({} {})", common_priority_sum, group_count); 
     
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_char_priority_value(item_id: char) -> u32 {
    let item_id_priority: u32;
    if item_id.is_lowercase() {
        item_id_priority = item_id as u32 - 96
    } else {
        item_id_priority = item_id as u32 - 38
    }
    println!("item_id item_id_priority {} {}", item_id, item_id_priority);

    if item_id_priority < 1 || item_id_priority > 52 {
        panic!("Houston, we had a problem!");
    } 

    return item_id_priority;
}

