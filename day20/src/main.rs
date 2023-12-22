use clap::{arg, command, ArgAction};
use gcd::Gcd;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ModType {
    FlipFlop,
    Conjunction,
    /// Button/Output are modeled by broadcast type
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module {
    id: usize,
    name: String,
    mtype: ModType,

    state: bool,
    conj_state: HashMap<usize, bool>,

    pulses: Vec<(usize, bool)>,

    incoming: Vec<usize>,
    outgoing: Vec<usize>,
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.mtype == other.mtype
            && self.state == other.state
            && self.incoming == other.incoming
            && self.outgoing == other.outgoing
    }
}

impl Eq for Module {}

impl Hash for Module {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.mtype.hash(state);
        self.state.hash(state);
        self.incoming.hash(state);
        self.outgoing.hash(state);
    }
}

impl Module {
    fn new(id: usize, name: &str, mtype: ModType, state: bool) -> Self {
        Module {
            id,
            name: name.to_string(),
            mtype,
            state,
            conj_state: HashMap::new(),
            pulses: Vec::new(),
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }

    fn send_signal(&self, pulse: bool) -> Option<VecDeque<((usize, usize), bool)>> {
        let mut to_process = VecDeque::new();

        for id in &self.outgoing {
            to_process.push_back(((self.id, *id), pulse));
        }

        Some(to_process)
    }

    fn process_signal(
        id: usize,
        modules: &mut [Module],
        iter: usize,
        watch: &mut HashMap<usize, usize>,
    ) -> Option<VecDeque<((usize, usize), bool)>> {
        if let Some(module) = modules.get_mut(id) {
            if let Some((idx, pulse)) = module.pulses.pop() {
                match module.mtype {
                    ModType::FlipFlop => {
                        if !pulse {
                            module.state = !module.state;
                            module.send_signal(module.state)
                        } else {
                            None
                        }
                    }
                    ModType::Conjunction => {
                        module.conj_state.entry(idx).and_modify(|v| *v = pulse);
                        let signal = !module.conj_state.values().all(|b| *b);

                        if signal {
                            watch.entry(module.id).and_modify(|v| {
                                if *v == 0 {
                                    *v = iter
                                }
                            });
                        }

                        module.send_signal(!module.conj_state.values().all(|b| *b))
                    }
                    ModType::Broadcast => module.send_signal(pulse),
                }
            } else {
                None
            }
        } else {
            None
        }
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

fn run_module_loop(modules: &mut [Module], iter: usize, watch: &mut HashMap<usize, usize>) {
    let mut to_process = VecDeque::new();
    for s in modules[0].send_signal(false).unwrap() {
        to_process.push_back(s);
    }

    while let Some(((in_id, out_id), pulse)) = to_process.pop_front() {
        if in_id == out_id {
            for s in Module::process_signal(in_id, modules, iter, watch)
                .iter()
                .flatten()
            {
                to_process.push_front(*s);
            }

            continue;
        }

        let module = &mut modules[out_id];
        module.pulses.push((in_id, pulse));

        for s in Module::process_signal(out_id, modules, iter, watch)
            .iter()
            .flatten()
        {
            to_process.push_back(*s);
        }

        if !modules[in_id].pulses.is_empty() {
            to_process.push_front(((in_id, out_id), pulse));
        }
    }
}

pub fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut modules = Vec::new();
    let mut cons: Vec<(usize, Vec<&str>)> = Vec::new();

    let mut broadcaster_id = 0;
    let mut id = 1;
    for line in lines {
        let (name, out) = line.split_once("->").unwrap();

        let mtype = match name.trim() {
            "broadcaster" => ModType::Broadcast,
            _ => match name.chars().next() {
                Some('%') => ModType::FlipFlop,
                Some('&') => ModType::Conjunction,
                _ => panic!("Invalid name: {}!", name.trim()),
            },
        };

        if mtype == ModType::Broadcast {
            broadcaster_id = id;
            modules.push(Module::new(id, name.trim(), mtype, false));
        } else {
            modules.push(Module::new(
                id,
                name.chars().skip(1).collect::<String>().trim(),
                mtype,
                false,
            ));
        }

        cons.push((id, out.split(',').map(|s| s.trim()).collect()));
        id += 1;
    }

    modules.insert(0, Module::new(0, "button", ModType::Broadcast, false));
    modules[0].outgoing.push(broadcaster_id);

    for (id, out) in cons {
        for o in out.iter().map(|s| s.trim()) {
            let mod_id;
            if let Some(m) = modules.iter_mut().find(|m| m.name.as_str() == o) {
                if m.mtype == ModType::Conjunction {
                    m.conj_state.insert(id, false);
                }

                m.incoming.push(id);
                mod_id = Some(m.id);
            } else {
                modules.insert(
                    modules.len(),
                    Module::new(modules.len(), o, ModType::Broadcast, false),
                );

                let len = modules.len() - 1;
                modules[len].incoming.push(id);
                mod_id = Some(len);
            }

            if let Some(m_id) = mod_id {
                modules[id].outgoing.push(m_id);
            }
        }
    }

    let rx = modules.iter().find(|m| m.name.as_str() == "rx").unwrap();
    let rx_parent = if rx.incoming.len() == 1 {
        rx.incoming[0]
    } else {
        panic!()
    };
    let mut to_watch = HashMap::new();

    for mod_id in modules[rx_parent].incoming.clone() {
        to_watch.insert(mod_id, 0);
    }

    let mut iterations = 1;
    loop {
        run_module_loop(&mut modules, iterations, &mut to_watch);
        iterations += 1;

        if to_watch.values().all(|v| *v != 0) {
            break;
        }
    }

    let lcm = |a: usize, b: usize| a * (b / a.gcd(b));
    to_watch.into_values().reduce(lcm).unwrap() as u64
}
