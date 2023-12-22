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

    let matches = command!()
        .arg(arg!(example: -e).action(ArgAction::SetTrue))
        .get_matches();

    if matches.get_flag("example") {
        panic!("solution to part 2 will not work with the example!");
    } else {
        println!("solution: {}", solution(input, 26501365));
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
        grid[pos.0][pos.1].path = path;
        if path == limit {
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

/// https://en.wikipedia.org/wiki/Divided_differences
fn div_diff(points: &[(f64, f64)], k: usize) -> f64 {
    if k == 0 {
        return points[0].1;
    }

    let k_0 = (points[k].1 - points[k - 1].1) / (points[k].0 - points[k - 1].0);

    if k == 1 {
        return k_0;
    }

    (points[k].1 - points[k - 1].1 - div_diff(points, k - 1)) / (points[k].0 - points[0].0)
}

/// https://en.wikipedia.org/wiki/Newton_polynomial
fn poly_fit(points: &[(f64, f64)], x: f64) -> f64 {
    let poly = |x: f64| {
        div_diff(points, 0)
            + div_diff(points, 1) * (x - points[0].0)
            + div_diff(points, 2) * (x - points[0].0) * (x - points[1].0)
    };

    poly(x)
}

/// https://www.reddit.com/r/adventofcode/comments/18nevo3/comment/keaiiq7/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
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

    let grid_limit = grid.len();
    assert!(grid.len() == grid[0].len());

    let remainder = step_limit % grid_limit;
    assert!(remainder == 65);

    let mut fn_vals = [0, 0, 0];

    for (i, v) in fn_vals.iter_mut().enumerate() {
        let mut fn_grid = grid.clone();

        if i != 0 {
            for r in 0..fn_grid.len() {
                for _ in 0..(u64::pow(2, i as u32) as usize) {
                    fn_grid[r].append(&mut grid[r].clone())
                }
            }

            let temp = fn_grid.clone();

            for _ in 0..(u64::pow(2, i as u32) as usize) {
                fn_grid.append(&mut temp.clone());
            }
        }
        assert!(fn_grid.len() == fn_grid[0].len());

        let search = grid_limit * i;
        let start = (start.0 + search, start.1 + search);
        bfs(start, remainder + search, &mut fn_grid);

        *v = fn_grid
            .iter()
            .map(|v| v.iter().filter(|t| t.path == remainder + search).count())
            .fold(0, |acc, s| acc + s as u64);
    }

    let fn_vals: Vec<_> = fn_vals
        .iter()
        .enumerate()
        .map(|(i, v)| (i as f64, *v as f64))
        .collect();

    poly_fit(&fn_vals, (step_limit / grid_limit) as f64) as u64
}
