use std::io::BufRead; // lines()

use anyhow::Result;

pub struct Data {
    rev_sorted_totals: Vec<u64>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut elves: Vec<Vec<u64>> = Vec::new();
        let mut elf: Vec<u64> = Vec::new();
        for line_result in std::io::BufReader::new(file).lines() {
            let line = line_result?;
            if line.is_empty() {
                elves.push(elf.clone());
                elf.clear();
            } else {
                let calories: u64 = line.parse()?;
                elf.push(calories);
            }
        }
        elves.push(elf.clone());
        let mut totals: Vec<u64> =
            elves.iter().map(|elf| elf.iter().sum()).collect();
        totals.sort();
        totals.reverse();
        Ok(Data {
            rev_sorted_totals: totals,
        })
    }

    pub fn part1(&self) -> Result<u64> {
        match self.rev_sorted_totals[..] {
            [] => Err(anyhow::anyhow!("zero elves in data")),
            [highest, ..] => Ok(highest),
        }
    }

    pub fn part2(&self) -> Result<u64> {
        match self.rev_sorted_totals[..] {
            [t1, t2, t3, ..] => Ok(t1 + t2 + t3),
            _ => Err(anyhow::anyhow!("less than 3 elves in data")),
        }
    }
}
