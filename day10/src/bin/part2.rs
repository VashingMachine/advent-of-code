use std::{
    collections::{HashSet, VecDeque},
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Left,
    Right,
}

fn right(p: (usize, usize)) -> (usize, usize) {
    (p.0 + 1, p.1)
}

fn left(p: (usize, usize)) -> (usize, usize) {
    (p.0 - 1, p.1)
}

fn above(p: (usize, usize)) -> (usize, usize) {
    (p.0, p.1 - 1)
}

fn below(p: (usize, usize)) -> (usize, usize) {
    (p.0, p.1 + 1)
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

fn direction_points(current_point: (usize, usize), current_pipe: char, next_direction: Direction, orientation: Orientation) -> Vec<(usize, usize)> {
    match (current_pipe, next_direction) {
        ('-', Direction::Left) => match orientation {
            Orientation::Left => {
                return vec![below(current_point)];
            }
            Orientation::Right => {
                return vec![above(current_point)];
            }
        },
        ('-', Direction::Right) => match orientation {
            Orientation::Left => {
                return vec![above(current_point)];
            }
            Orientation::Right => {
                return vec![below(current_point)];
            }
        },
        ('|', Direction::Up) => match orientation {
            Orientation::Left => {
                return vec![left(current_point)];
            }
            Orientation::Right => {
                return vec![right(current_point)];
            }
        },
        ('|', Direction::Down) => match orientation {
            Orientation::Left => {
                return vec![right(current_point)];
            }
            Orientation::Right => {
                return vec![left(current_point)];
            }
        },
        ('L', Direction::Right) => match orientation {
            Orientation::Right => {
                return vec![left(current_point), below(current_point)];
            },
            Orientation::Left => {
                return vec![];
            }
        },
        ('L', Direction::Up) => match orientation {
            Orientation::Right => {
                return vec![];
            },
            Orientation::Left => {
                return vec![left(current_point), below(current_point)];
            }
        },
        ('7', Direction::Left) => match orientation {
            Orientation::Right => {
                return vec![right(current_point), above(current_point)];
            },
            Orientation::Left => {
                return vec![];
            }
        },
        ('7', Direction::Down) => match orientation {
            Orientation::Right => {
                return vec![];
            },
            Orientation::Left => {
                return vec![right(current_point), above(current_point)];
            }
        },
        ('F', Direction::Right) => match orientation {
            Orientation::Right => {
                return vec![];
            },
            Orientation::Left => {
                return vec![left(current_point), above(current_point)];
            }
        },
        ('F', Direction::Down) => match orientation {
            Orientation::Right => {
                return vec![left(current_point), above(current_point)];
            },
            Orientation::Left => {
                return vec![];
            }
        },
        ('J', Direction::Left) => match orientation {
            Orientation::Right => {
                return vec![];
            },
            Orientation::Left => {
                return vec![right(current_point), below(current_point)];
            }
        },
        ('J', Direction::Up) => match orientation {
            Orientation::Right => {
                return vec![right(current_point), below(current_point)];
            },
            Orientation::Left => {
                return vec![];
            }
        },
        _ => panic!("Invalid pipe")
    }
}

fn try_go(map: &Map, p: (usize, usize), direction: Direction, orientation: Orientation) -> i32 {
    let next_point = match direction {
        Direction::Up => above(p),
        Direction::Down => below(p),
        Direction::Left => left(p),
        Direction::Right => right(p),
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

        let mut path = vec![p, next_point];

        let mut orientation_points: HashSet<(usize, usize)> = HashSet::new();

        let mut current_point = next_point;
        let mut prev_direction = direction;

        while current_point != p {
            let current_pipe = map.get(current_point).unwrap();
            if let Some(next_direction) = next_direction(&prev_direction, current_pipe) {
                for pt in direction_points(current_point, current_pipe, next_direction, orientation) {
                    orientation_points.insert(pt);
                }

                current_point = match next_direction {
                    Direction::Up => (current_point.0, current_point.1 - 1),
                    Direction::Down => (current_point.0, current_point.1 + 1),
                    Direction::Left => (current_point.0 - 1, current_point.1),
                    Direction::Right => (current_point.0 + 1, current_point.1),
                };

                path.push(current_point);

                prev_direction = next_direction;
            } else {
                return -1;
            }
        }

        let mut inner_points: Vec<(usize, usize)> = orientation_points
            .iter()
            .filter(|p| !path.contains(p))
            .map(|p| *p)
            .collect();

        let mut stack: VecDeque<(usize, usize)> = inner_points.clone().into();

        while let Some(p) = stack.pop_front() {
            // it cannot go outsize, because we want to caunt fields inside the loop
            let neighbours = vec![
                above(p),
                below(p),
                left(p),
                right(p),
            ];

            for neighbour in neighbours {
                if !path.contains(&neighbour) && !inner_points.contains(&neighbour) {
                    inner_points.push(neighbour);
                    stack.push_back(neighbour);
                }
            }
        }

        return inner_points.len() as i32;
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

    let x = try_go(&map, (s_x, s_y), Direction::Right, Orientation::Left);

    println!("Inner points: {}", x);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
