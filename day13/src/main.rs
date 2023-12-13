use clap::{arg, command, ArgAction};
use itertools::*;
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

fn find_horizontal_solution(rows: &Vec<String>) -> u64 {
    let mut reflection_point;

    for (r0, r1) in (0..rows.len()).tuple_windows() {
        if rows[r0] == rows[r1] {
            reflection_point = r0;
        } else {
            continue;
        }

        let mut found = true;
        for (i, j) in ((reflection_point + 1)..rows.len()).zip((0..=reflection_point).rev()) {
            if rows[i] != rows[j] {
                found = false;
                break;
            }
        }

        if found {
            return reflection_point as u64 + 1;
        }
    }

    0
}

fn find_solution(rows: &Vec<String>) -> u64 {
    let horizontal = find_horizontal_solution(rows) * 100;
    if horizontal != 0 {
        return horizontal;
    }

    let mut columns = Vec::new();

    for i in 0..rows[0].len() {
        columns.push(String::from(""));

        for r in rows {
            let c = r.chars().nth(i);
            if let Some(c) = c {
                columns[i] += &c.to_string();
            }
        }
    }

    find_horizontal_solution(&columns)
}

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;

    let mut set: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            sum += find_solution(&set);
            set = Vec::new();
        } else {
            set.push(line.to_string());
        }
    }

    sum += find_solution(&set);
    sum
}
