use std::collections::HashSet;
use std::io::BufRead;

use anyhow::{anyhow, Result};

#[derive(Clone, Copy)]
enum Dir {
    R,
    L,
    D,
    U,
}

impl std::str::FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = match s {
            "R" => Dir::R,
            "L" => Dir::L,
            "D" => Dir::D,
            "U" => Dir::U,
            _ => return Err(anyhow!("invalid direction: {:?}", s)),
        };
        Ok(d)
    }
}

type Mv = (Dir, u8);

pub struct Data {
    moves: Vec<Mv>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut moves = Vec::new();
        for (ln, line_result) in
            std::io::BufReader::new(file).lines().enumerate()
        {
            let line = line_result?;
            match line.split_whitespace().collect::<Vec<&str>>()[..] {
                [d, v] => moves.push((d.parse::<Dir>()?, v.parse::<u8>()?)),
                _ => {
                    return Err(anyhow!(
                        "invalid move on line {}: {:?}",
                        ln,
                        line
                    ))
                }
            }
        }
        Ok(Self { moves })
    }

    pub fn solve1(&self) -> Result<usize> {
        Ok(solve(&self.moves, 2))
    }

    pub fn solve2(&self) -> Result<usize> {
        Ok(solve(&self.moves, 10))
    }
}

fn solve(moves: &[Mv], n: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knot_pos = vec![(4, 0); n];
    let head = 0;
    let tail = n - 1;
    visited.insert(knot_pos[tail]);
    for d in deltas(moves) {
        // head
        knot_pos[head] = add(knot_pos[head], d);

        // tail
        for current in 1..n {
            let ahead = current - 1;
            knot_pos[current] = catchup(knot_pos[current], knot_pos[ahead]);
            visited.insert(knot_pos[tail]);
        }
    }
    visited.len()
}

type Pos = (i32, i32);

fn catchup(current: Pos, ahead: Pos) -> Pos {
    match distance(ahead, current) {
        0 | 1 => current,
        _ => add(current, sig(sub(ahead, current))),
    }
}

fn add((r1, k1): Pos, (r2, k2): Pos) -> Pos {
    ((r1 + r2), (k1 + k2))
}

fn sub((r1, k1): Pos, (r2, k2): Pos) -> Pos {
    ((r1 - r2), (k1 - k2))
}

fn sig((r, k): Pos) -> Pos {
    (r.signum(), k.signum())
}

fn distance(p1: Pos, p2: Pos) -> u32 {
    let (r1, k1) = p1;
    let (r2, k2) = p2;
    let rd = r1.abs_diff(r2);
    let kd = k1.abs_diff(k2);
    std::cmp::max(rd, kd)
}

fn deltas(moves: &[Mv]) -> impl Iterator<Item = Pos> + '_ {
    moves.iter().flat_map(|(dir, delta): &Mv| {
        (0..(*delta)).map(move |_| match dir {
            Dir::R => (0, 1),
            Dir::L => (0, -1),
            Dir::D => (1, 0),
            Dir::U => (-1, 0),
        })
    })
}
