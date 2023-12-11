use std::io;

use crate::pipe::*;

mod pipe;

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");

    println!("solution: {}", solution(input));
    Ok(())
}

fn solution(input: &str) -> usize {
    let lines = input.lines();
    let mut pipes: Vec<Vec<Pipe>> = Vec::new();
    let mut start_pos: (usize, usize) = (0, 0);

    println!("Making pipes...");
    for (i, line) in lines.enumerate() {
        pipes.push(Vec::new());

        for (j, c) in line.char_indices() {
            let mut is_start = false;
            let comps = match c {
                '|' => [true, true, false, false],
                '-' => [false, false, true, true],
                'L' => [true, false, true, false],
                'J' => [true, false, false, true],
                '7' => [false, true, false, true],
                'F' => [false, true, true, false],
                '.' => [false; 4],
                _ => [false; 4],
            };

            if c == 'S' {
                is_start = true;
                start_pos = (i, j);
            }

            let new_pipe = Pipe::new((i, j), comps, is_start);
            pipes[i].push(new_pipe);
        }
    }

    println!("Generating starting pipe...");
    let (r, c) = start_pos;
    pipes[r][c] = find_start_kind(start_pos, &pipes);

    println!("Finding connections...");
    for i in 0..pipes.len() {
        for j in 0..pipes[0].len() {
            let pc = pipes.clone();
            let p = &mut pipes[i][j];

            p.find_connections(&pc);
        }
    }

    println!("Traversing loop...");
    traverse_loop(start_pos, start_pos, 0, &mut pipes);

    let mut max = 0;
    for (i, r) in pipes.iter().enumerate() {
        print!("{:4}: ", i);
        for p in r {
            if p.is_marked() {
                print!("{}", p);
                let steps = p.get_steps();

                if steps > max {
                    max = steps;
                }
            } else {
                print!(" ");
            }
        }

        println!();
    }

    (max as f64 / 2.0).round() as usize
}
