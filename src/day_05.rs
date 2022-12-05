use std::collections::HashMap;
use std::io::BufRead; // lines()

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
enum Ix {
    Mv { quant: usize, src: u64, dst: u64 },
}

#[derive(Debug, Clone)]
pub struct Data {
    stacks: HashMap<u64, Vec<char>>,
    instructions: Vec<Ix>,
}

#[derive(Debug)]
enum Section {
    Cts,
    Ixs,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut section = Section::Cts;
        let mut stacks: HashMap<u64, Vec<char>> = HashMap::new();
        let mut instructions: Vec<Ix> = Vec::new();
        for line_result in std::io::BufReader::new(file).lines() {
            let line = line_result?;
            match (&section, line.as_str()) {
                (Section::Cts, "") => section = Section::Ixs,
                (Section::Cts, _) => {
                    let mut stack_id = 0;
                    let mut pos = 0;
                    loop {
                        match line.get(pos..pos + 3) {
                            None => break,
                            Some(ct) => {
                                stack_id += 1;
                                match ct.chars().collect::<Vec<char>>()[..] {
                                    ['[', crate_name @ 'A'..='Z', ']'] => {
                                        let mut stack =
                                            match stacks.get(&stack_id) {
                                                None => Vec::new(),
                                                Some(s) => s.to_vec(),
                                            };
                                        stack.push(crate_name);
                                        stacks.insert(stack_id, stack);
                                    },
                                    [' ', ' ', ' '] => {
                                    },
                                    [' ', '0'..='9', ' '] => {
                                        // XXX We could just ignore this line
                                        //     with explicit stack_id lables
                                        //     because the stacks seem to
                                        //     always be sequential.
                                    },
                                    _ => return Err(anyhow!("invalid crate: {ct:?} in line: {line:?}")),
                                }
                            }
                        }
                        pos += 4;
                    }
                }
                (Section::Ixs, _) => {
                    match line.split_whitespace().collect::<Vec<&str>>()[..] {
                        ["move", quant, "from", src, "to", dst] => {
                            let quant: usize = quant.parse()?;
                            let src: u64 = src.parse()?;
                            let dst: u64 = dst.parse()?;
                            instructions.push(Ix::Mv { quant, src, dst });
                        }
                        _ => {
                            return Err(anyhow!(
                                "invalid instruction line: {line:?}"
                            ))
                        }
                    }
                }
            }
        }
        for (_, stack) in stacks.iter_mut() {
            stack.reverse();
        }
        Ok(Self {
            stacks,
            instructions,
        })
    }

    pub fn part1(&self) -> Result<String> {
        let mut data = self.clone();
        for Ix::Mv { quant, src, dst } in self.instructions.iter() {
            for _ in 0..*quant {
                let src_stack = data
                    .stacks
                    .get_mut(src)
                    .unwrap_or_else(|| unreachable!());
                let krate = src_stack.pop().unwrap_or_else(|| unreachable!());
                let dst_stack = data
                    .stacks
                    .get_mut(dst)
                    .unwrap_or_else(|| unreachable!());
                dst_stack.push(krate);
            }
        }
        let max = *data.stacks.keys().max().unwrap_or_else(|| unreachable!());
        let min = *data.stacks.keys().min().unwrap_or_else(|| unreachable!());
        let mut msg = String::new();
        for i in min..max + 1 {
            let stack = data.stacks.get(&i).unwrap_or_else(|| unreachable!());
            stack.last().iter().for_each(|c| msg.push(**c))
        }
        Ok(msg)
    }

    pub fn part2(&self) -> Result<String> {
        let mut data = self.clone();
        for Ix::Mv { quant, src, dst } in self.instructions.iter() {
            let src_stack =
                data.stacks.get_mut(src).unwrap_or_else(|| unreachable!());
            let mut krates = src_stack.split_off(src_stack.len() - quant);
            let dst_stack =
                data.stacks.get_mut(dst).unwrap_or_else(|| unreachable!());
            dst_stack.append(&mut krates);
        }
        let max = *data.stacks.keys().max().unwrap_or_else(|| unreachable!());
        let min = *data.stacks.keys().min().unwrap_or_else(|| unreachable!());
        let mut msg = String::new();
        for i in min..max + 1 {
            let stack = data.stacks.get(&i).unwrap_or_else(|| unreachable!());
            stack.last().iter().for_each(|c| msg.push(**c))
        }
        Ok(msg)
    }
}
