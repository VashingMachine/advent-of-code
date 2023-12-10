use std::{
    default,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<char>>,
}

impl Map {
    fn new(lines: &Vec<String>) -> Map {
        let width = lines[0].len();
        let height = lines.len();
        let mut map = Vec::new();
        for line in lines {
            map.push(line.chars().collect());
        }
        Map { width, height, map }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<char> {
        if y >= self.height || x >= self.width {
            return None;
        }

        Some(self.map[y][x])
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn next_direction(prev_direction: &Direction, current_pipe: char) -> Option<Direction> {
    match current_pipe {
        'L' => match prev_direction {
            Direction::Down => Some(Direction::Right),
            Direction::Left => Some(Direction::Up),
            _ => None,
        },
        '7' => match prev_direction {
            Direction::Up => Some(Direction::Left),
            Direction::Right => Some(Direction::Down),
            _ => None,
        },
        'F' => match prev_direction {
            Direction::Up => Some(Direction::Right),
            Direction::Left => Some(Direction::Down),
            _ => None,
        },
        'J' => match prev_direction {
            Direction::Down => Some(Direction::Left),
            Direction::Right => Some(Direction::Up),
            _ => None,
        },
        '-' => match prev_direction {
            Direction::Left => Some(Direction::Left),
            Direction::Right => Some(Direction::Right),
            _ => None,
        },
        '|' => match prev_direction {
            Direction::Up => Some(Direction::Up),
            Direction::Down => Some(Direction::Down),
            _ => None,
        },
        _ => panic!("Invalid pipe"),
    }
}

fn try_go(map: &Map, p: (usize, usize), direction: Direction) -> i32 {
    let next_point = match direction {
        Direction::Up => (p.0, p.1 - 1),
        Direction::Down => (p.0, p.1 + 1),
        Direction::Left => (p.0 - 1, p.1),
        Direction::Right => (p.0 + 1, p.1),
    };

    if let Some(next_pipe) = map.get(next_point) {
        let correct_connection = match (next_pipe, direction) {
            ('L', Direction::Down) => true,
            ('L', Direction::Left) => true,
            ('7', Direction::Up) => true,
            ('7', Direction::Right) => true,
            ('F', Direction::Up) => true,
            ('F', Direction::Left) => true,
            ('J', Direction::Down) => true,
            ('J', Direction::Right) => true,
            ('-', Direction::Left) => true,
            ('-', Direction::Right) => true,
            ('|', Direction::Up) => true,
            ('|', Direction::Down) => true,
            _ => false,
        };

        if !correct_connection {
            return -1;
        }

        let mut current_point = next_point;
        let mut prev_direction = direction;

        let mut length = 1;

        while current_point != p {
            let current_pipe = map.get(current_point).unwrap();
            if let Some(next_direction) = next_direction(&prev_direction, current_pipe) {
                current_point = match next_direction {
                    Direction::Up => (current_point.0, current_point.1 - 1),
                    Direction::Down => (current_point.0, current_point.1 + 1),
                    Direction::Left => (current_point.0 - 1, current_point.1),
                    Direction::Right => (current_point.0 + 1, current_point.1),
                };

                prev_direction = next_direction;
                length += 1;
            } else {
                return -1;
            }
        }

        return length;
    } else {
        return -1;
    }
}

fn main() {
    let lines = lines_from_file("input.txt");

    let mut s_x: usize = usize::MAX;
    let mut s_y: usize = usize::MAX;

    for (y, line) in lines.iter().enumerate() {
        if let Some(x) = line.find('S') {
            s_x = x;
            s_y = y;
            break;
        }
    }

    let map = Map::new(&lines);

    let x = try_go(&map, (s_x, s_y), Direction::Right);

    println!("Size of path: {}", x);
    println!("Divide by 2 to get the answer: {}", x / 2);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
