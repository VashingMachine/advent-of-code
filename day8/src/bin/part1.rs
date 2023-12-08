use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path, collections::HashMap,
};

use itertools::Itertools;
use regex::Regex;


fn main() {
    let lines = lines_from_file("input.txt");

    let (instructions, nodes) = lines.split(|line| line.is_empty()).collect_tuple().unwrap();

    let mut nodes_map: HashMap<String, [String; 2]> = HashMap::new();

    let node_re = Regex::new(r"[A-Z]{3}").unwrap();

    for node in nodes {
        let (src, left, right) = node_re.find_iter(node).map(|m| m.as_str()).collect_tuple().unwrap();
        nodes_map.insert(src.to_string(), [left.to_string(), right.to_string()]);
    }

    let mut current_node = "AAA";
    let mut counter = 0;

    while current_node != "ZZZ" {
        for instruction in instructions[0].chars() {
            counter += 1;
            let [left, right] = nodes_map.get(current_node).unwrap();

            match instruction {
                'R' => {
                    current_node = right;
                },
                'L' => {
                    current_node = left;
                },
                _ => panic!("Unknown instruction"),
            }

            if current_node == "ZZZ" {
                break;
            }
        }
    }
    println!("Instruction len {}", instructions[0].len());
    println!("Part 1: {}", counter);
    println!("Divided: {}", counter / instructions[0].len() as i64);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
