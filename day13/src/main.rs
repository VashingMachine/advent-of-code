use std::{
    fs::File,
    io::{prelude::*, BufReader},
    iter,
    path::Path,
};

use itertools::Itertools;

fn differences(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a != b)
        .count()
}

fn find_mirroring(items: Vec<String>, target_diff: usize) -> Option<usize> {
    for i in 1..items.len() {
        let mut diff = 0;
        for j in 0..i {
            if i + j >= items.len() || i as i32 - j as i32 - 1 < 0 {
                break;
            }

            diff += differences(&items[i + j], &items[i - j - 1]);
            if diff > 1 {
                break;
            }
        }
        if diff == target_diff {
            return Some(i);
        }
    }
    None
}

fn main() {
    let lines = lines_from_file("input.txt");

    let boards = lines
        .split(|line| line.is_empty())
        .map(|lines| lines.to_vec())
        .collect_vec();

    let mut sum = 0;

    for board in boards {
        let rows = board.clone();
        let cols = (0..board[0].len())
            .map(|col| {
                board
                    .iter()
                    .map(|row| row.chars().nth(col).unwrap())
                    .collect::<String>()
            })
            .collect_vec();

        let mut found_row = false;

        if let Some(row) = find_mirroring(rows, 1) {
            println!("row: {}", row);
            sum += row * 100;
            found_row = true;
        }
        if let Some(col) = find_mirroring(cols, 1) {
            println!("col: {}", col);
            sum += col;
            if found_row {
                panic!("both: :(");
            }
        }
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
