use std::{fs, io};
use std::collections::HashMap;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../input.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn parse_pull<'a>(pull: &'a str) -> Vec<(&'a str, u32)> {
    let mut cubes = Vec::new();
    let pull: Vec<&str> = pull.split(",").map(|s| s.trim()).collect();

    for c in pull {
        let temp: Vec<&str> = c.split(" ").collect();

        if temp.len() != 2 {
            continue;
        }

        cubes.push((temp[1], u32::from_str_radix(temp[0], 10).unwrap()));
    }

    cubes
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;

    for g in lines.iter() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        let data = (*g.split(":").collect::<Vec<&str>>().last().unwrap()).trim();
        let pulls: Vec<&str> = data.split(";").collect();

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

    sum as u32
}
