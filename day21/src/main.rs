use clap::{arg, command, ArgAction};
use std::collections::{HashSet, VecDeque};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Plot,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    kind: TileType,
    path: usize,
}

impl Tile {
    fn new(kind: TileType) -> Self {
        Tile { kind, path: 0 }
    }

    fn is_traversable(&self) -> bool {
        self.kind != TileType::Rock
    }
}

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");
    let example = include_str!("../../example.txt");

    let matches = command!()
        .arg(arg!(example: -e).action(ArgAction::SetTrue))
        .get_matches();

    if matches.get_flag("example") {
        println!("solution (example): {}", solution(example, 6));
    } else {
        println!("solution: {}", solution(input, 64));
    }

    Ok(())
}

#[allow(unused)]
fn print_grid(start: (usize, usize), step_limit: usize, grid: &[Vec<Tile>]) {
    for (i, row) in grid.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            match t.kind {
                TileType::Plot => {
                    if t.path == step_limit {
                        print!("O");
                    } else if (i, j) == start {
                        print!("S");
                    } else {
                        print!(".");
                    }
                }
                TileType::Rock => print!("#"),
            }
        }

        println!();
    }
}

fn get_adjacent(pos: (usize, usize), grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut adj = Vec::new();
    let (r, c) = (pos.0, pos.1);

    if r < grid.len() - 1 && grid[r + 1][c].is_traversable() {
        adj.push((r + 1, c));
    }

    if r > 0 && grid[r - 1][c].is_traversable() {
        adj.push((r - 1, c));
    }

    if c < grid[0].len() - 1 && grid[r][c + 1].is_traversable() {
        adj.push((r, c + 1));
    }

    if c > 0 && grid[r][c - 1].is_traversable() {
        adj.push((r, c - 1));
    }

    adj
}

fn bfs(start: (usize, usize), limit: usize, grid: &mut Vec<Vec<Tile>>) {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((start, 0));

    while let Some((pos, path)) = queue.pop_front() {
        if path == limit {
            grid[pos.0][pos.1].path = path;
            continue;
        }

        if seen.contains(&(pos, path)) {
            continue;
        }

        seen.insert((pos, path));

        for t in get_adjacent(pos, grid) {
            queue.push_back((t, path + 1));
        }
    }
}

fn solution(input: &str, step_limit: usize) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut grid = Vec::new();
    let mut start = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        grid.push(Vec::new());

        for (j, c) in line.char_indices() {
            match c {
                '#' => grid[i].push(Tile::new(TileType::Rock)),
                '.' => grid[i].push(Tile::new(TileType::Plot)),
                'S' => {
                    start = (i, j);
                    grid[i].push(Tile::new(TileType::Plot))
                }
                _ => panic!(),
            }
        }
    }

    bfs(start, step_limit, &mut grid);

    grid.iter()
        .map(|v| v.iter().filter(|t| t.path == step_limit).count())
        .fold(0, |acc, s| acc + s as u64)
}
