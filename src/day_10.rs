use std::io::BufRead;

use anyhow::{anyhow, Result};

enum Ix {
    Noop,
    Addx(i32),
}

impl Ix {
    fn cost(&self) -> u64 {
        match self {
            Ix::Noop => 1,
            Ix::Addx(_) => 2,
        }
    }

    fn val(&self) -> i32 {
        match self {
            Ix::Noop => 0,
            Ix::Addx(v) => *v,
        }
    }
}

pub struct Data {
    program: Vec<Ix>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut program = Vec::new();
        for (_ln, line_result) in
            std::io::BufReader::new(file).lines().enumerate()
        {
            let line = line_result?;
            match line.split_whitespace().collect::<Vec<&str>>()[..] {
                ["noop"] => program.push(Ix::Noop),
                ["addx", v] => {
                    let v: i32 = v.parse()?;
                    program.push(Ix::Addx(v));
                }
                _ => return Err(anyhow!("invalid line: {:?}", line)),
            }
        }
        Ok(Self { program })
    }

    pub fn solve1(&self) -> Result<i32> {
        let mut x = 1;
        let mut strengths = 0;
        for (cycle, delta) in execute(&self.program) {
            // during cycle
            if let 20 | 60 | 100 | 140 | 180 | 220 = cycle {
                strengths += cycle * x
            }
            // after cycle
            x += delta;
        }
        Ok(strengths)
    }

    pub fn solve2(&self) -> Result<String> {
        let mut buf = String::new();
        let mut sprite_pos: i32 = 1;
        for (pixel_pos, sprite_pos_delta) in execute(&self.program) {
            // during cycle
            if ((sprite_pos - 1)..(sprite_pos + 2))
                .contains(&((pixel_pos - 1) % 40))
            {
                buf.push('#');
            } else {
                buf.push('.');
            }
            if pixel_pos % 40 == 0 {
                buf.push('\n');
            }
            // after cycle
            sprite_pos += sprite_pos_delta;
        }
        eprint!("{}", buf); // For visual reading.
        Ok(buf)
    }
}

fn execute(program: &[Ix]) -> impl Iterator<Item = (i32, i32)> + '_ {
    program
        .iter()
        .flat_map(|ix| {
            let cost = ix.cost();
            (1..cost + 1)
                .map(move |paid| if paid == cost { ix.val() } else { 0 })
        })
        .enumerate()
        .map(|(cycle, delta)| ((cycle + 1) as i32, delta))
}
