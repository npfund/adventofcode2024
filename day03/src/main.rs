use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut map = HashMap::new();
    for (y, line) in file.lines().map(|l| l.unwrap()).enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), char);
        }
    }

    let mut sum = 0;
    for y in 0..140 {
        let mut number = 0;
        let mut keep = false;
        for x in 0..140 {
            let char = map.get(&(x, y)).unwrap();
            if char.is_ascii_digit() {
                number *= 10;
                number += char.to_digit(10).unwrap();
                if !keep {
                    for z in -1..=1 {
                        for w in -1..=1 {
                            if let Some(char) = map.get(&(x + w, y + z)) {
                                if !char.is_ascii_digit() && *char != '.' {
                                    keep = true
                                }
                            }
                        }
                    }
                }
            } else {
                if keep {
                    sum += number;
                }
                number = 0;
                keep = false;
            }
        }

        if keep {
            sum += number;
        }
    }

    println!("{sum}");
}
