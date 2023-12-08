use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let ip = ip.split(":").last().unwrap();

                let (winning, attempt) = ip.split("|").collect_tuple().unwrap();
                let winning_numbers = winning
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let attempt_numbers = attempt
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let x = attempt_numbers
                    .iter()
                    .map(|x| winning_numbers.contains(x) as i32)
                    .sum::<i32>();

                if x > 0 {
                    sum += 1 << (x - 1);
                }
            }
        }
    }

    println!("{}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}