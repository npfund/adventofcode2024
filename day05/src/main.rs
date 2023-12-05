use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
struct MapEntry {
    source_offset: i64,
    destination_offset: i64,
    range: i64,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut seeds: Vec<i64> = Vec::new();
    let mut maps = Vec::new();
    let mut current = Vec::new();
    for line in file.lines().map(|l| l.unwrap()) {
        if line.contains("seeds") {
            let (_, s) = line.split_once(": ").unwrap();
            seeds = s.split(' ').map(|s| s.trim().parse().unwrap()).collect();
        } else if line.contains("map") {
            if !current.is_empty() {
                maps.push(current.clone());
                current.clear();
            }
        } else if line.is_empty() {
            continue;
        } else {
            let parts: Vec<i64> = line.split(' ').map(|p| p.parse().unwrap()).collect();
            current.push(MapEntry {
                source_offset: parts[1],
                destination_offset: parts[0],
                range: parts[2],
            });
        }
    }
    maps.push(current);

    let mut lowest = i64::MAX;
    for seed in seeds {
        let mut value = seed;
        for map in &maps {
            for entry in map {
                if value >= entry.source_offset && value < entry.source_offset + entry.range {
                    let delta = value - entry.source_offset;
                    value = entry.destination_offset + delta;
                    break;
                }
            }
        }
        if value < lowest {
            lowest = value;
        }
    }

    println!("{lowest}");
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut maps = Vec::new();
    let mut current = Vec::new();
    for line in file.lines().map(|l| l.unwrap()) {
        if line.contains("seeds") {
            let (_, s) = line.split_once(": ").unwrap();
            let s: Vec<_> = s.split(' ').map(|s| s.trim().parse().unwrap()).collect();
            seeds = s.chunks(2).map(|c| (c[0], c[1])).collect();
        } else if line.contains("map") {
            if !current.is_empty() {
                maps.push(current.clone());
                current.clear();
            }
        } else if line.is_empty() {
            continue;
        } else {
            let parts: Vec<i64> = line.split(' ').map(|p| p.parse().unwrap()).collect();
            current.push(MapEntry {
                source_offset: parts[1],
                destination_offset: parts[0],
                range: parts[2],
            });
        }
    }
    maps.push(current);

    let mut lowest = i64::MAX;
    for seed_pair in seeds {
        for seed in seed_pair.0..seed_pair.0 + seed_pair.1 {
            let mut value = seed;
            for map in &maps {
                for entry in map {
                    if value >= entry.source_offset && value < entry.source_offset + entry.range {
                        let delta = value - entry.source_offset;
                        value = entry.destination_offset + delta;
                        break;
                    }
                }
            }
            if value < lowest {
                lowest = value;
            }
        }
    }

    println!("{lowest}");
}
