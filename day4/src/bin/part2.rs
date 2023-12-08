use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use itertools::Itertools;

fn main() {
    let lines = lines_from_file("./input.txt");
    let mut powers: Vec<usize> = vec![1; lines.len()];

    for (idx, line) in lines_from_file("./input.txt").iter().enumerate() {
        let ip = line.split(":").last().unwrap();

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

        for i in 1..x+1 {
            powers[(idx + i as usize).min(lines.len() - 1)] += powers[idx];
        }
    }

    let sum = powers.iter().sum::<usize>();

    println!("{}", sum);

}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
