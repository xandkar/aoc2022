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
                let o = (r0, k0);
                let up = is_visible(g, o, view_up(o));
                let down = is_visible(g, o, view_down(o, nrows));
                let left = is_visible(g, o, view_left(o));
                let right = is_visible(g, o, view_right(o, ncols));
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
        let rn = g.len();
        let kn = g[0].len();
        let mut score: Vec<Vec<u64>> = vec![vec![0; kn]; rn];

        for r0 in 1..rn - 1 {
            for k0 in 1..kn - 1 {
                let o = (r0, k0);
                let u = count_visible(g, o, view_up(o));
                let d = count_visible(g, o, view_down(o, rn));
                let l = count_visible(g, o, view_left(o));
                let r = count_visible(g, o, view_right(o, kn));
                score[r0][k0] = u * d * l * r;
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

type Point = (usize, usize);

fn view_up((r0, k0): Point) -> impl Iterator<Item = Point> {
    (0..r0).rev().map(move |ri| (ri, k0))
}

fn view_down((r0, k0): Point, nrows: usize) -> impl Iterator<Item = Point> {
    (r0 + 1..nrows).map(move |ri| (ri, k0))
}

fn view_left((r0, k0): Point) -> impl Iterator<Item = Point> {
    (0..k0).rev().map(move |ki| (r0, ki))
}

fn view_right((r0, k0): Point, ncols: usize) -> impl Iterator<Item = Point> {
    (k0 + 1..ncols).map(move |ki| (r0, ki))
}

fn count_visible<View: Iterator<Item = Point>>(
    data: &[Vec<u8>],
    origin: Point,
    view: View,
) -> u64 {
    let (r0, k0) = origin;
    let mut counter = 0;
    view.fold(false, |is_already_blocked, (ri, ki)| {
        if !is_already_blocked {
            counter += 1;
        };
        data[ri][ki] >= data[r0][k0] || is_already_blocked
    });
    counter
}

fn is_visible<View: Iterator<Item = Point>>(
    data: &[Vec<u8>],
    (r, k): Point,
    view: View,
) -> bool {
    view.filter(|(ri, ki)| data[*ri][*ki] >= data[r][k]).count() == 0
}
