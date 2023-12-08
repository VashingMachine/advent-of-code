use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // File hosts.txt must exist in the current path
    let mut sum = 0;

    let spelledNumbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut leftIndex = ip.len();
                let mut leftValue = 0;
                for (idx, char) in ip.chars().enumerate() {
                    if let Some(number) = char.to_digit(10) {
                        leftValue = number;
                        leftIndex = idx;
                        break;
                    }
                }
                for (i, spelledNumber) in spelledNumbers.iter().enumerate() {
                    if let Some(j) = ip.match_indices(spelledNumber).next() {
                        if j.0 < leftIndex as usize {
                            leftValue = (i as u32) + 1;
                            leftIndex = j.0;
                        }
                    }
                }

                let mut rightIndex = 0;
                let mut rightValue = 0;

                for (idx, char) in ip.chars().rev().enumerate() {
                    if let Some(number) = char.to_digit(10) {
                        rightValue = number;
                        rightIndex = ip.len() - idx - 1;
                        break;
                    }
                }

                for (i, spelledNumber) in spelledNumbers.iter().enumerate() {
                    if let Some(j) = ip.match_indices(spelledNumber).last() {
                        if j.0 > rightIndex as usize {
                            rightValue = (i as u32) + 1;
                            rightIndex = j.0;
                        }
                    }
                }

                sum += leftValue * 10 + rightValue;
                println!("{} <- partial sum", leftValue * 10 + rightValue)
            }
        }
    }
    println!("The total: {} ", sum)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

