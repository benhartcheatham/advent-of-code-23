use std::io;

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");

    println!("solution: {}", solution(&input));
    Ok(())
}

fn process_race(i: u32, race: (u32, u32)) -> Option<u32> {
    if i * (race.0 - i) > race.1 {
        Some(i)
    } else {
        None
    }
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    if lines.len() != 2 {
        panic!("Invalid Input!");
    }

    let times: Vec<u32> = lines[0].split_whitespace().skip(1).map(|n| n.parse::<u32>().unwrap()).collect();
    let distances: Vec<u32> = lines[1].split_whitespace().skip(1).map(|n| n.parse::<u32>().unwrap()).collect();

    let mut sum = 1;
    for (t, d) in times.iter().zip(distances) {
        let t = *t;

        let s = (0..t).find_map(|i| process_race(i, (t, d)));
        let e = (0..t).rev().find_map(|i| process_race(i, (t, d)));

        match (s, e) {
            (Some(v0), Some(v1)) => sum *= v1 - v0 + 1,
            _ => continue,
        }
    }

    sum
}
