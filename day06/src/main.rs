use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let times: Vec<f32> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|t| t.trim().parse().unwrap())
        .collect();
    let distances: Vec<f32> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|t| t.trim().parse().unwrap())
        .collect();

    let mut product = 1.0;
    for race in 0..4 {
        let root = (times[race].powi(2) - 4.0 * (distances[race] + 0.1)).sqrt();
        let total =
            ((-times[race] - root) / -2.0).floor() - ((-times[race] + root) / -2.0).ceil() + 1.0;
        product *= total;
    }

    println!("{product}");
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let time: f64 = lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: f64 = lines[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let root = (time.powi(2) - 4.0 * (distance + 0.1)).sqrt();
    let total = ((-time - root) / -2.0).floor() - ((-time + root) / -2.0).ceil() + 1.0;

    println!("{total}");
}
