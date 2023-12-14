use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let lines = lines_from_file("input.txt");

    let springs = lines
        .iter()
        .map(|line| {
            let (spring_str, record_str) = line.split(" ").collect_tuple().unwrap();

            let spring_str_five = vec![spring_str; 5].join("?");
            let records_str_five = vec![record_str; 5].join(",");

            let records = records_str_five
                .split(",")
                .map(|r| r.parse::<i64>().unwrap())
                .collect_vec();
            (spring_str_five, records)
        })
        .collect_vec();


    let mut sum = 0;
    for (spring, records) in springs {
        let cominations = find_right_combinations(spring.as_str(), &records);
        println!("{}: {}", spring, cominations);
        sum += cominations;
    }

    println!("Sum: {}", sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"#+").unwrap();
}

fn find_right_combinations(spring: &str, records: &Vec<i64>) -> i64 {
    let unknowns = spring.chars().filter(|c| *c == '?').count();
    let hashs = spring.chars().filter(|c| *c == '#').count();
    let target = records.iter().sum::<i64>();

    if target as i64 - hashs as i64 == 0 {
        return 1;
    }

    let sub_springs = SubSpring::new(
        spring.to_string(),
        target as i64 - hashs as i64,
        unknowns as i64,
    );

    let mut right_combinations = 0;

    for sub_spring in sub_springs {
        if verify_spring(&sub_spring, records) {
            right_combinations += 1;
        }
    }

    right_combinations
}

fn verify_spring(spring: &str, records: &Vec<i64>) -> bool {
    let mut i = 0;

    for m in RE.find_iter(spring) {
        if records.len() <= i {
            return false;
        }

        if records[i] != m.len() as i64 {
            return false;
        }
        i += 1;
    }

    true
}

struct SubSpring {
    spring: String,
    current_subset: i64,
    set_size: i64,
}

impl SubSpring {
    fn new(spring: String, combinations: i64, set_size: i64) -> Self {
        Self {
            spring,
            current_subset: (1 << combinations) - 1,
            set_size,
        }
    }
}

impl Iterator for SubSpring {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_subset & 1 << self.set_size != 0 && self.current_subset != 0 {
            return None;
        }

        let mut q_counter = -1;

        let next_spring = self
            .spring
            .chars()
            .map(|c| {
                if c == '?' {
                    q_counter += 1;
                    if self.current_subset & (1 << q_counter) == 0 {
                        return '.';
                    } else {
                        return '#';
                    }
                }
                return c;
            })
            .collect();

        let lo = self.current_subset & !(self.current_subset - 1);
        let lz = (self.current_subset + lo) & !self.current_subset;

        self.current_subset |= lz;
        self.current_subset &= !(lz - 1);
        self.current_subset |= (lz / lo / 2) - 1;

        Some(next_spring)
    }
}
