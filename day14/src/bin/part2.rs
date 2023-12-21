use std::{
    collections::HashMap,
    fs::File,
    hash::{Hash, Hasher},
    io::{prelude::*, BufReader},
    path::Path,
};

use std::fs::OpenOptions;
use std::io::Write;

use std::collections::hash_map::DefaultHasher;

fn hash_vector<T: Hash>(vec: &[T]) -> u64 {
    let mut hasher = DefaultHasher::new();
    let mut combined_hash = 0;

    for item in vec {
        let mut item_hasher = DefaultHasher::new();
        item.hash(&mut item_hasher);
        combined_hash ^= item_hasher.finish(); // XOR operation for combination
    }

    combined_hash.hash(&mut hasher);
    hasher.finish()
}

// for debug purposes
#[allow(dead_code)]
fn print_table(
    stones: &Vec<(usize, usize)>,
    hashes: &Vec<(usize, usize)>,
    width: usize,
    height: usize,
) {
    let mut table = vec![vec!['.'; width]; height];

    for (x, y) in stones {
        table[*y][*x] = 'O';
    }

    for (x, y) in hashes {
        table[*y][*x] = '#';
    }

    for row in table {
        let str = row.iter().collect::<String>();
        println!("{}", str);
    }
    println!("------------------");
}

// for debug purposes
#[allow(dead_code)]
fn print_table_to_file(
    stones: &Vec<(usize, usize)>,
    hashes: &Vec<(usize, usize)>,
    width: usize,
    height: usize,
    filename: &str,
) {
    let mut table = vec![vec!['.'; width]; height];

    for (x, y) in stones {
        table[*y][*x] = 'O';
    }

    for (x, y) in hashes {
        table[*y][*x] = '#';
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .expect("Could not open file");

    for row in table {
        let str = row.iter().collect::<String>();
        file.write_all(str.as_bytes())
            .expect("Could not write to file");
        file.write_all("\n".as_bytes())
            .expect("Could not write to file");
    }
    file.write_all("\n".as_bytes())
        .expect("Could not write to file");
}

fn move_stones_up(stones: &mut Vec<(usize, usize)>, columns: &HashMap<usize, Vec<usize>>) {
    let mut used_hashes = HashMap::new();

    for (x, y) in stones.iter_mut() {
        if let Some(y_row) = columns.get(x) {
            match y_row.binary_search(y) {
                Ok(_) => {
                    panic!("Stone should not be here");
                }
                Err(idx) => {
                    let y_idx = if idx == 0 { -1 } else { y_row[idx - 1] as i32 };
                    *used_hashes.entry((*x as i32, y_idx as i32)).or_insert(0) += 1;

                    let value = used_hashes[&(*x as i32, y_idx as i32)];

                    if y_idx < 0 {
                        *y = value - 1;
                    } else {
                        *y = y_idx as usize + value;
                    }
                }
            }
        } else {
            *used_hashes.entry((*x as i32, -1)).or_insert(0) += 1;
            let value = used_hashes[&(*x as i32, -1)];
            *y = value - 1;
        }
    }
}

fn move_stones_down(
    stones: &mut Vec<(usize, usize)>,
    columns: &HashMap<usize, Vec<usize>>,
    height: usize,
) {
    let mut used_hashes = HashMap::new();

    for (x, y) in stones.iter_mut() {
        if let Some(y_row) = columns.get(x) {
            match y_row.binary_search(y) {
                Ok(_) => {
                    panic!("Stone should not be here");
                }
                Err(idx) => {
                    let y_idx = if idx == y_row.len() {
                        height as i32
                    } else {
                        y_row[idx] as i32
                    };
                    *used_hashes.entry((*x as i32, y_idx as i32)).or_insert(0) += 1;

                    let value = used_hashes[&(*x as i32, y_idx as i32)];

                    *y = y_idx as usize - value;
                }
            }
        } else {
            *used_hashes.entry((*x as i32, height as i32)).or_insert(0) += 1;
            let value = used_hashes[&(*x as i32, height as i32)];
            *y = height - value;
        }
    }
}

fn move_stones_left(stones: &mut Vec<(usize, usize)>, rows: &HashMap<usize, Vec<usize>>) {
    let mut used_hashes = HashMap::new();

    for (x, y) in stones.iter_mut() {
        if let Some(x_row) = rows.get(y) {
            match x_row.binary_search(x) {
                Ok(_) => {
                    panic!("Stone should not be here");
                }
                Err(idx) => {
                    let x_idx = if idx == 0 { -1 } else { x_row[idx - 1] as i32 };
                    *used_hashes.entry((x_idx, *y as i32)).or_insert(0) += 1;

                    let value = used_hashes[&(x_idx, *y as i32)];

                    if x_idx < 0 {
                        *x = value - 1;
                    } else {
                        *x = x_idx as usize + value;
                    }
                }
            }
        } else {
            *used_hashes.entry((-1, *y as i32)).or_insert(0) += 1;
            let value = used_hashes[&(-1, *y as i32)];
            *x = value - 1;
        }
    }
}

fn move_stones_right(
    stones: &mut Vec<(usize, usize)>,
    rows: &HashMap<usize, Vec<usize>>,
    width: usize,
) {
    let mut used_hashes = HashMap::new();

    for (x, y) in stones.iter_mut() {
        if let Some(x_row) = rows.get(y) {
            match x_row.binary_search(x) {
                Ok(_) => {
                    panic!("Stone should not be here");
                }
                Err(idx) => {
                    let x_idx = if idx == x_row.len() {
                        width as i32
                    } else {
                        x_row[idx] as i32
                    };
                    *used_hashes.entry((x_idx, *y as i32)).or_insert(0) += 1;

                    let value = used_hashes[&(x_idx, *y as i32)];

                    *x = x_idx as usize - value;
                }
            }
        } else {
            *used_hashes.entry((width as i32, *y as i32)).or_insert(0) += 1;
            let value = used_hashes[&(width as i32, *y as i32)];
            *x = width - value;
        }
    }
}

fn cycle(
    stones: &mut Vec<(usize, usize)>,
    rows: &HashMap<usize, Vec<usize>>,
    columns: &HashMap<usize, Vec<usize>>,
    height: usize,
    width: usize,
) {
    move_stones_up(stones, columns);
    move_stones_left(stones, rows);
    move_stones_down(stones, columns, height);
    move_stones_right(stones, rows, width);
}

fn stones_value(stones: &Vec<(usize, usize)>, height: usize) -> u64 {
    stones
        .iter()
        .fold(0, |acc, (_, y)| acc + height as u64 - *y as u64)
}

fn main() {
    let lines = lines_from_file("input.txt");

    let width = lines[0].len();
    let height = lines.len();

    let hashes = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();

    let rows: HashMap<usize, Vec<usize>> =
        hashes.iter().fold(HashMap::new(), |mut acc, &(x, y)| {
            if acc.contains_key(&y) {
                match acc.get_mut(&y).unwrap().binary_search(&x) {
                    Ok(_) => {}
                    Err(idx) => {
                        acc.get_mut(&y).unwrap().insert(idx, x);
                    }
                }
            } else {
                acc.insert(y, vec![x]);
            }
            acc
        });

    let columns: HashMap<usize, Vec<usize>> =
        hashes.iter().fold(HashMap::new(), |mut acc, &(x, y)| {
            if acc.contains_key(&x) {
                match acc.get_mut(&x).unwrap().binary_search(&y) {
                    Ok(_) => {}
                    Err(idx) => {
                        acc.get_mut(&x).unwrap().insert(idx, y);
                    }
                }
            } else {
                acc.insert(x, vec![y]);
            }
            acc
        });

    let mut stones = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'O')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();

    let mut hash_map: HashMap<u64, i32> = HashMap::new();

    let mut cycle_idx = 0;
    let mut cycle_found = false;

    while cycle_idx != 1000000000 {
        cycle(&mut stones, &rows, &columns, height, width);
        cycle_idx += 1;
        if cycle_found {
            continue;
        }
        let hash = hash_vector(&stones);
        if let Some(k) = hash_map.insert(hash, cycle_idx) {
            let cycle_length = cycle_idx - k;
            println!("Found cycle at {} with length {}", cycle_idx, cycle_length);
            cycle_idx = 1000000000 - (1000000000 - cycle_idx) % cycle_length;
            cycle_found = true;
            continue;
        }
    }
    println!("Stones value: {}", stones_value(&stones, height));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
