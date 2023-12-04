use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut sum = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let (_card, numbers) = line.split_once(": ").unwrap();
        let (winning, numbers) = numbers.split_once(" | ").unwrap();

        let winning: HashSet<i32> = winning
            .split_whitespace()
            .map(|w| w.trim().parse().unwrap())
            .collect();
        let numbers: HashSet<i32> = numbers
            .split_whitespace()
            .map(|w| w.trim().parse().unwrap())
            .collect();

        let count = winning.intersection(&numbers).count();
        if count > 0 {
            sum += 2_i32.pow(count as u32 - 1);
        }
    }

    println!("{sum}");
}
