use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Pipe {
    pos: (usize, usize),
    /// Components are in order of: N,S,E,W
    components: [bool; 4],
    step_cnt: usize,
    edges: Vec<(usize, usize)>,
    is_start: bool,
    marked: bool,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_start {
            return write!(f, "S");
        }

        let c = match self.components {
            [true, true, false, false] => '┃',
            [true, false, true, false] => '┗',
            [true, false, false, true] => '┛',
            [false, true, true, false] => '┏',
            [false, true, false, true] => '┓',
            [false, false, true, true] => '━',
            [false, false, false, false] => '.',
            _ => '.',
        };

        write!(f, "{}", c)
    }
}

impl Pipe {
    pub fn new(pos: (usize, usize), components: [bool; 4], is_start: bool) -> Self {
        Pipe {
            pos,
            components,
            step_cnt: usize::MAX,
            edges: Vec::new(),
            is_start,
            marked: false,
        }
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn is_intersection(&self) -> bool {
        match self.components {
            // NS "|"
            [true, true, false, false] => true,
            // SE "F"
            [false, true, true, false] => true,
            // SW "7"
            [false, true, false, true] => true,
            _ => false,
        }
    }

    #[allow(unused)]
    pub fn get_steps(&self) -> usize {
        self.step_cnt
    }

    pub fn find_connections(&mut self, pipes: &Vec<Vec<Pipe>>) {
        let (r, c) = (self.pos.0, self.pos.1);
        let mut surround = Vec::new();

        if r > 0 {
            let other = &pipes[r - 1][c];
            if self.components[0] && other.components[1] {
                surround.push(pipes[r - 1][c].pos);
            }
        }

        if r < pipes.len() - 1 {
            let other = &pipes[r + 1][c];

            if self.components[1] && other.components[0] {
                surround.push(pipes[r + 1][c].pos);
            }
        }

        if c > 0 {
            let other = &pipes[r][c - 1];
            if self.components[3] && other.components[2] {
                surround.push(pipes[r][c - 1].pos);
            }
        }

        if c < pipes[0].len() - 1 {
            let other = &pipes[r][c + 1];

            if self.components[2] && other.components[3] {
                surround.push(pipes[r][c + 1].pos);
            }
        }

        self.edges = surround;
    }
}

pub fn find_start_kind(start: (usize, usize), pipes: &Vec<Vec<Pipe>>) -> Pipe {
    let (r, c) = start;
    let mut connections = [false, false, false, false];

    if r > 0 {
        let other = &pipes[r - 1][c];
        if other.components[1] {
            connections[0] = true;
        }
    }

    if r < pipes.len() - 1 {
        let other = &pipes[r + 1][c];

        if other.components[0] {
            connections[1] = true;
        }
    }

    if c > 0 {
        let other = &pipes[r][c - 1];
        if other.components[2] {
            connections[3] = true;
        }
    }

    if c < pipes[0].len() - 1 {
        let other = &pipes[r][c + 1];

        if other.components[3] {
            connections[2] = true;
        }
    }

    Pipe::new(start, connections, true)
}

pub fn traverse_loop(
    start: (usize, usize),
    last: (usize, usize),
    steps: usize,
    pipes: &mut Vec<Vec<Pipe>>,
) {
    let (sr, sc) = start;
    let (lr, lc) = last;

    if sr >= pipes.len() || sc >= pipes[0].len() || lr >= pipes.len() || lc >= pipes[0].len() {
        return;
    }

    let pipe = &mut pipes[sr][sc];

    if pipe.components.iter().all(|c| !*c) || (pipe.is_start && steps > 0) {
        return;
    }

    pipe.marked = true;

    if steps >= pipe.step_cnt {
        return;
    }

    pipe.step_cnt = steps;

    let edges = pipe.edges.clone();
    for e in edges.iter().copied() {
        traverse_loop(e, start, steps + 1, pipes);
    }
}
