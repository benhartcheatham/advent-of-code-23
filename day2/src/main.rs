use std::collections::HashMap;
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn parse_pull(pull: &str) -> Vec<(&str, u32)> {
    let mut cubes = Vec::new();
    let pull: Vec<&str> = pull.split(',').map(|s| s.trim()).collect();

    for c in pull {
        let temp: Vec<&str> = c.split(' ').collect();

        if temp.len() != 2 {
            continue;
        }

        cubes.push((temp[1], temp[0].parse::<u32>().unwrap()));
    }

    cubes
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for g in lines.iter() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        let data = (*g.split(':').collect::<Vec<&str>>().last().unwrap()).trim();
        let pulls: Vec<&str> = data.split(';').collect();

        for p in pulls {
            for c in parse_pull(p) {
                let e = *map.entry(c.0).or_insert(c.1);
                if e < c.1 {
                    map.insert(c.0, c.1);
                }
            }
        }

        sum += map.into_values().reduce(|acc, e| acc * e).unwrap_or(0);
    }

    sum
}
