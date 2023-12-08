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

    // set to max i64 value
    let mut min_location: i64 = i64::MAX;

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

        let mut ranges: Vec<(i64, i64, i64)> = section[1..]
            .iter()
            .map(|line| {
                line.split(" ")
                    .map(|number| number.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64, i64)>()
                    .unwrap()
            })
            .collect();

        ranges.sort_by(|a, b| a.1.cmp(&b.1));

        rules.push(Rule {
            name: rule_name.to_string(),
            ranges: ranges,
        });
    }


    let mut counter: i64 = 0;
    for seed_chunk in seeds.chunks(2) {
        println!("seed chunk: {:?}", seed_chunk);
        for seed in seed_chunk[0]..seed_chunk[0] + seed_chunk[1] {
            counter += 1;
            if counter % 1000000 == 0 {
                println!("counter: {} ({})%", counter, counter as f64 / seed_chunk[1] as f64 * 100.0);
            }
            let mut point = seed;

            for rule in rules.iter() {
                let location = rule.ranges.binary_search_by(|range| range.1.cmp(&point));
                
                match location {
                    Ok(n) => {
                        let r = rule.ranges[n];
                        if point >= r.1 && point < r.1 + r.2 {
                            point = r.0 + point - r.1;
                            continue;
                        }
                    }
                    Err(n) => {
                        if n == 0 {
                            continue;
                        }
                        let r = rule.ranges[n - 1];
                        if point >= r.1 && point < r.1 + r.2 {
                            point = r.0 + point - r.1;
                            continue;
                        }
                    }
                }
            }

            min_location = min_location.min(point);
        }
    }

    println!("min location: {}", min_location);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
