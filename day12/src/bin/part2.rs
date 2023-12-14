use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


// the soultion is strictly based on https://github.com/andypymont/advent2023-rust/blob/main/src/bin/12.rs
// I was not able to solve it myself, but hey, i am learning

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Spring {
    Unknown,
    Hash,
    Dot,
}

struct Analyzer {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Analyzer {
    fn possible_arrangements(&self) -> u64 {
        let mut cache = HashMap::new();
        self.arrangements(&mut cache, 0, 0)
    }

    fn arrangements(&self, cache: &mut HashMap<(usize, usize), u64>, spring_idx: usize, group_idx: usize) -> u64 {
        if let Some(cached) = cache.get(&(spring_idx, group_idx)) {
            return *cached;
        }

        let consumer_group = self.groups.get(group_idx).map_or(0, |group_len| {
            if (spring_idx + group_len) > self.springs.len() {
                return 0;
            }

            if (0..*group_len).any(|i| self.springs.get(spring_idx + i) == Some(&Spring::Dot)) {
                return 0;
            }

            if self.springs.get(spring_idx + group_len) == Some(&Spring::Hash) {
                return 0;
            }

            self.arrangements(cache, spring_idx + group_len + 1, group_idx + 1)
        });

        let skip = match self.springs.get(spring_idx) {
            None => u64::from(group_idx >= self.groups.len()),
            Some(&Spring::Hash) => 0,
            Some(_) => self.arrangements(cache, spring_idx + 1, group_idx),
        };

        let arrangements = consumer_group + skip;
        cache.insert((spring_idx, group_idx), arrangements);

        arrangements
    }

}

fn main() {
    let lines = lines_from_file("input.txt");
    let repeat = 5;

    let springs = lines
        .iter()
        .map(|line| {
            let (spring_str, record_str) = line.split(" ").collect_tuple().unwrap();

            let spring_str_repeated = vec![spring_str; repeat].join("?");
            let records_str_repeated = vec![record_str; repeat].join(",");

            let records = records_str_repeated
                .split(",")
                .map(|r| r.parse::<usize>().unwrap())
                .collect_vec();

            let springs = spring_str_repeated
                .chars()
                .map(|c| match c {
                    '?' => Spring::Unknown,
                    '#' => Spring::Hash,
                    '.' => Spring::Dot,
                    _ => panic!("Unknown char"),
                })
                .collect_vec();

            (springs, records)
        })
        .collect_vec();

    

    let mut sum = 0;
    for ((spring, records), line) in springs.iter().zip(lines) {
        let analyzer = Analyzer {
            springs: spring.clone(),
            groups: records.clone(),
        };

        let cominations = analyzer.possible_arrangements();

        println!("{}: {}", line, cominations);
        
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
