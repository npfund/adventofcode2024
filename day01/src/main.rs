use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in file.lines().map(|l| l.unwrap()) {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    let sum = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();

    println!("{sum}");
}
