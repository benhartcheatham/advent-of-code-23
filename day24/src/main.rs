use clap::{arg, command, ArgAction};
use nalgebra::{matrix, ArrayStorage, Const, Matrix};
use std::io;

type Matrix2d = Matrix<f64, Const<2>, Const<2>, ArrayStorage<f64, 2, 2>>;
type Vec2d = Matrix<f64, Const<2>, Const<1>, ArrayStorage<f64, 2, 1>>;

#[derive(Debug)]
struct Hailstone {
    pos: Vec<f64>,
    vel: Vec<f64>,
}

#[allow(unused)]
impl Hailstone {
    fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            pos: vec![x, y, z],
            vel: vec![vx, vy, vz],
        }
    }

    fn from_vecs(pos: &[f64], vel: &[f64]) -> Option<Self> {
        if pos.len() != 3 || vel.len() != 3 {
            return None;
        }

        Some(Self {
            pos: pos.to_vec(),
            vel: vel.to_vec(),
        })
    }

    fn in_bounds(&self, lower: f64, upper: f64, t: f64) -> bool {
        let (x, y) = (
            self.pos[0] + (self.vel[0] * t),
            self.pos[1] + (self.vel[1] * t),
        );

        (x - lower > 1e-10 && upper - x > -1e-10) && (y - lower > 1e-10 && upper - y > -1e-10)
    }
}

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");
    let example = include_str!("../../example.txt");

    let matches = command!()
        .arg(arg!(example: -e).action(ArgAction::SetTrue))
        .get_matches();

    if matches.get_flag("example") {
        println!("solution (example): {}", solution(7, 27, example));
    } else {
        println!(
            "solution: {}",
            solution(200_000_000_000_000, 400_000_000_000_000, input)
        );
    }

    Ok(())
}

fn get_2d_matrix(stone0: &Hailstone, stone1: &Hailstone) -> (Matrix2d, Vec2d) {
    let b = matrix![stone1.pos[0] - stone0.pos[0]; stone1.pos[1] - stone0.pos[1]];
    let m = matrix![stone0.vel[0], -stone1.vel[0]; stone0.vel[1], -stone1.vel[1]];

    (m, b)
}

fn find_solutions(lower: f64, upper: f64, stones: &[Hailstone]) -> u64 {
    let mut sum = 0;

    for i in 0..(stones.len() - 1) {
        for j in (i + 1)..stones.len() {
            let (m, b) = get_2d_matrix(&stones[i], &stones[j]);
            let lu = m.full_piv_lu();

            let Some(ts) = lu.solve(&b) else {
                continue;
            };

            if ts[(0, 0)] < 0.0 || ts[(1, 0)] < 0.0 {
                continue;
            }

            if stones[i].in_bounds(lower, upper, ts[(0, 0)]) {
                sum += 1;
            }
        }
    }

    sum
}

fn solution(lower: u64, upper: u64, input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut stones = Vec::new();

    for line in lines {
        let (pos, vel) = line.split_once('@').unwrap();

        let pvec: Vec<f64> = pos
            .split(',')
            .map(|s| {
                let st = s.trim();
                st.parse::<f64>().unwrap()
            })
            .collect();

        let vvec: Vec<f64> = vel
            .split(',')
            .map(|s| {
                let st = s.trim();
                st.parse::<f64>().unwrap()
            })
            .collect();

        stones.push(Hailstone::from_vecs(&pvec, &vvec).unwrap());
    }

    find_solutions(lower as f64, upper as f64, &stones)
}
