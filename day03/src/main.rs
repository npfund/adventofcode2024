use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let instructions = file.lines().map(|l| l.unwrap()).collect::<String>();
    let mul = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    let mut sum = 0;
    for (_, [lhs, rhs]) in mul.captures_iter(&instructions).map(|c| c.extract()) {
        let lhs = lhs.parse::<i32>().unwrap();
        let rhs = rhs.parse::<i32>().unwrap();

        sum += lhs * rhs;
    }

    println!("{sum}");
}
