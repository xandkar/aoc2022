use std::collections::HashMap;
use std::io::BufRead; // lines()

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
enum Ix {
    Mv {
        quant: usize,
        src: usize,
        dst: usize,
    },
}

#[derive(Debug, Clone)]
pub struct Data {
    stacks: Vec<Vec<char>>,
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
        let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
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
                                                Some(stack) => stack.to_vec(),
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
                            let src: usize = src.parse()?;
                            let dst: usize = dst.parse()?;
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
        let mut stacks_vec = vec![Vec::new(); stacks.len() + 1];
        for (i, stack) in stacks.iter_mut() {
            stack.reverse();
            stacks_vec[*i] = stack.to_vec();
        }
        Ok(Self {
            stacks: stacks_vec,
            instructions,
        })
    }

    pub fn solve1(&self) -> Result<String> {
        let mut stacks = self.stacks.clone();
        for Ix::Mv { quant, src, dst } in self.instructions.iter() {
            for _ in 0..*quant {
                let krate =
                    stacks[*src].pop().unwrap_or_else(|| unreachable!());
                stacks[*dst].push(krate);
            }
        }
        Ok(Self::msg(stacks))
    }

    pub fn solve2(&self) -> Result<String> {
        let mut stacks = self.stacks.clone();
        for Ix::Mv { quant, src, dst } in self.instructions.iter() {
            let len = stacks[*src].len();
            let mut krates = stacks[*src].split_off(len - quant);
            stacks[*dst].append(&mut krates);
        }
        Ok(Self::msg(stacks))
    }

    fn msg(stacks: Vec<Vec<char>>) -> String {
        stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect::<String>()
            .trim()
            .to_string()
    }
}
