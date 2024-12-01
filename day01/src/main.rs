use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();

    println!("{sum}");
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut left = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in file.lines().map(|l| l.unwrap()) {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<i32>().unwrap());
        *right.entry(r.parse::<i32>().unwrap()).or_default() += 1;
    }

    left.sort_unstable();

    let sum = left
        .iter()
        .map(|l| l * right.get(l).copied().unwrap_or_default())
        .sum::<i32>();

    println!("{sum}");
}
