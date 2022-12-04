use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut overlapping_count: u32 = 0;
    let mut overlapping_partly_count: u32 = 0;

    if let Ok(lines) = read_lines("./input-task.txt") {
        for line in lines {
            if let Ok(area_pair) = line {
                let (first_area, second_area) = area_pair.split_once(',').unwrap();
                let (first_area_min, first_area_max) = first_area.split_once('-').unwrap();
                let (second_area_min, second_area_max) = second_area.split_once('-').unwrap();
                
                let first_area_limits = get_area_limits((first_area_min, first_area_max));
                let second_area_limits = get_area_limits((second_area_min, second_area_max));

                // part 1 calculation
                if is_areas_overlapping(first_area_limits, second_area_limits) {
                    overlapping_count += 1;
                }

                //part 2 calculation
                if is_areas_overlapping_partly(first_area_limits, second_area_limits) {
                    overlapping_partly_count += 1;
                }
                 
            }
        }
    }

    println!("overlapping_count {}", overlapping_count);
    println!("overlapping_partly_count {}", overlapping_partly_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_area_limits(area_limit_pair: (&str, &str)) -> (u32, u32) {
    // println!("area limits {} {}", area_limit_pair.0, area_limit_pair.1);
    let min_limit_result = area_limit_pair.0.trim().parse::<u32>();
    match min_limit_result {
        Ok(min_limit) => {
            let max_limit_result = area_limit_pair.1.trim().parse::<u32>();
            match max_limit_result {
                Ok(max_limit) => {
                    return (min_limit, max_limit);
                },
                Err(e) => {
                    panic!("max_limit parse error {}", e);
                }
            }
        },
        Err(e) => {
            panic!("min_limit parse error {}", e);
        }
    }
}

fn is_areas_overlapping(first_area: (u32, u32), second_area: (u32, u32)) -> bool {
    if first_area.0 >= second_area.0 && first_area.1 <= second_area.1 {
        return true;
    } else if second_area.0 >= first_area.0 && second_area.1 <= first_area.1 {
        return true;
    }

    return false;   
}

fn is_areas_overlapping_partly(first_area: (u32, u32), second_area: (u32, u32)) -> bool {
    if first_area.0 >= second_area.0 && first_area.0 <= second_area.1 {
        return true;
    } else if first_area.1 >= second_area.0 && first_area.1 <= second_area.1 {
        return true;
    }
    else if second_area.0 >= first_area.0 && second_area.0 <= first_area.1 {
        return true;
    }
    else if second_area.1 >= first_area.0 && second_area.1 <= first_area.1 {
        return true;
    }

    return false;   
}
