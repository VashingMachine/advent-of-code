use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn column_value(column: &str) -> u64 {
    let mut wall: Option<usize> = None;
    let mut sum = 0;

    let len = column.len();

    for (idx, char) in column.chars().enumerate() {
        match char {
            'O' => {
                if let Some(wall_idx) = wall {
                    sum += len - wall_idx - 1;
                    wall = Some(wall_idx + 1)
                } else {
                    sum += len;
                    wall = Some(0);
                }
            },
            '#' => {
                wall = Some(idx)
            },
            _ => {}
        };
    }
    sum as u64
}


fn main() {
    let lines = lines_from_file("input.txt");
    let columns = (0..lines[0].len()).map(|idx| {
        lines.iter().map(|line| line.chars().nth(idx).unwrap()).collect::<String>()
    }).collect::<Vec<String>>();

    let mut sum = 0;

    for column in columns {
        sum += column_value(&column);
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