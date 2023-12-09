use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(name: String, left: String, right: String) -> Self {
        Node { name, left, right }
    }
}

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");

    println!("solution: {}", solution(input));
    Ok(())
}

fn solution(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let instructions = lines[0].trim();
    let mut graph: HashMap<String, Node> = HashMap::new();
    let mut start: Vec<Node> = Vec::new();

    for line in lines.iter().skip(1) {
        let l: Vec<String> = line
            .split_whitespace()
            .map(|s| {
                s.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .filter(|s| !s.is_empty())
            .collect();

        if l.len() != 3 {
            continue;
        }

        let name = &l[0];
        if name.ends_with('A') {
            start.push(Node::new(name.clone(), l[1].clone(), l[2].clone()));
        }

        graph.insert(
            name.clone(),
            Node::new(name.clone(), l[1].clone(), l[2].clone()),
        );
    }

    let mut path_steps: Vec<u64> = Vec::new();
    for s in start.iter_mut() {
        let mut steps = 1;
        for i in instructions.chars().cycle() {
            let next = if i == 'L' { &s.left } else { &s.right };
            *s = graph.get(next).unwrap().clone();

            if s.name.ends_with('Z') {
                path_steps.push(steps);
                break;
            }

            steps += 1;
        }
    }

    for i in 2..=u64::MAX {
        if path_steps.iter().all(|n| i % n == 0) {
            return i as u32;
        }
    }

    0
}
