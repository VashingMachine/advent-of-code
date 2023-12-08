use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

use itertools::Itertools;

fn main() {
    let lines = lines_from_file("input.txt");
    let (time, distance) = lines
        .iter()
        .map(|x| {
            x.split(":")
                .last()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect_tuple()
        .unwrap();

    let races = time.iter().zip(distance.iter());

    for (t, s) in races {
        let t = *t as f64;
        let s = *s as f64;

        let delta = (t * t - 4.0 * s).sqrt();
        let v_0 = (t - delta) / 2.0;
        let v_1 = (t + delta) / 2.0;

        println!("v1: {} v2: {}, diff: {}", v_0, v_1, v_1.floor() - v_0.floor());
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
