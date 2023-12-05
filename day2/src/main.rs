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

fn check_cubes(map: &HashMap<&str, u32>, cubes: &Vec<(&str, u32)>) -> bool {
    for c in cubes {
        match map.get(c.0) {
            Some(v) => if *v > c.1 { return false; },
            None => { return false; },
        }
    }

    true
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let cubes = vec![("red", 12), ("green", 13), ("blue", 14)];
    let mut sum = 0;

    for (i, g) in lines.iter().enumerate() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        let id = i + 1;
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

        if check_cubes(&map, &cubes) {
           sum += id;
        }
    }

    sum as u32
}
