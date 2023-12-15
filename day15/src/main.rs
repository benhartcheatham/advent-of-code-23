use clap::{arg, command, ArgAction};
use std::collections::HashMap;
use std::io;

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

fn hash(input: &str) -> u64 {
    input.chars().fold(0, |out, c| (out + c as u64) * 17 % 256)
}

fn insert_lens(lens: &str, boxes: &mut HashMap<u64, Vec<(String, u64)>>) {
    let label: String = lens.chars().filter(|c| c.is_alphabetic()).collect();
    let op: String = lens.chars().filter(|c| !c.is_alphabetic()).collect();

    if lens.contains('-') {
        if let Some(b) = boxes.get_mut(&hash(&label)) {
            for idx in 0..b.len() {
                if b[idx].0 == label {
                    b.remove(idx);
                    break;
                }
            }
        }
    } else {
        let n: u64 = op
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        boxes
            .entry(hash(&label))
            .and_modify(|b| {
                let mut contains = false;
                for l in b.iter_mut() {
                    if l.0 == label {
                        *l = (label.clone(), n);
                        contains = true;
                        break;
                    }
                }

                if !contains {
                    b.push((label.clone(), n));
                }
            })
            .or_insert(vec![(label, n)]);
    }
}

fn solution(input: &str) -> u64 {
    let sequence: Vec<_> = input.trim().split(',').collect();

    let mut boxes = HashMap::new();
    for s in sequence {
        insert_lens(s, &mut boxes);
    }

    boxes.iter().fold(0, |pow, (b, v)| {
        pow + v
            .iter()
            .enumerate()
            .map(|(i, l)| (1 + b) * (i as u64 + 1) * l.1)
            .sum::<u64>()
    })
}
