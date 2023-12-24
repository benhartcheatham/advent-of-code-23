use clap::{arg, command, ArgAction};
use std::collections::HashMap;
use std::io;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    pos: [i64; 3],
    ranges: [Option<RangeInclusive<i64>>; 3],
    block_height: i64,
    supporting: Option<Vec<usize>>,
    supported_by: Option<Vec<usize>>,
}

impl Block {
    fn new(id: usize, pos: &[i64], len: &[i64]) -> Self {
        assert!(pos.len() == 3);
        assert!(len.len() == 3);

        let mut ranges = [None, None, None];
        for (i, dim) in len.iter().enumerate() {
            if *dim != 0 {
                ranges[i] = Some(0..=*dim);
            }
        }

        let mut ps = [0, 0, 0];
        ps[..].copy_from_slice(pos);

        let block_height = if let Some(zr) = &ranges[2] {
            zr.end() + 1
        } else {
            1
        };

        Block {
            id,
            pos: ps,
            ranges,
            block_height,
            supporting: None,
            supported_by: None,
        }
    }

    fn construct_xy_points(&self) -> Vec<[i64; 3]> {
        let mut points = Vec::new();

        if let Some(xr) = &self.ranges[0] {
            for i in xr.clone().skip(1) {
                points.push([self.pos[0] + i, self.pos[1], self.pos[2]]);
            }
        }

        if let Some(yr) = &self.ranges[1] {
            for i in yr.clone().skip(1) {
                points.push([self.pos[0], self.pos[1] + i, self.pos[2]]);
            }
        }

        points.push(self.pos);
        points
    }

    /// Only compares the x and y dimensions
    fn overlaps(&self, other: &Block) -> bool {
        let sp = self.construct_xy_points();
        let op = other.construct_xy_points();

        for p0 in sp.iter() {
            for p1 in op.iter() {
                if p0[0] == p1[0] && p0[1] == p1[1] {
                    return true;
                }
            }
        }

        false
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.ranges == other.ranges
    }
}

impl Eq for Block {}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos[2].cmp(&other.pos[2])
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

/*
 * Algorithm as follows:
 *  1. take list of blocks (sorted by z in ascending order) and get towers(pos) for each block
 *  2. if towers(pos), set block z to towers(pos) and insert towers(pos) + z into towers(pos),
 *     else, set block z to 1 and insert z into towers(pos)
 *  3. store each block that is in z level into z_blocks
 *  4. update block.supporting based on blocks in z level just under block
 */
fn update_z(blocks: &mut [Block]) {
    // holds (x,y) => height
    let mut towers: HashMap<(i64, i64), i64> = HashMap::new();
    let mut z_blocks: HashMap<i64, Vec<usize>> = HashMap::new();

    for b in blocks.iter_mut() {
        let mut max_z = 1;
        for p in b.construct_xy_points() {
            if let Some(h) = towers.get(&(p[0], p[1])) {
                max_z = i64::max(max_z, *h);
            }
        }

        b.pos[2] = max_z;

        for p in b.construct_xy_points() {
            towers.insert((p[0], p[1]), b.pos[2] + b.block_height);
        }
    }

    for b in blocks.iter() {
        z_blocks
            .entry(b.pos[2])
            .and_modify(|v| v.push(b.id))
            .or_insert(vec![b.id]);
    }

    let mut supporting = Vec::new();
    for b in blocks.iter() {
        if let Some(bs) = z_blocks.get(&(b.pos[2] + b.block_height)) {
            for id in bs {
                let b2 = blocks.iter().find(|b2| b2.id == *id).unwrap();

                if b.overlaps(b2) {
                    supporting.push((b.id, b2.id));
                }
            }
        }
    }

    for (b_id, b2_id) in supporting {
        let block = blocks.iter_mut().find(|fb| fb.id == b_id).unwrap();
        match &mut block.supporting {
            Some(ref mut sb) => sb.push(b2_id),
            None => block.supporting = Some(vec![b2_id]),
        }

        let block = blocks.iter_mut().find(|fb| fb.id == b2_id).unwrap();
        match &mut block.supported_by {
            Some(ref mut sb) => sb.push(b_id),
            None => block.supported_by = Some(vec![b_id]),
        }
    }
}

fn find_num_deleteable(blocks: &Vec<Block>) -> u64 {
    let mut sum = 0;

    for b in blocks {
        if let Some(s) = &b.supporting {
            if s.iter().all(|supported| {
                let block = blocks.iter().find(|b| b.id == *supported).unwrap();
                if let Some(v) = &block.supported_by {
                    v.len() > 1
                } else {
                    false
                }
            }) {
                sum += 1;
            }
        } else {
            sum += 1;
        }
    }

    sum
}

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut blocks = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let (p0, p1) = line.split_once('~').unwrap();

        let mut p0: Vec<i64> = p0.split(',').map(|c| c.parse::<i64>().unwrap()).collect();
        let mut p1: Vec<i64> = p1.split(',').map(|c| c.parse::<i64>().unwrap()).collect();

        if p0
            .iter()
            .zip(p1.iter())
            .map(|(d0, d1)| d1 - d0)
            .any(|d| d < 0)
        {
            let temp = p0.clone();
            p0 = p1.clone();
            p1 = temp.clone();
        }

        let len: Vec<i64> = p1.iter().enumerate().map(|(i, v)| *v - p0[i]).collect();

        blocks.push(Block::new(i, &p0, &len));
    }

    blocks.sort();

    update_z(&mut blocks);

    blocks.sort();

    find_num_deleteable(&blocks)
}
