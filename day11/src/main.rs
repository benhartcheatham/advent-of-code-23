use std::collections::HashSet;
use std::fmt::Display;
use std::io;

const EXPANSION_CONST: usize = 1_000_000;

#[derive(Debug)]
#[allow(unused)]
struct Galaxy {
    // for debugging
    id: usize,
    coords: (usize, usize),
}

impl Galaxy {
    fn new(id: usize, coords: (usize, usize)) -> Self {
        Galaxy { id, coords }
    }

    fn distance(&self, other: &Galaxy) -> u64 {
        let (sr, sc) = (self.coords.0 as i64, self.coords.1 as i64);
        let (or, oc) = (other.coords.0 as i64, other.coords.1 as i64);

        let a = or - sr;
        let b = oc - sc;

        (a.abs() + b.abs()) as u64
    }
}

impl Display for Galaxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.id, self.coords)
    }
}

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");

    println!("solution: {}", solution(input));
    Ok(())
}

fn solution(input: &str) -> u64 {
    let lines = input.lines();
    let mut space: Vec<Galaxy> = Vec::new();
    let mut expand_rows: Vec<usize> = Vec::new();
    let mut expand_columns: HashSet<usize> = HashSet::new();
    let mut sum = 0;

    let mut id = 1;
    for (i, line) in lines.enumerate() {
        let first_id = id;

        for (j, c) in line.char_indices() {
            if c == '#' {
                space.push(Galaxy::new(id, (i, j)));
                expand_columns.insert(j);
                id += 1;
            }
        }

        if id == first_id {
            expand_rows.push(i);
        }
    }

    let expand_columns: Vec<usize> = (0..input.lines().next().unwrap().len())
        .filter(|i| !expand_columns.contains(i))
        .collect();

    for (i, r) in expand_rows.iter().enumerate() {
        let expanded = r + i * (EXPANSION_CONST - 1);

        for g in space.iter_mut() {
            if g.coords.0 > expanded {
                g.coords.0 += EXPANSION_CONST - 1;
            }
        }
    }

    for (j, c) in expand_columns.iter().enumerate() {
        let expanded = c + j * (EXPANSION_CONST - 1);

        for g in space.iter_mut() {
            if g.coords.1 > expanded {
                g.coords.1 += EXPANSION_CONST - 1;
            }
        }
    }

    for i in 0..(space.len() - 1) {
        for j in (i + 1)..space.len() {
            let distance = space[i].distance(&space[j]);
            sum += distance;
        }
    }

    sum
}
