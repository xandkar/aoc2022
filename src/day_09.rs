use std::cmp::max;
use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Clone, Copy)]
enum Dir {
    R,
    L,
    D,
    U,
}

impl FromStr for Dir {
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
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut head: (i32, i32) = (4, 0);
        let mut tail: (i32, i32) = head;
        visited.insert(tail);
        self.moves.iter().for_each(|m| {
            mv(m).for_each(|(rd, kd)| {
                head = (head.0 + rd, head.1 + kd);
                if distance(head, tail) > 1 {
                    tail = behind(head, m.0);
                }
                visited.insert(tail);
            });
        });
        Ok(visited.len())
    }

    pub fn solve2(&self) -> Result<usize> {
        todo!();
    }
}

type Point = (i32, i32);

fn distance(p1: Point, p2: Point) -> u32 {
    let (r1, k1) = p1;
    let (r2, k2) = p2;
    let rd = r1.abs_diff(r2);
    let kd = k1.abs_diff(k2);
    max(rd, kd)
}

fn behind((r, k): Point, facing: Dir) -> Point {
    match facing {
        Dir::R => (r, k - 1),
        Dir::L => (r, k + 1),
        Dir::D => (r - 1, k),
        Dir::U => (r + 1, k),
    }
}

fn mv((dir, delta): &Mv) -> impl Iterator<Item = Point> + '_ {
    (0..(*delta)).map(move |_| match dir {
        Dir::R => (0, 1),
        Dir::L => (0, -1),
        Dir::D => (1, 0),
        Dir::U => (-1, 0),
    })
}
