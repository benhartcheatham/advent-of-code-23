use std::io;

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");

    println!("solution: {}", solution(input));
    Ok(())
}

fn process_race(i: u64, race: (u64, u64)) -> Option<u64> {
    if i * (race.0 - i) > race.1 {
        Some(i)
    } else {
        None
    }
}

fn solution(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    if lines.len() != 2 {
        panic!("Invalid Input!");
    }

    let time: u64 = lines[0]
        .split(':')
        .skip(1)
        .map(|n| n.replace(' ', "").parse::<u64>().unwrap())
        .next()
        .unwrap();
    let distance: u64 = lines[1]
        .split(':')
        .skip(1)
        .map(|n| n.replace(' ', "").parse::<u64>().unwrap())
        .next()
        .unwrap();

    let (t, d) = (time, distance);
    let s = (0..t).find_map(|i| process_race(i, (t, d)));
    let e = (0..t).rev().find_map(|i| process_race(i, (t, d)));

    match (s, e) {
        (Some(v0), Some(v1)) => v1 - v0 + 1,
        _ => 0,
    }
}
