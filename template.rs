use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn solution(input: &str) -> u32 {
    0
}
