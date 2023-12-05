use std::collections::HashSet;
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn get_matching(winning: &str, have: &str) -> u32 {
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
            sum += 1;
        }
    }

    sum
}

fn get_num_cards(mut card_instances: Vec<(u32, u32)>) -> u32 {
    let len = card_instances.len();

    for i in 0..len {
        let (n, p) = card_instances[i];

        if p > 0 {
            for j in 0..(p as usize) {
                let k = (j + 1) % (len - i);
                let (nk, pk) = card_instances[i + k];
                card_instances[i + k] = (nk + n, pk);
            }
        }
    }

    card_instances.iter().map(|(i, _)| i).sum()
}

fn solution(input: &str) -> u32 {
    let lines: Vec<String> = input
        .lines()
        .map(|s| s[(s.find(':').unwrap() + 1)..].to_string())
        .collect();
    // holds (num card instances for card i, num points for card i)
    let mut card_instances: Vec<(u32, u32)> = Vec::new();

    for l in lines {
        let bar = l.find('|').unwrap();
        let l = l.replace('|', " ");
        let (winning, have) = l.split_at(bar + 1);

        card_instances.push((1, get_matching(winning, have)));
    }

    get_num_cards(card_instances)
}
