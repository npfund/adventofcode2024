use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut count = 0;
    for (y, line) in lines.iter().enumerate() {
        for x in 0..line.len() {
            if let Some("XMAS") = line.get(x..x + 4) {
                count += 1;
            }

            if let Some("SAMX") = line.get(x.saturating_sub(3)..=x) {
                count += 1;
            }

            if let Some(&["X", "M", "A", "S"]) = lines
                .get(y..y + 4)
                .map(|s| s.iter().map(|l| &l[x..x + 1]).collect::<Vec<_>>())
                .as_deref()
            {
                count += 1;
            }

            if let Some(&["S", "A", "M", "X"]) = lines
                .get(y.saturating_sub(3)..=y)
                .map(|s| s.iter().map(|l| &l[x..x + 1]).collect::<Vec<_>>())
                .as_deref()
            {
                count += 1;
            }

            if let Some(&[Some("X"), Some("M"), Some("A"), Some("S")]) = lines
                .get(y..y + 4)
                .map(|s| {
                    s.iter()
                        .enumerate()
                        .map(|(i, l)| l.get((x + i)..(x + i) + 1))
                        .collect::<Vec<_>>()
                })
                .as_deref()
            {
                count += 1;
            }

            if let Some(&[Some("S"), Some("A"), Some("M"), Some("X")]) = lines
                .get(y.saturating_sub(3)..=y)
                .map(|s| {
                    s.iter()
                        .enumerate()
                        .map(|(i, l)| l.get((x + (3 - i))..(x + (3 - i)) + 1))
                        .collect::<Vec<_>>()
                })
                .as_deref()
            {
                count += 1;
            }

            if let Some(&[Some("S"), Some("A"), Some("M"), Some("X")]) = lines
                .get(y.saturating_sub(3)..=y)
                .map(|s| {
                    s.iter()
                        .enumerate()
                        .map(|(i, l)| x.checked_sub(3 - i).and_then(|x| l.get(x..x + 1)))
                        .collect::<Vec<_>>()
                })
                .as_deref()
            {
                count += 1;
            }

            if let Some(&[Some("X"), Some("M"), Some("A"), Some("S")]) = lines
                .get(y..y + 4)
                .map(|s| {
                    s.iter()
                        .enumerate()
                        .map(|(i, l)| x.checked_sub(i).and_then(|x| l.get(x..x + 1)))
                        .collect::<Vec<_>>()
                })
                .as_deref()
            {
                count += 1;
            }
        }
    }

    println!("{count}");
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut count = 0;
    for (y, line) in lines.iter().enumerate() {
        for x in 0..line.len() {
            let result = (
                y.checked_sub(1).and_then(|y| {
                    lines
                        .get(y)
                        .and_then(|l| x.checked_sub(1).and_then(|x| l.get(x..x + 1)))
                }),
                y.checked_sub(1)
                    .and_then(|y| lines.get(y).and_then(|l| l.get(x + 1..x + 2))),
                lines.get(y).and_then(|l| l.get(x..x + 1)),
                lines
                    .get(y + 1)
                    .and_then(|l| x.checked_sub(1).and_then(|x| l.get(x..x + 1))),
                lines.get(y + 1).and_then(|l| l.get(x + 1..x + 2)),
            );
            match result {
                (Some("M"), Some("M"), Some("A"), Some("S"), Some("S"))
                | (Some("S"), Some("M"), Some("A"), Some("S"), Some("M"))
                | (Some("S"), Some("S"), Some("A"), Some("M"), Some("M"))
                | (Some("M"), Some("S"), Some("A"), Some("M"), Some("S")) => {
                    count += 1;
                }
                _ => {}
            }
        }
    }

    println!("{count}");
}
