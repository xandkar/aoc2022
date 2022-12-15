use std::io::BufRead;

use anyhow::{anyhow, Result};

pub struct Data {}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input).map_err(|e| {
            anyhow!("Failure to open input file {:?}: {:?}", input, e)
        })?;
        for (ln, line_result) in std::io::BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
        {
            let line = line_result?;
            eprintln!("{}: {:?}", ln, line);
        }
        Ok(Self {})
    }

    pub fn solve1(&self) -> Result<u64> {
        todo!();
    }

    pub fn solve2(&self) -> Result<u64> {
        todo!();
    }
}
