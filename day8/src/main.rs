use std::collections::HashMap;
use std::io;

#[derive(Debug)]
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
    let instructions = lines[0].trim().chars().cycle();
    let mut graph: HashMap<String, Node> = HashMap::new();

    for line in lines.iter().skip(1) {
        let l: Vec<String> = line
            .split_whitespace()
            .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect::<String>())
            .filter(|s| !s.is_empty())
            .collect();

        if l.len() != 3 {
            continue;
        }

        let name = &l[0];
        graph.insert(
            name.clone(),
            Node::new(name.clone(), l[1].clone(), l[2].clone()),
        );
    }

    let mut steps = 1;
    let mut current = graph.get("AAA").unwrap();
    for i in instructions {
        match i {
            'L' => current = graph.get(&current.left).unwrap(),
            'R' => current = graph.get(&current.right).unwrap(),
            _ => panic!("Invalid input!"),
        }

        if current.name == "ZZZ" {
            return steps;
        }

        steps += 1;
    }

    steps
}
