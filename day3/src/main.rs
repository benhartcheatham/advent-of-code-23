use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn construct_numbers(line: &str, idx: usize) -> Option<Vec<u32>> {
    let mut filter = line.to_string();

    let mut left_idx = idx;
    let left = line.chars().rev().skip(line.len() - left_idx);
    for c in left {
        if c.is_numeric() {
            left_idx -= 1;
        } else {
            break;
        }
    }

    let mut right_idx = idx + 1;
    let right = line.chars().skip(right_idx);
    for c in right {
        if c.is_numeric() {
            right_idx += 1;
        } else {
            break;
        }
    }

    filter = filter
        .chars()
        .skip(left_idx)
        .take(right_idx - left_idx)
        .map(|c| if c.is_numeric() { c } else { ' ' })
        .collect();
    let ret: Vec<u32> = filter
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    if !ret.is_empty() {
        Some(ret)
    } else {
        None
    }
}

/// takes the lines around a * and the idx of the * in lines[1]
fn find_numbers(lines: &[Option<&str>], idx: usize) -> Vec<u32> {
    let mut nums = Vec::new();

    for line in lines.iter().flatten() {
        let ns = construct_numbers(line, idx);

        if let Some(ns) = ns {
            for n in ns {
                nums.push(n)
            }
        }
    }

    nums
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for i in 0..lines.len() {
        for (j, c) in lines[i].char_indices() {
            if c == '*' {
                let l0 = if i == 0 { None } else { Some(lines[i - 1]) };
                let l2 = if i == lines.len() - 1 {
                    None
                } else {
                    Some(lines[i + 1])
                };

                let nums = find_numbers(&[l0, Some(lines[i]), l2], j);
                if nums.len() == 2 {
                    sum += nums[0] * nums[1];
                }
            }
        }
    }

    sum
}
