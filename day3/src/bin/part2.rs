use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use regex::Regex;

#[derive(Debug)]
struct StarMap {
    height: usize,

    stars: Vec<String>,
}

impl StarMap {
    fn new(stars: Vec<String>) -> StarMap {
        let height = stars.len();

        StarMap { height, stars }
    }

    fn get_numbers_around(&mut self, x: usize, y: usize) -> Vec<i32> {
        let mut numbers = Vec::new();
        let numbers_re = Regex::new(r"\d+").unwrap();

        for line in self.stars[(y - 1).max(0)..(y + 2).min(self.height)].iter() {
            for number in numbers_re.find_iter(line) {
                let start = number.start();
                let end: usize = number.end() - 1;

                if start.abs_diff(x) <= 1 || end.abs_diff(x) <= 1 || start < x && end > x {
                    numbers.push(number.as_str().parse::<i32>().unwrap());
                }
            }
        }

        numbers
    }
}

fn main() {
    let lines = lines_from_file("input.txt");
    let mut star_map = StarMap::new(lines);

    let gear = Regex::new(r"\*").unwrap();

    let mut sum = 0;

    for (y, line) in star_map.stars.clone().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if gear.is_match(&c.to_string()) {
                let numbers = star_map.get_numbers_around(x, y);

                if numbers.len() == 2 {
                    sum += numbers.get(0).unwrap() * numbers.get(1).unwrap();
                }

                println!("{:?}", numbers);
            }
        }
    }

    println!("{:?}", sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
