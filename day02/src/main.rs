use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut sum = 0;
    'line: for line in file.lines().map(|l| l.unwrap()) {
        let (game, handfuls) = line.split_once(": ").unwrap();

        let id: i32 = game.split_once(' ').unwrap().1.parse().unwrap();

        for handful in handfuls.split("; ") {
            for cubes in handful.split(", ") {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count: i32 = count.parse().unwrap();
                match color {
                    "red" => if count > 12 { continue 'line },
                    "green" => if count > 13 { continue 'line },
                    "blue" => if count > 14 { continue 'line },
                    _ => panic!()
                }
            }
        }

        sum += id;
    }

    println!("{sum}");
}
