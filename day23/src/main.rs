use clap::{arg, command, ArgAction};
use std::{collections::HashSet, io};

#[derive(Debug, PartialEq, Eq)]
enum TileType {
    Path,
    Forest,
    Slope(char),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TileType {
    fn is_passable(&self, direction: Direction) -> bool {
        use Direction::*;
        use TileType::*;

        match self {
            Path => true,
            Forest => false,
            Slope(c) => {
                matches!(
                    (c, direction),
                    ('^', Up) | ('v', Down) | ('<', Left) | ('>', Right)
                )
            }
        }
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

fn get_next(current: (usize, usize), graph: &[Vec<TileType>]) -> Vec<(usize, usize)> {
    let mut next = Vec::new();

    if graph[current.0 - 1][current.1].is_passable(Direction::Up) {
        next.push((current.0 - 1, current.1));
    }

    if graph[current.0 + 1][current.1].is_passable(Direction::Down) {
        next.push((current.0 + 1, current.1));
    }

    if graph[current.0][current.1 - 1].is_passable(Direction::Left) {
        next.push((current.0, current.1 - 1));
    }

    if graph[current.0][current.1 + 1].is_passable(Direction::Right) {
        next.push((current.0, current.1 + 1));
    }

    next
}

fn find_longest_path(
    curr: (usize, usize),
    end: (usize, usize),
    mut seen: HashSet<(usize, usize)>,
    graph: &Vec<Vec<TileType>>,
) -> u64 {
    if curr == end {
        return 1;
    }

    seen.insert(curr);

    let mut max = 0;
    for n in get_next(curr, graph) {
        if !seen.contains(&n) {
            max = u64::max(find_longest_path(n, end, seen.clone(), graph), max);
        }
    }

    max + 1
}

fn solution(input: &str) -> u64 {
    let mut lines: Vec<_> = input.lines().collect();
    let mut graph = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    let buffer = "#".repeat(lines[0].len());
    lines.insert(0, buffer.as_str());
    lines.push(buffer.as_str());

    for (i, line) in lines.iter().enumerate() {
        graph.push(Vec::new());

        for (j, c) in line.char_indices() {
            match c {
                '.' => {
                    if i == 1 {
                        start = (i, j);
                    }

                    if i == lines.len() - 2 {
                        end = (i, j);
                    }

                    graph[i].push(TileType::Path)
                }
                '#' => graph[i].push(TileType::Forest),
                _ => graph[i].push(TileType::Slope(c)),
            }
        }

        println!("{}", line);
    }

    let seen = HashSet::new();
    find_longest_path(start, end, seen, &graph).saturating_sub(1)
}
