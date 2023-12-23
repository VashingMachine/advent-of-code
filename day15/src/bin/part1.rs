use std::{fs::File, io::{BufReader, BufRead}, path::Path};

fn compute_hash(input: &str) -> u32 {
    input.chars().fold(0, |acc, x| ((acc + x as u32) * 17) % 256)
}

fn main() {
    let lines = lines_from_file("input.txt");

    let mut sum = 0;

    for line in lines {
        for part in line.split(",") {
            sum += compute_hash(&part);
        }
    }

    println!("{}", sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}