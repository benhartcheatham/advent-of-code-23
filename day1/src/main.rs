use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut nums = Vec::new();

    for c in lines {
        let digits: Vec<u32> = c.chars().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).collect();

        if digits.is_empty() {
            continue;
        }

        let num = digits.first().unwrap() * 10 + digits.last().unwrap();
        println!("num: {}", num);
        nums.push(digits.first().unwrap() * 10 + digits.last().unwrap());
    }

    nums.iter().sum()
}
