use clap::{arg, command, ArgAction};
use std::io;

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");
    let example = include_str!("../../example.txt");

    let matches = command!()
        .arg(arg!(example: -e).action(ArgAction::SetTrue))
        .get_matches();

    if matches.get_flag("example") {
        println!("solution (example): {}", solution(example));
    } else {
        println!("solution: {}", solution(input));
    }

    Ok(())
}

fn find_arrangements(springs: String, groups: &Vec<usize>) -> u32 {
    if !springs.contains('?') {
        let hashes: Vec<_> = springs.split('.').filter(|h| !h.is_empty()).collect();

        if hashes.len() != groups.len() {
            return 0;
        }

        if hashes.iter().enumerate().all(|(i, h)| h.len() == groups[i]) {
            return 1;
        } else {
            return 0;
        }
    }

    find_arrangements(springs.replacen('?', "#", 1), groups)
        + find_arrangements(springs.replacen('?', ".", 1), groups)
}

fn solution(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();

        if parts.len() != 2 {
            continue;
        }

        let springs = parts[0];
        let groups: Vec<_> = parts[1]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        println!("finding arrangements for spring {}!", springs);
        sum += find_arrangements(springs.to_string(), &groups);
    }

    sum
}
