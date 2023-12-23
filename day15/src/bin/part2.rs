use std::{fs::File, io::{BufReader, BufRead}, path::Path};

use itertools::Itertools;

fn compute_hash(input: &str) -> usize {
    input.chars().fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
}

enum Operation<'a> {
    Remove(&'a str),
    Emplace(&'a str, u32)
}

impl<'a> Operation<'a> {
    fn from_str(input: &'a str) -> Self {
        if input.contains("=") {
            let (key, value) = input.split("=").collect_tuple().unwrap();
            Operation::Emplace(key, value.parse().unwrap())
        } else {
            Operation::Remove(&input[0..input.len() - 1])
        }
    }
}

fn main() {
    let lines = lines_from_file("input.txt");

    let mut hash_map: Vec<Vec<(&str, u32)>> = std::iter::repeat(vec![]).take(256).collect::<Vec<_>>();

    let line = lines.join("");

    for command in line.split(",") {
        match Operation::from_str(command) {
            Operation::Remove(input) => {
                let hash = compute_hash(input);
                hash_map[hash].retain(|(x, _)| x != &input)
            },
            Operation::Emplace(input, value) => {
                let hash = compute_hash(input);
                match hash_map[hash as usize].iter_mut().find(|(label, _)| label == &input) {
                    Some((_, x)) => *x = value,
                    None => hash_map[hash as usize].push((input, value))
                }
            }
        }
    }

    let mut focus_power = 0;

    for (box_index, boxx) in hash_map.into_iter().enumerate() {
        for (idx, (_, value)) in boxx.into_iter().enumerate() {
            focus_power += (box_index + 1) * (idx + 1) * value as usize;
        }
    }

    println!("{}", focus_power);

}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}