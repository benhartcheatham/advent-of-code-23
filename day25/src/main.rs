use clap::{arg, command, ArgAction};
use std::{fmt::Display, io};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    id: usize,
    name: String,
}

impl Node {
    fn new(id: usize, name: &str) -> Self {
        Node {
            id,
            name: name.to_string(),
        }
    }
}

#[derive(Clone)]
struct Edge {
    n0: usize,
    n1: usize,
}

impl Edge {
    fn new(n0: &Node, n1: &Node) -> Self {
        Edge {
            n0: n0.id,
            n1: n1.id,
        }
    }

    fn contains_node(&self, id: usize) -> bool {
        self.n0 == id || self.n1 == id
    }

    fn is_between(&self, id0: usize, id1: usize) -> bool {
        self.n0 == id0 && self.n1 == id1 || self.n0 == id1 && self.n1 == id0
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.n0 == other.n0 && self.n1 == other.n1 || self.n0 == other.n1 && self.n1 == other.n0
    }
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, name: &str) -> Option<usize> {
        let id = self.nodes.len();
        let node = Node::new(id, name);

        for n in self.nodes.iter().map(|n| n.name.clone()) {
            if n.as_str() == name {
                return None;
            }
        }

        self.nodes.push(node);
        Some(id)
    }

    fn find_node(&self, name: &str) -> Option<usize> {
        for n in self.nodes.iter() {
            if n.name.as_str() == name {
                return Some(n.id);
            }
        }

        None
    }

    fn get_node_external(&self, id: usize, component: &[Node]) -> usize {
        let mut num = 0;
        for e in &self.edges {
            if e.contains_node(id) {
                for id1 in component.iter().map(|n| n.id) {
                    if e.is_between(id, id1) {
                        num += 1;
                    }
                }
            }
        }

        num
    }

    fn add_edge(&mut self, id0: usize, id1: usize) -> bool {
        if id0 > self.nodes.len() - 1 || id1 > self.nodes.len() - 1 || id0 == id1 {
            return false;
        }

        let new_edge = Edge::new(&self.nodes[id0], &self.nodes[id1]);
        for e in &self.edges {
            if e == &new_edge {
                return false;
            }
        }

        self.edges.push(new_edge);
        true
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in &self.nodes {
            let nid = n.id;
            let nname = &n.name;
            write!(f, "{}({}) -> ", nname, nid)?;

            for e in &self.edges {
                if e.n0 == nid {
                    write!(f, "{} ", self.nodes[e.n1].id)?;
                } else if e.n1 == nid {
                    write!(f, "{} ", self.nodes[e.n0].id)?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
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

// https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/ketzp94/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut graph = Graph::new();

    for line in lines {
        let (name, edges) = line.split_once(':').unwrap();

        let nid = graph
            .add_node(name)
            .unwrap_or_else(|| graph.find_node(name).unwrap());

        for ename in edges.split_whitespace().map(|s| s.trim()) {
            let eid = graph
                .add_node(ename)
                .unwrap_or_else(|| graph.find_node(ename).unwrap());

            graph.add_edge(nid, eid);
        }
    }

    let mut component = Vec::new();
    let mut g_component = graph.nodes.clone();
    let count = |gcomp: &Vec<_>, comp: Vec<Node>| {
        gcomp
            .iter()
            .map(|gn: &Node| graph.get_node_external(gn.id, &comp))
            .sum::<usize>()
    };

    while count(&g_component, component.clone()) != 3 {
        let mut max = 0;
        let mut idx = 0;
        for (i, gn) in g_component.iter().enumerate() {
            let num = graph.get_node_external(gn.id, &component);
            if max < num {
                max = num;
                idx = i;
            }
        }

        component.push(g_component.remove(idx));
    }

    (component.len() * g_component.len()) as u64
}
