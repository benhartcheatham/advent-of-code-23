use clap::{arg, command, ArgAction};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rule {
    ch: char,
    comp: Ordering,
    val: u64,
}

impl Rule {
    fn new(ch: char, comp: char, val: u64) -> Self {
        let rcomp = match comp {
            '<' => Ordering::Less,
            '>' => Ordering::Greater,
            _ => Ordering::Equal,
        };

        Rule {
            ch,
            comp: rcomp,
            val,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    start: char,
    rules: Vec<(Rule, String)>,
    end: String,
}

impl Workflow {
    fn new(name: &str) -> Self {
        Workflow {
            name: name.to_string(),
            start: '0',
            rules: Vec::new(),
            end: String::new(),
        }
    }

    fn add_rule(&mut self, rule: Rule, dest: &str) {
        if self.rules.is_empty() {
            self.start = rule.ch;
        }

        self.rules.push((rule, dest.to_string()));
    }

    fn set_end(&mut self, end: &str) {
        self.end = end.to_string();
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

fn ch_to_idx(ch: char) -> usize {
    match ch {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("Invalid char for ch_to_idx conversion {}!", ch),
    }
}

fn parse_flows(input: &[&str]) -> HashMap<String, Workflow> {
    let mut flows = HashMap::new();

    for flow in input {
        let parts: Vec<&str> = flow.split('{').collect();
        let name = parts[0];
        let rules: Vec<&str> = parts[1].split(',').collect();

        let mut wf = Workflow::new(name);
        for r in rules.iter().take(rules.len() - 1) {
            let var = r.chars().next().unwrap();
            let comp = r.chars().nth(1).unwrap();
            let val: String = r.chars().skip(2).take_while(|c| *c != ':').collect();
            let rule = Rule::new(var, comp, val.parse::<u64>().unwrap());

            let dest: String = r.chars().rev().take_while(|c| *c != ':').collect();
            let dest: String = dest.chars().rev().collect();
            wf.add_rule(rule, &dest);
        }

        let end: String = rules[rules.len() - 1]
            .chars()
            .take_while(|c| *c != '}')
            .collect();
        wf.set_end(&end);

        flows.insert(wf.name.clone(), wf);
    }

    flows
}

fn run_rules(dest: &str, mut ranges: [[u64; 2]; 4], flows: &HashMap<String, Workflow>) -> u64 {
    match dest {
        "A" => {
            return ranges
                .into_iter()
                .map(|r| r[1].saturating_sub(r[0]))
                .product()
        }
        "R" => return 0,
        _ => (),
    }

    let mut t = 0;
    if let Some(flow) = flows.get(dest) {
        for (r, d) in flow.rules.iter() {
            let range = ranges[ch_to_idx(r.ch)];

            match ((range[0], range[1]), r.comp) {
                ((_, u), Ordering::Less) if u <= r.val => {
                    return t + run_rules(d, ranges, flows);
                }
                ((l, _), Ordering::Greater) if l > r.val => {
                    return t + run_rules(d, ranges, flows);
                }
                ((l, u), Ordering::Less) if l < r.val => {
                    ranges[ch_to_idx(r.ch)] = [l, r.val];
                    t += run_rules(d, ranges, flows);
                    ranges[ch_to_idx(r.ch)] = [r.val, u];
                }
                ((l, u), Ordering::Greater) if u >= r.val => {
                    ranges[ch_to_idx(r.ch)] = [r.val + 1, u];
                    t += run_rules(d, ranges, flows);
                    ranges[ch_to_idx(r.ch)] = [l, r.val + 1];
                }
                _ => (),
            }
        }

        return t + run_rules(&flow.end, ranges, flows);
    }
    0
}

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut ls: Vec<&str> = Vec::new();
    let mut flows: HashMap<String, Workflow> = HashMap::new();

    for line in &lines {
        if line.is_empty() {
            flows = parse_flows(&ls);
            break;
        }

        ls.push(line);
    }

    run_rules("in", [[1, 4001]; 4], &flows)
}
