use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn check_line(line: &str) -> String {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut ret = line.to_string();

    for (w, d) in words.iter().zip(1..=9) {
        let match_str = ret.clone();

        for (i, (mi, _)) in match_str.match_indices(w).enumerate() {
            ret.insert(mi + i + 1, char::from_digit(d, 10).unwrap());
        }
    }

    ret
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut nums = Vec::new();

    for c in lines {
        let digits: Vec<u32> = check_line(c)
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        if digits.is_empty() {
            continue;
        }

        nums.push(digits.first().unwrap() * 10 + digits.last().unwrap());
    }

    nums.iter().sum()
}
