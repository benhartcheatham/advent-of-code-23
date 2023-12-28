use clap::{arg, command, ArgAction};
use nalgebra::{matrix, vector, Matrix3, Matrix6, Vector3};
use std::io;

#[derive(Debug)]
struct Hailstone {
    pos: Vector3<f64>,
    vel: Vector3<f64>,
}

impl Hailstone {
    fn from_vecs(pos: &[f64], vel: &[f64]) -> Option<Self> {
        if pos.len() != 3 || vel.len() != 3 {
            return None;
        }

        Some(Self {
            pos: vector![pos[0], pos[1], pos[2]],
            vel: vector![vel[0], vel[1], vel[2]],
        })
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

fn skew_matrix(v: &Vector3<f64>) -> Matrix3<f64> {
    matrix![0.0, -v[2], v[1];
            v[2], 0.0, -v[0];
            -v[1], v[0], 0.0]
}

fn find_solution(stones: &[Hailstone]) -> u64 {
    let v0 = -stones[0].pos.cross(&stones[0].vel) + stones[1].pos.cross(&stones[1].vel);
    let v1 = -stones[0].pos.cross(&stones[0].vel) + stones[2].pos.cross(&stones[2].vel);
    let rhs = matrix![v0[0]; v0[1]; v0[2]; v1[0]; v1[1]; v1[2]];

    let m00 = skew_matrix(&stones[0].vel) - skew_matrix(&stones[1].vel);
    let m30 = skew_matrix(&stones[0].vel) - skew_matrix(&stones[2].vel);
    let m03 = -skew_matrix(&stones[0].pos) + skew_matrix(&stones[1].pos);
    let m33 = -skew_matrix(&stones[0].pos) + skew_matrix(&stones[2].pos);
    let m: Matrix6<f64> = matrix![ m00[(0, 0)], m00[(0, 1)], m00[(0, 2)], m03[(0, 0)], m03[(0, 1)], m03[(0, 2)];
       m00[(1, 0)], m00[(1, 1)], m00[(1, 2)], m03[(1, 0)], m03[(1, 1)], m03[(1, 2)];
       m00[(2, 0)], m00[(2, 1)], m00[(2, 2)], m03[(2, 0)], m03[(2, 1)], m03[(2, 2)];
       m30[(0, 0)], m30[(0, 1)], m30[(0, 2)], m33[(0, 0)], m33[(0, 1)], m33[(0, 2)];
       m30[(1, 0)], m30[(1, 1)], m30[(1, 2)], m33[(1, 0)], m33[(1, 1)], m33[(1, 2)];
       m30[(2, 0)], m30[(2, 1)], m30[(2, 2)], m33[(2, 0)], m33[(2, 1)], m33[(2, 2)];
    ];

    let res = m.try_inverse().unwrap() * rhs;

    let mut sum = 0;
    for i in 0..3 {
        sum += res[(i, 0)].round() as u64;
    }

    sum
}

fn solution(input: &str) -> u64 {
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

    find_solution(&stones)
}
