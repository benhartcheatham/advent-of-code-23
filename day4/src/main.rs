use std::collections::HashSet;
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn get_points(winning: &str, have: &str) -> u32 {
    let mut sum = 0;
    let mut wnums = HashSet::new();

    for n in winning
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
    {
        wnums.insert(n);
    }

    for n in have.split_whitespace().map(|s| s.parse::<u32>().unwrap()) {
        if wnums.contains(&n) {
            sum = if sum > 0 { sum * 2 } else { 1 };
        }
    }

    sum
}

fn solution(input: &str) -> u32 {
    let lines: Vec<String> = input
        .lines()
        .map(|s| s[(s.find(':').unwrap() + 1)..].to_string())
        .collect();
    let mut sum: u32 = 0;

    for l in lines {
        let bar = l.find('|').unwrap();
        let l = l.replace('|', " ");
        let (winning, have) = l.split_at(bar + 1);

        sum += get_points(winning, have);
    }

    sum
}
