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
    let mut reflection_points = Vec::new();

    for (r0, r1) in (0..rows.len()).tuple_windows() {

        let reflection_point = if rows[r0] == rows[r1] {
            r0
        } else {
            continue;
        };

        let mut found = true;
        for (i, j) in ((reflection_point + 1)..rows.len()).zip((0..=reflection_point).rev()) {
            if rows[i] != rows[j] {
                found = false;
                break;
            }
        }

        if found {
            reflection_points.push(reflection_point as u64);
        }
    }

    for (r0, r1) in (0..rows.len()).tuple_windows() {
        let mut smudge_found = false;
        let reflection_point;
        let diff: Vec<u64> = rows[r0]
            .char_indices()
            .zip(rows[r1].char_indices())
            .filter(|((_, c0), (_, c1))| c0 != c1)
            .map(|((r0, _), (_, _))| r0 as u64)
            .collect();

        if diff.len() == 1 {
            reflection_point = r0;
            smudge_found = true;
        } else if diff.is_empty() {
            reflection_point = r0;
        } else {
            continue;
        }

        let mut found = true;
        for (i, j) in ((reflection_point + 2)..rows.len()).zip((0..reflection_point).rev()) {
            let diff: Vec<u64> = rows[i]
                .char_indices()
                .zip(rows[j].char_indices())
                .filter(|((_, ci), (_, cj))| ci != cj)
                .map(|((ri, _), (_, _))| ri as u64)
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

        if found && !reflection_points.contains(&(r0 as u64)) {
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
