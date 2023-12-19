use clap::{arg, command, ArgAction};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::io;

/// Directions from perspective of grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq)]
struct Vertex {
    row: usize,
    col: usize,
    dir: Direction,
    steps: usize,
    cost: u64,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
            && self.col == other.col
            && self.dir == other.dir
            && self.steps == other.steps
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Vertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
        self.dir.hash(state);
        self.steps.hash(state);
        self.cost.hash(state);
    }
}

impl Vertex {
    fn new(row: usize, col: usize, dir: Direction, steps: usize, cost: u64) -> Self {
        Vertex {
            row,
            col,
            dir,
            steps,
            cost,
        }
    }

    fn get_edges(&self, graph: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();

        if self.row > 0 && self.dir != Direction::Down {
            edges.push((self.row - 1, self.col));
        }

        if self.row < graph.len() - 1 && self.dir != Direction::Up {
            edges.push((self.row + 1, self.col));
        }

        if self.col > 0 && self.dir != Direction::Right {
            edges.push((self.row, self.col - 1));
        }

        if self.col < graph[0].len() - 1 && self.dir != Direction::Left {
            edges.push((self.row, self.col + 1));
        }

        edges
    }

    fn get_direction(&self, v1: (usize, usize)) -> Direction {
        let (dr, dc) = (v1.0 as i64 - self.row as i64, v1.1 as i64 - self.col as i64);

        match dr {
            -1 => Direction::Up,
            1 => Direction::Down,
            _ => match dc {
                -1 => Direction::Left,
                1 => Direction::Right,
                _ => panic!("invalid direction!"),
            },
        }
    }

    fn get_coords(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn can_turn(&self, turn_dir: Direction, weights: &Vec<Vec<u64>>) -> bool {
        use Direction::*;

        match (self.dir, turn_dir) {
            (Up, Right) | (Down, Right) => self.col < weights[0].len() - 1,
            (Up, Left) | (Down, Left) => self.col > 0,
            (Right, Up) | (Left, Up) => self.row > 0,
            (Right, Down) | (Left, Down) => self.row < weights.len(),
            _ => false,
        }
    }

    /// Computes the vertex as if we turned *and moved* in turn_dir
    fn turn(&self, turn_dir: Direction, weights: &[Vec<u64>]) -> Vertex {
        use Direction::*;

        match turn_dir {
            Up => Vertex::new(
                self.row - 1,
                self.col,
                Up,
                1,
                self.cost + weights[self.row - 1][self.col],
            ),
            Down => Vertex::new(
                self.row + 1,
                self.col,
                Down,
                1,
                self.cost + weights[self.row + 1][self.col],
            ),
            Left => Vertex::new(
                self.row,
                self.col - 1,
                Left,
                1,
                self.cost + weights[self.row][self.col - 1],
            ),
            Right => Vertex::new(
                self.row,
                self.col + 1,
                Right,
                1,
                self.cost + weights[self.row][self.col + 1],
            ),
        }
    }

    /// Computes vertex as if we moved foward by 1 in the same direction
    fn go_straight(&self, weights: &Vec<Vec<u64>>) -> Option<Vertex> {
        use Direction::*;

        if self.steps == 3 {
            return None;
        }

        let (r, c) = match self.dir {
            Up => {
                if self.row == 0 {
                    return None;
                } else {
                    (self.row - 1, self.col)
                }
            }
            Down => {
                if self.row >= weights.len() - 1 {
                    return None;
                } else {
                    (self.row + 1, self.col)
                }
            }
            Left => {
                if self.col == 0 {
                    return None;
                } else {
                    (self.row, self.col - 1)
                }
            }
            Right => {
                if self.col >= weights[0].len() - 1 {
                    return None;
                } else {
                    (self.row, self.col + 1)
                }
            }
        };

        Some(Vertex::new(
            r,
            c,
            self.dir,
            self.steps + 1,
            self.cost + weights[r][c],
        ))
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

fn find_path(start: (usize, usize), target: (usize, usize), weights: &Vec<Vec<u64>>) -> u64 {
    let mut queue: BinaryHeap<Vertex> = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize), u64> = HashMap::new();
    let mut seen: HashSet<((usize, usize), Direction, usize)> = HashSet::new();

    queue.push(Vertex::new(start.0, start.1, Direction::Right, 0, 0));

    while !queue.is_empty() {
        let u = queue.pop().unwrap();
        if seen.get(&(u.get_coords(), u.dir, u.steps)).is_some() {
            continue;
        } else {
            seen.insert((u.get_coords(), u.dir, u.steps));
        }

        for e in u.get_edges(weights) {
            let e_dir = u.get_direction(e);

            if u.dir != e_dir && u.can_turn(e_dir, weights) {
                queue.push(u.turn(e_dir, weights));
            }

            if let Some(v) = u.go_straight(weights) {
                dist.entry(v.get_coords())
                    .and_modify(|c| {
                        if v.cost < *c {
                            *c = v.cost
                        }
                    })
                    .or_insert(v.cost);
            } else if u.get_coords() == target {
                dist.entry(u.get_coords())
                    .and_modify(|c| {
                        if u.cost < *c {
                            *c = u.cost
                        }
                    })
                    .or_insert(u.cost);
            }
        }
    }

    *dist.get(&target).unwrap()
}

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut weights: Vec<Vec<u64>> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        weights.push(Vec::new());
        for w in line.chars().filter_map(|c| c.to_digit(10)) {
            weights[i].push(w as u64);
        }
    }

    find_path((0, 0), (weights.len() - 1, weights[0].len() - 1), &weights)
}
