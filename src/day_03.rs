use std::collections::HashSet;
use std::io::BufRead; // lines()

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
struct Sack {
    compartment_1: HashSet<char>,
    compartment_2: HashSet<char>,
}

impl Sack {
    fn contents(&self) -> HashSet<char> {
        self.compartment_1
            .union(&self.compartment_2)
            .copied()
            .collect::<HashSet<char>>()
    }

    fn intersection(&self, s2: &Self) -> HashSet<char> {
        let c1 = self.contents();
        let c2 = s2.contents();
        c1.intersection(&c2).copied().collect::<HashSet<char>>()
    }
}

pub struct Data {
    sacks: Vec<Sack>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut sacks = Vec::new();
        for line_result in std::io::BufReader::new(file).lines() {
            let line = line_result?;
            let len = line.len();
            if len < 2 || len % 2 != 0 {
                return Err(anyhow!(
                    "invalid number of items ({len}) in {line:?}"
                ));
            }
            let mid = len / 2;
            let c1: HashSet<char> =
                line[0..mid].to_string().chars().collect();
            let c2: HashSet<char> = line[mid..].to_string().chars().collect();
            sacks.push(Sack {
                compartment_1: c1,
                compartment_2: c2,
            });
        }
        Ok(Self { sacks })
    }

    pub fn part1(&self) -> Result<u64> {
        let mut total = 0;
        for Sack {
            compartment_1: c1,
            compartment_2: c2,
        } in self.sacks.iter()
        {
            let intersection: HashSet<&char> = c1.intersection(c2).collect();
            let intersection = if intersection.len() != 1 {
                return Err(anyhow!(
                    "invalid number of intersections: {}",
                    intersection.len()
                ));
            } else {
                **intersection.iter().next().unwrap()
            };
            // unwrap OK because we checked len.
            let p = priority(intersection);
            total += p;
        }
        Ok(total as u64)
    }

    pub fn part2(&self) -> Result<u64> {
        let mut groups = Vec::new();
        let mut group_buf = Vec::new();
        let mut i = 1;
        for sack in self.sacks.iter() {
            group_buf.push(sack);
            if i % 3 == 0 {
                groups.push(group_buf.clone());
                group_buf.clear();
            };
            i += 1;
        }
        let mut total = 0;
        for group in groups {
            let (s1, s2, s3) = match &group[..] {
                [s1, s2, s3] => (s1, s2, s3),
                _ => unreachable!(),
            };
            let intersection: HashSet<char> = s1
                .intersection(s2)
                .intersection(&s3.contents())
                .copied()
                .collect();
            if intersection.len() != 1 {
                return Err(anyhow!(
                    "invalid number of intersections: {}",
                    intersection.len()
                ));
            }
            let intersection = intersection.iter().next().unwrap();
            let p = priority(*intersection);
            total += p;
        }
        Ok(total as u64)
    }
}

fn priority(c: char) -> u32 {
    let init_lower = 'a' as u32;
    let init_upper = 'A' as u32;
    let (init_old, init_new) = match c {
        'a'..='z' => (init_lower, 1),
        'A'..='Z' => (init_upper, 27),
        _ => panic!("unexpected character: {c:?}"),
    };
    let n = c as u32;
    init_new + (n - init_old)
}
