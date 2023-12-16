use clap::{arg, command, ArgAction};
use std::io;
use std::thread;

#[derive(Clone, Copy)]
struct Tile {
    energized: bool,
    direction: (i32, i32),
    obstacle: Option<char>,
}

impl Tile {
    fn new(ch: char) -> Self {
        match ch {
            '.' => Tile {
                energized: false,
                direction: (0, 0),
                obstacle: None,
            },
            '|' | '/' | '\\' | '-' => Tile {
                energized: false,
                direction: (0, 0),
                obstacle: Some(ch),
            },
            _ => panic!("Invalid input to Tile constructor: {}", ch),
        }
    }

    /// Takes a LightBeam and returns the new direction after passing through this Tile.
    /// Tile will become energized if it doesn't contain an obstacle.
    ///
    /// Arguments
    /// `@beam:` LightBeam to use for calculations
    ///
    /// `returns:` Direction of @beam in first tuple, direction of new LightBeam in second
    /// tuple (if applicable)
    fn deflect(&mut self, beam: &LightBeam) -> ((i32, i32), Option<(i32, i32)>) {
        self.energized = true;

        if self.obstacle.is_none() {
            self.direction = beam.direction;
            return (beam.direction, None);
        }

        match self.obstacle.unwrap() {
            '/' => ((-beam.direction.1, -beam.direction.0), None),
            '\\' => ((beam.direction.1, beam.direction.0), None),
            '|' => match beam.direction {
                (0, 1) => ((-1, 0), Some((1, 0))),
                (0, -1) => ((1, 0), Some((-1, 0))),
                _ => (beam.direction, None),
            },
            '-' => match beam.direction {
                (1, 0) => ((0, -1), Some((0, 1))),
                (-1, 0) => ((0, 1), Some((0, -1))),
                _ => (beam.direction, None),
            },
            _ => ((0, 0), None),
        }
    }
}

struct LightBeam {
    /// direction is normalized velocity as (row, column) (or (y, x))
    direction: (i32, i32),
    /// (row, column)
    pos: (usize, usize),
    /// whether this light beam should be destroyed
    cleanup: bool,
}

impl LightBeam {
    fn new(pos: (usize, usize), direction: (i32, i32)) -> Self {
        LightBeam {
            pos,
            direction,
            cleanup: false,
        }
    }

    fn update_position(&mut self, tiles: &Vec<Vec<Tile>>) {
        match self.direction {
            (-1, 0) => {
                if self.pos.0 == 0 {
                    self.cleanup = true
                } else {
                    self.pos.0 -= 1;
                }
            }
            (0, -1) => {
                if self.pos.1 == 0 {
                    self.cleanup = true
                } else {
                    self.pos.1 -= 1;
                }
            }
            (1, 0) => {
                if self.pos.0 == tiles.len() - 1 {
                    self.cleanup = true
                } else {
                    self.pos.0 += 1;
                }
            }
            (0, 1) => {
                if self.pos.1 == tiles[0].len() - 1 {
                    self.cleanup = true
                } else {
                    self.pos.1 += 1;
                }
            }
            _ => panic!("Invalid direction for a LightBeam: {:?}", self.direction),
        };
    }

    fn update(&mut self, tiles: &mut Vec<Vec<Tile>>) -> Option<LightBeam> {
        if self.cleanup {
            return None;
        }

        let tile = &mut tiles[self.pos.0][self.pos.1];
        if tile.direction == self.direction && tile.energized {
            self.cleanup = true;
            return None;
        }

        let (new_dir, new_beam) = tile.deflect(self);
        self.direction = new_dir;

        self.update_position(tiles);

        if new_beam.is_some() {
            let new_beam = new_beam.unwrap();
            let r_dir = if new_beam.0 < 0 {
                self.pos
                    .0
                    .saturating_sub(new_beam.0.unsigned_abs() as usize)
            } else {
                self.pos.0 + new_beam.0 as usize
            };
            let c_dir = if new_beam.1 < 0 {
                self.pos
                    .1
                    .saturating_sub(new_beam.1.unsigned_abs() as usize)
            } else {
                self.pos.1 + new_beam.1 as usize
            };

            Some(LightBeam::new((r_dir, c_dir), new_beam))
        } else {
            None
        }
    }
}

// for debug
#[allow(unused)]
fn print_tiles(tiles: &Vec<Vec<Tile>>) {
    for r in tiles {
        for t in r {
            match t.obstacle {
                Some(ch) => print!("{}", ch),
                None => {
                    if t.energized {
                        print!(
                            "{}",
                            match t.direction {
                                (-1, 0) => '^',
                                (1, 0) => 'V',
                                (0, -1) => '<',
                                (0, 1) => '>',
                                _ => '.',
                            }
                        );
                    } else {
                        print!(".");
                    }
                }
            }
        }

        println!();
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

fn calculate_beam(beam: LightBeam, mut tiles: Vec<Vec<Tile>>) -> u64 {
    let mut beams = Vec::new();
    beams.push(beam);

    while beams.iter().any(|b| !b.cleanup) {
        let mut new_beams = Vec::new();

        for b in beams.iter_mut() {
            let new_beam = b.update(&mut tiles);

            if let Some(nb) = new_beam {
                new_beams.push(nb);
            }
        }

        beams.append(&mut new_beams);
    }

    tiles.iter().fold(0, |acc, v| {
        acc + v
            .iter()
            .map(|t| if t.energized { 1 } else { 0 })
            .sum::<u64>()
    })
}

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut tiles = Vec::new();
    let mut handles = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        tiles.push(Vec::new());

        for (_, c) in line.char_indices() {
            tiles[i].push(Tile::new(c));
        }
    }

    for i in 0..tiles.len() {
        let t1 = tiles.clone();
        let t2 = tiles.clone();

        let h1 = thread::spawn(move || {
            let beam = LightBeam::new((i, 0), (0, 1));
            calculate_beam(beam, t1)
        });

        handles.push(h1);

        let h2 = thread::spawn(move || {
            let beam = LightBeam::new((i, t2[0].len() - 1), (0, -1));
            calculate_beam(beam, t2)
        });

        handles.push(h2);
    }

    for j in 0..tiles[0].len() {
        let t1 = tiles.clone();
        let t2 = tiles.clone();

        let h1 = thread::spawn(move || {
            let beam = LightBeam::new((0, j), (1, 0));
            calculate_beam(beam, t1)
        });

        handles.push(h1);

        let h2 = thread::spawn(move || {
            let beam = LightBeam::new((t2.len() - 1, j), (-1, 0));
            calculate_beam(beam, t2)
        });

        handles.push(h2);
    }

    let mut sols = Vec::new();
    for h in handles {
        sols.push(h.join().unwrap());
    }

    *sols.iter().max().unwrap()
}
