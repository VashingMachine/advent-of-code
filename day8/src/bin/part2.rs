use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use itertools::Itertools;
use regex::Regex;
use num::integer::lcm;


fn main() {
    let lines = lines_from_file("input.txt");

    let (instructions, nodes) = lines.split(|line| line.is_empty()).collect_tuple().unwrap();

    let mut nodes_map: HashMap<String, [String; 2]> = HashMap::new();
    let mut current_nodes: Vec<String> = vec![];

    let node_re = Regex::new(r"[A-Z]{3}").unwrap();

    for node in nodes {
        let (src, left, right) = node_re
            .find_iter(node)
            .map(|m| m.as_str())
            .collect_tuple()
            .unwrap();
        nodes_map.insert(src.to_string(), [left.to_string(), right.to_string()]);
        if src.ends_with("A") {
            current_nodes.push(src.to_string());
        }
    }

    let mut least_common_multiple = 1;

    for current_node in current_nodes.iter_mut() {
        let mut counter = 0;

        while !current_node.ends_with("Z") {
            for instruction in instructions[0].chars() {
                counter += 1;
                let [left, right] = nodes_map.get(current_node).unwrap();
    
                match instruction {
                    'R' => {
                        *current_node = right.into();
                    },
                    'L' => {
                        *current_node = left.into();
                    },
                    _ => panic!("Unknown instruction"),
                }
    
                if current_node.ends_with("Z") {
                    break;
                }
            }
        }

        let loops = counter / instructions[0].len() as i64;
        println!("Loops: {}", loops);
        println!("Mod: {}", counter % instructions[0].len() as i64);

        least_common_multiple = lcm(least_common_multiple, loops);
    }

    println!("LCM: {}", least_common_multiple);
    println!("Instructions: {}", instructions[0].len() as i64 * least_common_multiple);


}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
