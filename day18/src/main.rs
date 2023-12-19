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

#[allow(unused)]
struct Point<'a> {
    x: i64,
    y: i64,
    color: &'a str,
}

impl<'a> Point<'a> {
    fn new(x: i64, y: i64, color: &'a str) -> Self {
        Point { x, y, color }
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

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut points = Vec::new();

    let mut x = 0;
    let mut y = 0;
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let dir: Direction = parts[0].chars().next().unwrap().into();
        let num: i64 = parts[1].parse().unwrap();

        match dir {
            Direction::Up => y -= num,
            Direction::Down => y += num,
            Direction::Left => x -= num,
            Direction::Right => x += num,
        }

        points.push(Point::new(x, y, parts[2]));
    }

    get_area(&points)
}
