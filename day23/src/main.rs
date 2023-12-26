use clap::{arg, command, ArgAction};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    io,
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum TileType {
    Path,
    Forest,
    Slope(char),
}

impl TileType {
    fn is_passable(&self, _direction: Direction) -> bool {
        use TileType::*;

        match self {
            Path => true,
            Forest => false,
            Slope(_) => true,
        }
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TileType::*;

        write!(
            f,
            "{}",
            match self {
                Path => '.',
                Forest => '#',
                Slope(c) => *c,
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Node {
    idx: usize,
    pos: (usize, usize),
    edges: Vec<(usize, i64)>,
}

impl Node {
    fn new(pos: (usize, usize)) -> Self {
        Node {
            idx: 0,
            pos,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, node: usize, weight: i64) {
        self.edges.push((node, weight));
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
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

fn get_adjacent(current: (usize, usize), graph: &[Vec<TileType>]) -> Vec<(usize, usize)> {
    let mut adj = Vec::new();

    if graph[current.0 - 1][current.1].is_passable(Direction::Up) {
        adj.push((current.0 - 1, current.1));
    }

    if graph[current.0 + 1][current.1].is_passable(Direction::Down) {
        adj.push((current.0 + 1, current.1));
    }

    if graph[current.0][current.1 - 1].is_passable(Direction::Left) {
        adj.push((current.0, current.1 - 1));
    }

    if graph[current.0][current.1 + 1].is_passable(Direction::Right) {
        adj.push((current.0, current.1 + 1));
    }

    adj
}

fn create_nodes(start: (usize, usize), end: (usize, usize), tiles: &[Vec<TileType>]) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut idx = 0;
    nodes.push(Node::new(start));
    nodes[0].idx = idx;

    for i in 2..(tiles.len() - 1) {
        for j in 1..(tiles[i].len() - 1) {
            if tiles[i][j] != TileType::Forest && get_adjacent((i, j), tiles).len() > 2 {
                nodes.push(Node::new((i, j)));
                idx += 1;
                nodes[idx].idx = idx;
            }
        }
    }

    nodes.push(Node::new(end));
    idx += 1;
    nodes[idx].idx = idx;
    nodes
}

fn find_adj_intersections(
    inter: (usize, usize),
    tiles: &[Vec<TileType>],
) -> Vec<((usize, usize), u64)> {
    let mut adj_ints = Vec::new();
    let adj = get_adjacent(inter, tiles);

    for a in adj {
        let mut curr = a;
        let mut weight = 1;
        let mut seen = HashSet::new();

        seen.insert(inter);
        loop {
            let mut next = get_adjacent(curr, tiles);
            next.retain(|n| !seen.contains(n));

            seen.insert(curr);
            weight += 1;

            if next.len() != 1 {
                break;
            }

            curr = next.pop().unwrap();
        }

        adj_ints.push((curr, weight));
    }

    adj_ints
}

fn create_edges(nodes: &mut [Node], tiles: &[Vec<TileType>]) {
    let mut adj_ints = HashMap::new();
    for n in nodes.iter() {
        let adj = find_adj_intersections(n.pos, tiles);

        for a in adj {
            adj_ints
                .entry(n.pos)
                .and_modify(|v: &mut Vec<_>| v.push(a))
                .or_insert(vec![a]);
        }
    }

    for (k, v) in adj_ints {
        let idx = nodes.iter().find(|n| n.pos == k).unwrap().idx;

        for (n, w) in v {
            let enode = nodes.iter().find(|en| en.pos == n).unwrap().clone();

            nodes[idx].add_edge(enode.idx, w as i64 - 1);
        }
    }
}

/*
 * I got this from another person's solution. I had the right algorithm, but hashing the
 * Nodes to put into a HashSet and finding the node based on position ever loop
 * caused it to take forever (i.e. I never tried to even let it finish). This uses bit
 * manipulation to mark indexes as seen, and it goes pretty quick!
 */
fn find_longest_path(start: usize, end: usize, nodes: &[Node]) -> u64 {
    let mut max = 0;
    let mut stack = Vec::new();

    stack.push((start, 0, 0i64));

    while let Some((node, steps, vis)) = stack.pop() {
        if node == end {
            max = max.max(steps);
            continue;
        }

        for e in &nodes[node].edges {
            if vis & (1 << e.0) == 0 {
                stack.push((nodes[e.0].idx, steps + e.1, vis | (1 << e.0)));
            }
        }
    }

    max as u64
}

fn solution(input: &str) -> u64 {
    let mut lines: Vec<_> = input.lines().collect();
    let mut graph = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    let buffer = "#".repeat(lines[0].len());
    lines.insert(0, buffer.as_str());
    lines.push(buffer.as_str());

    for (i, line) in lines.iter().enumerate() {
        graph.push(Vec::new());

        for (j, c) in line.char_indices() {
            match c {
                '.' => {
                    if i == 1 {
                        start = (i, j);
                    }

                    if i == lines.len() - 2 {
                        end = (i, j);
                    }

                    graph[i].push(TileType::Path)
                }
                '#' => graph[i].push(TileType::Forest),
                _ => graph[i].push(TileType::Slope(c)),
            }
        }
    }

    let mut nodes = create_nodes(start, end, &graph);
    create_edges(&mut nodes, &graph);

    let start = nodes.iter().find(|n| n.pos == start).unwrap();
    let end = nodes.iter().find(|n| n.pos == end).unwrap();
    find_longest_path(start.idx, end.idx, &nodes)
}
