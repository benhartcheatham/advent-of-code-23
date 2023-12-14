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

fn find_horizontal_solution(rows: &Vec<String>) -> u64 {
    let mut reflection_points = Vec::new();

    for i in 0..rows.len() {
        let mut found = true;
        for (i, j) in ((i + 1)..rows.len()).zip((0..=i).rev()) {
            if rows[i] != rows[j] {
                found = false;
                break;
            }
        }

        if found {
            reflection_points.push(i as u64);
        }
    }

    for i in 0..rows.len() {
        let mut smudge_found = false;
        let mut found = true;

        for (j, k) in ((i + 1)..rows.len()).zip((0..=i).rev()) {
            let diff: Vec<u64> = rows[j]
                .char_indices()
                .zip(rows[k].char_indices())
                .filter(|((_, cj), (_, ck))| cj != ck)
                .map(|((rj, _), (_, _))| rj as u64)
                .collect();

            match diff.len() {
                1 => {
                    if !smudge_found {
                        smudge_found = true;
                    } else {
                        found = false;
                        break;
                    }
                }
                0 => continue,
                _ => {
                    found = false;
                    break;
                }
            }
        }

        if found && !reflection_points.contains(&(i as u64)) {
            return i as u64 + 1;
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
