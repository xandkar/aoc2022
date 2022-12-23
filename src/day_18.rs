use std::collections::HashSet;
use std::io::BufRead;

use anyhow::{anyhow, Result};

type Cube = (i32, i32, i32);

pub struct Data {
    // 1x1x1 cubes on a 3D grid, each given as its x,y,z position:
    cubes: HashSet<Cube>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input).map_err(|e| {
            anyhow!("Failure to open input file {:?}: {:?}", input, e)
        })?;
        let mut cubes = HashSet::new();
        for (ln, line_result) in std::io::BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
        {
            let line = line_result?;
            let fields: Vec<&str> = line.split(',').collect();
            match fields[..] {
                [x, y, z] => {
                    let x: i32 = x.parse()?;
                    let y: i32 = y.parse()?;
                    let z: i32 = z.parse()?;
                    cubes.insert((x, y, z));
                }
                _ => return Err(anyhow!("Invalid line {}: {:?}", ln, line)),
            }
        }
        Ok(Self { cubes })
    }

    pub fn solve1(&self) -> Result<usize> {
        Ok(self.faces().filter(|c| !self.cubes.contains(c)).count())
    }

    pub fn solve2(&self) -> Result<usize> {
        let (lo, hi) = self.bounds();
        let mut facing_out: HashSet<Cube> = HashSet::new();
        let mut stack = Vec::new();
        stack.push(lo);
        while let Some(cube) = stack.pop() {
            for face in cube_faces(&cube) {
                if cube_is_within_bounds(face, lo, hi)
                    && !facing_out.contains(&face)
                    && !self.cubes.contains(&face)
                {
                    facing_out.insert(face);
                    stack.push(face);
                }
            }
        }
        Ok(self.faces().filter(|c| facing_out.contains(c)).count())
    }

    fn faces(&self) -> impl Iterator<Item = Cube> + '_ {
        self.cubes.iter().flat_map(cube_faces)
    }

    fn bounds(&self) -> (Cube, Cube) {
        macro_rules! bound {
            ($pos:tt, $min_or_max:ident) => {
                self.cubes
                    .iter()
                    .map(|c| c.$pos)
                    .$min_or_max()
                    .ok_or_else(|| unreachable!())
                    .unwrap()
            };
        }
        let x_lo = bound!(0, min);
        let x_hi = bound!(0, max);
        let y_lo = bound!(1, min);
        let y_hi = bound!(1, max);
        let z_lo = bound!(2, min);
        let z_hi = bound!(2, max);
        (
            (x_lo - 1, y_lo - 1, z_lo - 1),
            (x_hi + 1, y_hi + 1, z_hi + 1),
        )
    }
}

fn cube_is_within_bounds(
    (x, y, z): Cube,
    (x_lo, y_lo, z_lo): Cube,
    (x_hi, y_hi, z_hi): Cube,
) -> bool {
    let above_lo = x >= x_lo && y >= y_lo && z >= z_lo;
    let below_hi = x <= x_hi && y <= y_hi && z <= z_hi;
    above_lo && below_hi
}

fn cube_faces((x, y, z): &Cube) -> impl Iterator<Item = Cube> + '_ {
    [
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ]
    .iter()
    .map(move |(xo, yo, zo)| (x + xo, y + yo, z + zo))
}
