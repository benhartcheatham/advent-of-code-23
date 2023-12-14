use clap::{arg, command, ArgAction};
use std::io;

#[derive(Debug)]
struct CubeRock {
    pos: u64,
    num_rocks: u64,
}

impl CubeRock {
    fn new(pos: u64, num_rocks: u64) -> Self {
        CubeRock { pos, num_rocks }
    }

    fn get_load(&self, num_rows: u64) -> u64 {
        if self.num_rocks == 0 {
            return 0;
        }

        (1..=self.num_rocks).fold(0, |load, rock| load + num_rows - self.pos - rock)
    }
}

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

fn find_load(platform: Vec<String>) -> u64 {
    if platform.is_empty() {
        return 0;
    }

    let mut cubes = Vec::new();
    for c in 0..platform[0].len() {
        for (i, r) in platform.iter().enumerate() {
            if let Some(ch) = r.chars().nth(c) {
                match ch {
                    '#' => cubes.push(CubeRock::new(i as u64, 0)),
                    'O' => {
                        if let Some(cube) = cubes.last_mut() {
                            cube.num_rocks += 1;
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    cubes
        .iter()
        .fold(0, |load, rock| load + rock.get_load(platform.len() as u64))
}

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut platform = Vec::new();
    let n_cols = if let Some(line) = lines.first() {
        line.len()
    } else {
        0
    };

    platform.push("#".repeat(n_cols));
    for line in lines {
        platform.push(line.to_string());
    }

    find_load(platform)
}
