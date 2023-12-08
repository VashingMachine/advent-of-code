use std::{
    cmp::Ordering,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use itertools::Itertools;

use std::collections::HashMap;

fn score(a: &str) -> i32 {
    let mut hashMap: HashMap<char, i32> = HashMap::new();
    for c in a.chars() {
        let count = hashMap.entry(c).or_insert(0);
        *count += 1;
    }

    if hashMap.values().any(|v| *v == 5) {
        return 7;
    }
    if hashMap.values().any(|v| *v == 4) {
        if let Some(_) = hashMap.get(&'J') {
            return 7
        }
        return 6;
    }
    if hashMap.values().any(|v| *v == 3) && hashMap.values().any(|v| *v == 2) {
        if let Some(_) = hashMap.get(&'J') {
            return 7
        }
        return 5
    }
    if hashMap.values().any(|v| *v == 3) {
        if let Some(n) = hashMap.get(&'J') {
            if n == &3 {
                return 6
            }
            if n == &2 {
                return 7
            }
            if n == &1 {
                return 6
            }
        }
        return 4;
    }
    if hashMap
        .iter()
        .any(|(k, v)| *v == 2 && hashMap.iter().any(|(k2, v2)| *v2 == 2 && k != k2))
    {
        if let Some(n) = hashMap.get(&'J') {
            if n == &2 {
                return 6
            }
            if n == &1 {
                return 5
            }
        }
        return 3;
    }
    if hashMap.values().any(|v| *v == 2) {
        if let Some(_) = hashMap.get(&'J') {
            return 4
        }
        return 2
    }
    if let Some(_) = hashMap.get(&'J') {
        return 2
    }
    return 1;
}

fn compare_symbols(a: &str, b: &str) -> Ordering {
    let ordering: HashMap<char, i32> = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 0),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    for i in 0..a.len() {
        let a = ordering.get(&a.chars().nth(i).unwrap()).unwrap();
        let b = ordering.get(&b.chars().nth(i).unwrap()).unwrap();
        if a != b {
            return a.cmp(b);
        }
    }
    return Ordering::Equal;
}

fn main() {
    let lines = lines_from_file("input.txt");

    let mut hands: Vec<(String, i32)> = lines
        .iter()
        .map(|l| {
            let (hand, score) = l.split_whitespace().collect_tuple().unwrap();
            let score = score.parse::<i32>().unwrap();
            (hand.into(), score)
        })
        .collect();

    hands.sort_by(|a, b| {
        let score_a = score(&a.0);
        let score_b = score(&b.0);
        if score_a == score_b {
            return compare_symbols(&a.0, &b.0);
        }
        return score_a.cmp(&score_b);
    });

    let mut sum = 0;

    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) as i64 * hand.1 as i64;
    }

    println!("{:?}", sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
