use core::num;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn construct_polynomial(ys: &Vec<i128>) -> impl Fn(i128) -> i128 {
    let mut differences: Vec<Vec<i128>> = vec![ys.clone()];

    while !differences.last().unwrap().iter().all(|n| n == &0) {
        differences.push(
            differences
                .last()
                .unwrap()
                .iter()
                .zip(differences.last().unwrap().iter().skip(1))
                .map(|(a, b)| b - a)
                .collect(),
        );
    }

    println!("{:?}", differences);
    
    move |x| {
        let mut result = 0;
        for (diff_idx, diff) in differences[0..differences.len()-1].iter().enumerate() {
            let mut xs = 1;
            for i in 0..diff_idx {
                xs *= x - i as i128;
            }
            xs *= diff[0];
            for i in 1..diff_idx + 1 {
                assert!(xs % i as i128 == 0, "xs: {}, i: {}", xs, i);
                xs /= i as i128;
            }
            result += xs;
        }
        result
    }
}

fn main() {
    let lines = lines_from_file("input.txt");
    let line_numbers: Vec<Vec<i128>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i128>().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;

    for numbers in line_numbers {
        let x = construct_polynomial(&numbers)(numbers.len() as i128);
        sum += x;
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
