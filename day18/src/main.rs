use clap::{arg, command, ArgAction};
use iter_tools::*;
use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction {}!", value),
        }
    }
}

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
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

fn get_area(points: &[Point]) -> u64 {
    // area of polygon
    let mut sum: i64 = 0;
    // points on perimeter
    let mut perimeter: i64 = 0;

    // shoelace theorem for area
    for (p0, p1) in points.iter().tuple_windows() {
        sum += p0.x * p1.y - p0.y * p1.x;
        perimeter += (p1.x - p0.x).abs() + (p1.y - p0.y).abs();
    }

    let last = points.last().unwrap();
    sum += last.x * points[0].y - last.y * points[0].x;
    perimeter += (last.x - points[0].x).abs() + (last.y - points[0].y).abs();

    // pick's theorem
    let interior = sum.abs() / 2 + 1 - perimeter / 2;
    (interior + perimeter) as u64
}

fn map_hex(c: u8) -> i64 {
    if c.is_ascii_digit() {
        return (c - b'0') as i64;
    }

    if (b'a'..=b'f').contains(&c) {
        return (c - b'a' + 10) as i64;
    }

    panic!("Invalid hex u8 {}!", c);
}

fn convert_hex(hex: &str) -> (i64, Direction) {
    let digits: String = hex.chars().filter(|c| c.is_alphanumeric()).collect();
    let num = digits
        .as_bytes()
        .iter()
        .take(5)
        .rev()
        .enumerate()
        .fold(0, |n, (i, u)| n + map_hex(*u) * 16i64.pow(i as u32));
    let direction = match digits.chars().nth(5) {
        Some('0') => Direction::Right,
        Some('1') => Direction::Down,
        Some('2') => Direction::Left,
        Some('3') => Direction::Up,
        _ => panic!("Invalid direction!"),
    };

    (num, direction)
}

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut points = Vec::new();

    let mut x = 0;
    let mut y = 0;
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (num, dir) = convert_hex(parts[2]);

        match dir {
            Direction::Up => y -= num,
            Direction::Down => y += num,
            Direction::Left => x -= num,
            Direction::Right => x += num,
        }

        points.push(Point::new(x, y));
    }

    get_area(&points)
}
