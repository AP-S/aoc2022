use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: i64 = 0;
    let mut elf_index: i64 = 1;
    let mut max_sum_elf_index: i64 = -1;
    let mut max_sums: Vec<i64> = vec![0, 0, 0];

    if let Ok(lines) = read_lines("./elf-input-task.txt") {
        for line in lines {
            if let Ok(ip) = line {                        
                let value = ip.trim().parse::<i64>();
                match value {
                    Ok(parsed_value) => {
                        sum += parsed_value;
                    },
                    Err(e) => {
                        if sum > max_sums[2] {
                            max_sum_elf_index = elf_index;
                        }
                        if sum > max_sums[0] {
                            max_sums[0] = sum;
                            max_sums.sort();
                            println!("max sums ({} {} {})", max_sums[0], max_sums[1], max_sums[2]);
                        }
                        sum = 0;
                        elf_index += 1;
                    }, 
                }  
            }
        }
    }
    
    println!("max sum elf index ({})", max_sum_elf_index);
    println!("max sums ({} {} {})", max_sums[0], max_sums[1], max_sums[2]);
    let total_sum = max_sums[0] + max_sums[1] + max_sums[2];
    println!("total sum of three biggest ({})", total_sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}