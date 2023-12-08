use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    let filename = "input.txt";
    let mut idx_sum = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let score_str = ip.split(":").last().unwrap();

                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                score_str.split(";").for_each(|scores| {
                    for score in scores.split(",") {
                        let (score, color) = scan!(score.trim(), " ", usize, String);

                        if let Some(clr) = color {
                            match clr.as_str() {
                                "red" => {
                                    max_red = max_red.max(score.unwrap());
                                },
                                "green" => {
                                    max_green = max_green.max(score.unwrap());
                                },
                                "blue" => {
                                    max_blue = max_blue.max(score.unwrap());
                                },
                                _ => {}
                            }
                        }
                    }
                });

                idx_sum += max_red * max_green * max_blue;
            }
        }
    }

    println!("Sum of minimum game indexes: {}", idx_sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
