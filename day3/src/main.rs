use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

/// takes 3 lines (if applicable), and idx should be the
/// index in the "middle" line that we want to check
/// lines 0 & 2 can be None, line 1 MUST be Some(&str)
fn check_line(lines: &[Option<&str>], idx: usize, len: usize) -> bool {
    if lines.len() != 3 {
        println!("incorrect number of lines!");
        return false;
    }

    let start = if idx == 0 { 0 } else { idx - 1 };
    let end = if idx == 0 { len + 1 } else { len + 2 };
    let sub0 = if let Some(s) = lines[0] {
        Some(s.chars().skip(start).take(end))
    } else {
        None
    };
    let mut sub1: String = lines[1].unwrap().chars().skip(start).take(end).collect();
    let sub2 = if let Some(s) = lines[2] {
        Some(s.chars().skip(start).take(end))
    } else {
        None
    };

    if let Some(mut s) = sub0 {
        if s.any(|c| c != '.') {
            return true;
        }
    }

    if let Some(mut s) = sub2 {
        if s.any(|c| c != '.') {
            return true;
        }
    }

    if end + idx >= lines[1].unwrap().len() {
        sub1.push('.');
    } else if idx == 0 {
        sub1 = String::from(".") + &sub1;
    }

    if !sub1.starts_with('.') || !sub1.ends_with('.') {
        true
    } else {
        false
    }
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    let mut idx: i32 = -1;
    let mut len = 0;
    let mut num = String::new();

    for i in 0..lines.len() {
        for (j, c) in lines[i].char_indices() {
            if c.is_numeric() {
                if idx == -1 {
                    idx = j as i32;
                }

                len += 1;
                num.push(c);
            } else if idx != -1 {
                let l0 = if i == 0 { None } else { Some(lines[i - 1]) };
                let l2 = if i == lines.len() - 1 {
                    None
                } else {
                    Some(lines[i + 1])
                };
                let l = [l0, Some(lines[i]), l2];

                if check_line(&l, idx as usize, len) {
                    sum += num.parse::<u32>().unwrap();
                }

                idx = -1;
                len = 0;
                num = String::new();
            }
        }

        if idx != -1 {
            let l0 = if i == 0 { None } else { Some(lines[i - 1]) };
            let l2 = if i == lines.len() - 1 {
                None
            } else {
                Some(lines[i + 1])
            };
            let l = [l0, Some(lines[i]), l2];

            if check_line(&l, idx as usize, len) {
                sum += num.parse::<u32>().unwrap();
            }

            idx = -1;
            len = 0;
            num = String::new();
        }

        println!("sum: {}\n", sum);
    }

    sum
}
