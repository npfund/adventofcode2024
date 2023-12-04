use std::collections::{HashMap, HashSet, VecDeque};
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut cards = HashMap::new();
    let mut queue = VecDeque::new();
    for line in file.lines().map(|l| l.unwrap()) {
        let (card, numbers) = line.split_once(": ").unwrap();
        let card: i32 = card.split_whitespace().last().unwrap().parse().unwrap();
        let (winning, numbers) = numbers.split_once(" | ").unwrap();

        let winning: HashSet<i32> = winning
            .split_whitespace()
            .map(|w| w.trim().parse().unwrap())
            .collect();
        let numbers: HashSet<i32> = numbers
            .split_whitespace()
            .map(|w| w.trim().parse().unwrap())
            .collect();

        let count = winning.intersection(&numbers).count() as i32;
        cards.insert(card, count);
        queue.push_back(card);
    }

    let mut count = 0;
    while let Some(card) = queue.pop_front() {
        count += 1;
        let to_add = *cards.get(&card).unwrap();
        for next in 1..=to_add {
            queue.push_back(card + next);
        }
    }

    println!("{count}");
}
