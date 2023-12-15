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

fn hash(input: &str) -> u64 {
    input.chars().fold(0, |out, c| (out + c as u64) * 17 % 256)
}

fn solution(input: &str) -> u64 {
    let sequence: Vec<_> = input.trim().split(',').collect();
    sequence.iter().map(|s| hash(s)).sum()
}
