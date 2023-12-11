use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn taxi_metric(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    let lines = lines_from_file("input.txt");

    let mut locations: Vec<(i64, i64)> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                locations.push((x as i64, y as i64));
            }
        }
    }

    let mut y_jumps: Vec<i64> = vec![0; lines.len()];

    for (y, line) in lines.iter().enumerate() {
        if line.chars().all(|c| c == '.') && y > 0 {
            y_jumps[y] = y_jumps[y - 1] + 1;
            continue;
        }
        if y > 0 {
            y_jumps[y] = y_jumps[y - 1];
        }
    }

    let mut x_jumps: Vec<i64> = vec![0; lines[0].len()];

    for x in 0..lines[0].len() {
        if lines.iter().all(|line| line.chars().nth(x).unwrap() == '.') && x > 0 {
            x_jumps[x] = x_jumps[x - 1] + 1;
            continue;
        }
        if x > 0 {
            x_jumps[x] = x_jumps[x - 1];
        }
    }

    let mut sum = 0;

    let expand_constant = 999999;

    for (i, a) in locations.iter().enumerate() {
        for b in locations.iter().skip(i + 1) {
            let jump_x = (x_jumps[b.0 as usize] - x_jumps[a.0 as usize]).abs() * expand_constant;
            let jump_y = (y_jumps[b.1 as usize] - y_jumps[a.1 as usize]).abs() * expand_constant;

            let distance = taxi_metric(*a, *b) + jump_x + jump_y;

            println!("D: {:?} {:?} - {}", a, b, distance);

            sum += distance;
        }
    }

    println!("Total: {}", sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
