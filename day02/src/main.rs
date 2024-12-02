use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut safe = 0;
    'line: for line in file.lines().map(|l| l.unwrap()) {
        let levels = line
            .split(' ')
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut direction = None;
        for pair in levels.windows(2) {
            let first = pair[0];
            let second = pair[1];

            if let Some(direction) = direction {
                if direction != first.cmp(&second) {
                    continue 'line;
                }
            } else {
                direction = Some(first.cmp(&second));
            }

            let diff = first.abs_diff(second);
            if diff < 1 || diff > 3 {
                continue 'line;
            }
        }
        safe += 1;
    }

    println!("{safe}");
}
