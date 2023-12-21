use clap::{arg, command, ArgAction};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Rule {
    ch: char,
    comp: Ordering,
    val: u64,
    negate: bool,
}

impl Rule {
    fn new(ch: char, comp: Ordering, val: u64, negate: bool) -> Self {
        Rule {
            ch,
            comp,
            val,
            negate,
        }
    }

    fn apply(&self, val: u64) -> bool {
        if self.negate {
            val.cmp(&self.val) != self.comp
        } else {
            val.cmp(&self.val) == self.comp
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.ch,
            if self.negate {
                match self.comp {
                    Ordering::Greater => "<=",
                    Ordering::Less => ">=",
                    _ => "INVALID",
                }
            } else {
                match self.comp {
                    Ordering::Greater => ">",
                    Ordering::Less => "<",
                    _ => "INVALID",
                }
            },
            self.val
        )
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    start: char,
    rules: Vec<(Rule, String)>,
    end: String,
    visited: bool,
}

impl Workflow {
    fn new(name: &str) -> Self {
        Workflow {
            name: name.to_string(),
            start: '0',
            rules: Vec::new(),
            end: String::new(),
            visited: false,
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

fn parse_flows(input: &[&str]) -> HashMap<String, Workflow> {
    let mut flows = HashMap::new();

    for flow in input {
        let parts: Vec<&str> = flow.split('{').collect();
        let name = parts[0];
        let rules: Vec<&str> = parts[1].split(',').collect();

        let mut wf = Workflow::new(name);
        for r in rules.iter().take(rules.len() - 1) {
            let var = r.chars().next().unwrap();
            let comp = match r.chars().nth(1).unwrap() {
                '<' => Ordering::Less,
                '>' => Ordering::Greater,
                _ => Ordering::Equal,
            };

            let val: String = r.chars().skip(2).take_while(|c| *c != ':').collect();
            let rule = Rule::new(var, comp, val.parse::<u64>().unwrap(), false);

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

fn make_rules(workflow: &str, ends: &mut Vec<Vec<Rule>>, flows: &mut HashMap<String, Workflow>) {
    let mut to_visit: Vec<(String, Vec<(Rule, bool)>)> = Vec::new();

    to_visit.push((workflow.to_string(), Vec::new()));

    while let Some((name, mut path)) = to_visit.pop() {
        let flow = flows.get_mut(&name);

        if let Some(f) = flow {
            if f.visited {
                continue;
            }

            for (i, (r, d)) in f.rules.iter().enumerate() {
                if d.as_str() == "R" {
                    continue;
                }

                let mut p = path.clone();
                if i != 0 {
                    for (pr, _) in f.rules.iter().take(i) {
                        p.push((*pr, false));
                    }
                }

                let mut p1 = p.clone();
                let mut p2 = p.clone();

                p1.push((*r, true));

                p2.push((*r, false));
                to_visit.push((d.to_string(), p2));

                if d.as_str() == "A" {
                    ends.push(
                        p1.iter()
                            .map(|(r, b)| {
                                if *b {
                                    *r
                                } else {
                                    Rule::new(r.ch, r.comp, r.val, true)
                                }
                            })
                            .collect(),
                    );
                } else {
                    to_visit.push((d.to_string(), p1));
                }
            }

            if f.end.as_str() == "R" {
                continue;
            }

            for (r, _) in &f.rules {
                path.push((*r, false));
            }

            if f.end.as_str() == "A" {
                ends.push(
                    path.iter()
                        .map(|(r, b)| {
                            if *b {
                                *r
                            } else {
                                Rule::new(r.ch, r.comp, r.val, true)
                            }
                        })
                        .collect(),
                );
            } else {
                to_visit.push((f.end.clone(), path));
            }
            f.visited = true;
        }
    }
}

fn find_combos(rules: &[Rule]) -> u64 {
    let mut combos = 1;

    for c in ['x', 'm', 'a', 's'] {
        let rs: Vec<Rule> = rules.iter().filter(|r| r.ch == c).cloned().collect();
        let mut letter_combo = 0;

        for i in 1..=4000 {
            if rs.iter().all(|r| r.apply(i)) {
                letter_combo += 1;
            }
        }

        if rs.is_empty() {
            letter_combo = 4000;
        }

        println!("{}: {}", c, letter_combo);
        combos *= letter_combo;
    }

    combos
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

    let mut rules = Vec::new();
    make_rules("in", &mut rules, &mut flows);
    println!();

    let mut sum = 0;
    for v in rules {
        print!("[");
        for r in v.iter().take(v.len() - 1) {
            print!("{}, ", r);
        }

        print!("{}", v[v.len() - 1]);
        println!("]: ");
        let combos = find_combos(&v);
        sum += combos;
    }

    let rule = Rule::new('a', Ordering::Less, 3022, false);
    println!("{}: {}", rule, rule.apply(3021));
    sum
}
