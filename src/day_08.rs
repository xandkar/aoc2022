use std::io::BufRead; // lines()

use anyhow::{anyhow, Result};

pub struct Data {
    grid: Vec<Vec<u8>>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut grid = Vec::new();
        for (ln, line_result) in
            std::io::BufReader::new(file).lines().enumerate()
        {
            let line = line_result?;
            let mut row = Vec::new();
            for height in line.chars() {
                if !matches!(height, '0'..='9') {
                    return Err(anyhow!(
                        "invalid height {height:?} in line {ln}: {line:?}"
                    ));
                }
                let height: u8 = height.to_string().parse()?;
                row.push(height);
            }
            grid.push(row);
        }
        Ok(Self { grid })
    }

    pub fn solve1(&self) -> Result<u64> {
        let g = &self.grid;
        let nrows = g.len();
        let ncols = g[0].len();
        let rn = nrows - 1;
        let kn = ncols - 1;
        let mut v = vec![vec![true; ncols]; nrows];

        for r0 in 1..rn {
            for k0 in 1..kn {
                let mut up = true;
                let mut down = true;
                let mut left = true;
                let mut right = true;
                for (ri, ki) in (0..r0).rev().map(|ri| (ri, k0)) {
                    if g[ri][ki] >= g[r0][k0] {
                        up = false;
                    }
                }
                for (ri, ki) in (r0 + 1..nrows).map(|ri| (ri, k0)) {
                    if g[ri][ki] >= g[r0][k0] {
                        down = false;
                    }
                }
                for (ri, ki) in (0..k0).rev().map(|ki| (r0, ki)) {
                    if g[ri][ki] >= g[r0][k0] {
                        left = false;
                    }
                }
                for (ri, ki) in (k0 + 1..ncols).map(|ki| (r0, ki)) {
                    if g[ri][ki] >= g[r0][k0] {
                        right = false;
                    }
                }
                v[r0][k0] = up || down || left || right;
            }
        }

        let visible: u64 = v
            .iter()
            .map(|row| row.iter().filter(|is_vis| **is_vis).count() as u64)
            .sum();
        Ok(visible)
    }

    pub fn solve2(&self) -> Result<u64> {
        let g = &self.grid;
        let nrows = g.len();
        let ncols = g[0].len();
        let rn = nrows - 1;
        let kn = ncols - 1;
        let mut score: Vec<Vec<u64>> = vec![vec![0; ncols]; nrows];

        for r0 in 1..rn {
            for k0 in 1..kn {
                let mut up = 0;
                let mut down = 0;
                let mut left = 0;
                let mut right = 0;
                (0..r0).rev().map(|ri| (ri, k0)).fold(
                    false,
                    |blocked, (ri, ki)| {
                        if !blocked {
                            up += 1;
                        };
                        g[ri][ki] >= g[r0][k0] || blocked
                    },
                );
                (r0 + 1..nrows).map(|ri| (ri, k0)).fold(
                    false,
                    |blocked, (ri, ki)| {
                        if !blocked {
                            down += 1;
                        };
                        g[ri][ki] >= g[r0][k0] || blocked
                    },
                );
                (0..k0).rev().map(|ki| (r0, ki)).fold(
                    false,
                    |blocked, (ri, ki)| {
                        if !blocked {
                            left += 1;
                        };
                        g[ri][ki] >= g[r0][k0] || blocked
                    },
                );
                (k0 + 1..ncols).map(|ki| (r0, ki)).fold(
                    false,
                    |blocked, (ri, ki)| {
                        if !blocked {
                            right += 1;
                        };
                        g[ri][ki] >= g[r0][k0] || blocked
                    },
                );
                score[r0][k0] = up * down * left * right;
            }
        }

        let highest: &u64 = score
            .iter()
            .map(|row| row.iter().max().unwrap_or_else(|| unreachable!()))
            .max()
            .unwrap_or_else(|| unreachable!());
        Ok(*highest)
    }
}
