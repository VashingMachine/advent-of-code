use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use itertools::Itertools;

struct Rule {
    name: String,
    ranges: Vec<(i64, i64, i64)>,
}

fn main() {
    let lines = lines_from_file("input.txt");
    let sections: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();

    let mut seeds: Vec<i64> = Vec::new();
    let mut rules: Vec<Rule> = Vec::new();

    let mut min_location: Option<i64> = None;

    for section in sections {
        if section.len() == 1 && section[0].starts_with("seeds") {
            let seeds_str = section[0].split(":").collect::<Vec<&str>>()[1];
            seeds = seeds_str
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            continue;
        }

        let rule_name = section[0].split(":").collect::<Vec<&str>>()[0]
            .split(" ")
            .next()
            .unwrap();

        let ranges: Vec<(i64, i64, i64)> = section[1..]
            .iter()
            .map(|line| {
                line.split(" ")
                    .map(|number| number.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64, i64)>()
                    .unwrap()
            })
            .collect();

        rules.push(Rule {
            name: rule_name.to_string(),
            ranges: ranges,
        });
    }

    for seed in seeds {
        let mut point = seed;
        for rule in rules.iter() {
            for range in rule.ranges.iter() {
                if point >= range.1 && point < range.1 + range.2 {
                    point = range.0 + point - range.1;
                    break;
                }
            }
        }

        println!("Seed {}: {}", seed, point);

        match min_location {
            Some(min) => {
                if point < min {
                    min_location = Some(point);
                }
            }
            None => {
                min_location = Some(point);
            }
        }
    }

    println!("min location: {}", min_location.unwrap());
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
