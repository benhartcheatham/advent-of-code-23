use std::io;

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");

    println!("solution: {}", solution(input));
    Ok(())
}

fn compute_derivative(line: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::new();
    let mut deriv = Vec::new();
    let mut curr = 0;

    ret.push(line);
    while !ret[curr].iter().all(|n| *n == 0) {
        deriv.clear();

        for i in 0..(ret[curr].len() - 1) {
            deriv.push(ret[curr][i + 1] - ret[curr][i]);
        }

        ret.push(deriv.clone());
        curr += 1;
    }

    ret
}

fn solution(input: &str) -> i32 {
    let lines = input.lines();
    let mut ret = Vec::new();

    for line in lines {
        let mut derivatives: Vec<Vec<i32>> = compute_derivative(
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect(),
        )
        .iter()
        .map(|v| v.iter().copied().rev().collect())
        .collect();

        for i in (1..derivatives.len()).rev() {
            let di_len = derivatives[i].len();
            let ni = derivatives[i][di_len - 1];
            let dj = &mut derivatives[i - 1];

            dj.push(dj.last().unwrap() - ni);
        }

        ret.push(*derivatives[0].last().unwrap());
    }

    ret.iter().sum()
}
