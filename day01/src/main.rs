use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let first_pattern =
        Regex::new(r#"one|two|three|four|five|six|seven|eight|nine|\d"#).unwrap();
    let last_pattern =
        Regex::new(r#"enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|\d"#).unwrap();
    let mut sum = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let first = first_pattern.find(&line).unwrap();
        let first_number = match first.as_str() {
            "one" | "1" => 1,
            "two" | "2" => 2,
            "three" | "3" => 3,
            "four" | "4" => 4,
            "five" | "5" => 5,
            "six" | "6" => 6,
            "seven" | "7" => 7,
            "eight" | "8" => 8,
            "nine" | "9" => 9,
            _ => panic!(),
        };

        let reverse = line.chars().rev().collect::<String>();
        let last = last_pattern.find(&reverse).unwrap();
        let last_number = match last.as_str() {
            "eno" | "1" => 1,
            "owt" | "2" => 2,
            "eerht" | "3" => 3,
            "ruof" | "4" => 4,
            "evif" | "5" => 5,
            "xis" | "6" => 6,
            "neves" | "7" => 7,
            "thgie" | "8" => 8,
            "enin" | "9" => 9,
            _ => panic!(),
        };

        let number = first_number * 10 + last_number;
        sum += number;
    }

    println!("{sum}");
}
