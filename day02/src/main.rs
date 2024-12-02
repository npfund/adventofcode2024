use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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
            if !(1..=3).contains(&diff) {
                continue 'line;
            }
        }
        safe += 1;
    }

    println!("{safe}");
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut safe = 0;
    'line: for line in file.lines().map(|l| l.unwrap()) {
        let levels = line
            .split(' ')
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut direction = None;
        let mut bad_pair = None;
        for (pair_index, pair) in levels.windows(2).enumerate() {
            let first = pair[0];
            let second = pair[1];

            if let Some(direction) = direction {
                if direction != first.cmp(&second) {
                    bad_pair = Some(pair_index);
                    break;
                }
            } else {
                let dir = first.cmp(&second);
                if dir == Ordering::Equal {
                    bad_pair = Some(pair_index);
                    break;
                }
                direction = Some(dir);
            }

            let diff = first.abs_diff(second);
            if !(1..=3).contains(&diff) {
                bad_pair = Some(pair_index);
                break;
            }
        }

        if let Some(bad_pair) = bad_pair {
            let mut filtered_levels = levels.clone();
            filtered_levels.remove(bad_pair);
            let mut direction = None;
            let mut bad = false;
            for pair in filtered_levels.windows(2) {
                let first = pair[0];
                let second = pair[1];

                if let Some(direction) = direction {
                    if direction != first.cmp(&second) {
                        bad = true;
                        break;
                    }
                } else {
                    let dir = first.cmp(&second);
                    if dir == Ordering::Equal {
                        bad = true;
                        break;
                    }
                    direction = Some(dir);
                }

                let diff = first.abs_diff(second);
                if !(1..=3).contains(&diff) {
                    bad = true;
                    break;
                }
            }

            if bad {
                bad = false;
                let mut filtered_levels = levels.clone();
                let mut direction = None;
                filtered_levels.remove(bad_pair + 1);
                for pair in filtered_levels.windows(2) {
                    let first = pair[0];
                    let second = pair[1];

                    if let Some(direction) = direction {
                        if direction != first.cmp(&second) {
                            bad = true;
                            break;
                        }
                    } else {
                        let dir = first.cmp(&second);
                        if dir == Ordering::Equal {
                            bad = true;
                            break;
                        }
                        direction = Some(dir);
                    }

                    let diff = first.abs_diff(second);
                    if !(1..=3).contains(&diff) {
                        bad = true;
                        break;
                    }
                }
            }

            if bad {
                let mut filtered_levels = levels.clone();
                let mut direction = None;
                filtered_levels.remove(0);
                for pair in filtered_levels.windows(2) {
                    let first = pair[0];
                    let second = pair[1];

                    if let Some(direction) = direction {
                        if direction != first.cmp(&second) {
                            continue 'line;
                        }
                    } else {
                        let dir = first.cmp(&second);
                        if dir == Ordering::Equal {
                            continue 'line;
                        }
                        direction = Some(dir);
                    }

                    let diff = first.abs_diff(second);
                    if !(1..=3).contains(&diff) {
                        continue 'line;
                    }
                }
            }
        }

        safe += 1;
    }

    println!("{safe}");
}
