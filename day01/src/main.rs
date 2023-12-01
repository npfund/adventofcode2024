use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut sum = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let chars = line.chars().collect::<Vec<_>>();
        let first = line.find(char::is_numeric).unwrap();
        let last = line.rfind(char::is_numeric).unwrap();

        let number: u32 = format!("{}{}", chars[first], chars[last]).parse().unwrap();
        sum += number;
    }

    println!("{sum}");
}
