use std::collections::HashMap;
use std::io::BufRead; // lines()

use anyhow::{anyhow, Result};

#[derive(Clone)]
enum Entry {
    Dir { _name: String },
    File { _name: String, size: u64 },
}

enum Cmd {
    CdRoot,
    CdUp,
    Cd(String),
    Ls(Vec<Entry>),
}

pub struct Data {
    commands: Vec<Cmd>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut commands = Vec::new();
        let mut entries: Option<Vec<Entry>> = None;
        for line_result in std::io::BufReader::new(file).lines() {
            let line = line_result?;
            let fields = line.split_whitespace().collect::<Vec<&str>>();
            if let (Some(e), ["$"]) = (&mut entries, &fields[0..1]) {
                commands.push(Cmd::Ls(e.to_vec()));
                entries = None;
            }
            match (&mut entries, &fields[..]) {
                (_, ["$", "cd", "/"]) => {
                    commands.push(Cmd::CdRoot);
                }
                (_, ["$", "cd", ".."]) => {
                    commands.push(Cmd::CdUp);
                }
                (_, ["$", "cd", dir]) => {
                    commands.push(Cmd::Cd(dir.to_string()));
                }
                (_, ["$", "ls"]) => {
                    entries = Some(Vec::new());
                }
                (Some(ref mut e), ["dir", name]) => {
                    e.push(Entry::Dir {
                        _name: name.to_string(),
                    });
                }
                (Some(ref mut e), [size, name]) => {
                    e.push(Entry::File {
                        _name: name.to_string(),
                        size: size.parse::<u64>()?,
                    });
                }
                _ => return Err(anyhow!("invalid line: {line:?}")),
            }
        }
        if let Some(e) = &mut entries {
            commands.push(Cmd::Ls(e.to_vec()));
        }
        Ok(Self { commands })
    }

    pub fn part1(&self) -> Result<u64> {
        let sum = self
            .dir_sizes()
            .iter()
            .map(|(_, s)| s)
            .filter(|s| **s <= 100000)
            .sum();
        Ok(sum)
    }

    pub fn part2(&self) -> Result<Option<u64>> {
        let total = 70000000;
        let need = 30000000;
        let sizes = self.dir_sizes();
        let used = sizes.get(&vec!["/".to_string()]).unwrap();
        let available = total - used;
        if need > available {
            let missing = need - available;
            let smallest_to_fill_missing = sizes
                .iter()
                .map(|(_, s)| *s)
                .filter(|s| *s >= missing)
                .min();
            Ok(smallest_to_fill_missing)
        } else {
            Ok(None)
        }
    }

    fn dir_sizes(&self) -> HashMap<Vec<String>, u64> {
        let mut path: Vec<String> = Vec::new();
        let mut sizes: HashMap<Vec<String>, u64> = HashMap::new();
        for c in self.commands.iter() {
            match c {
                Cmd::CdRoot => {
                    path.clear();
                    path.push("/".to_string());
                }
                Cmd::CdUp => {
                    path.pop();
                }
                Cmd::Cd(dir) => {
                    path.push(dir.to_string());
                }
                Cmd::Ls(entries) => {
                    for e in entries {
                        match e {
                            Entry::Dir { _name: _ } => (),
                            Entry::File { size, _name: _ } => {
                                for (i, _) in path.iter().enumerate() {
                                    let subpath: Vec<String> = path[0..i + 1]
                                        .iter()
                                        .map(|p| p.to_string())
                                        .collect();
                                    sizes
                                        .entry(subpath)
                                        .and_modify(|s| *s += size)
                                        .or_insert(*size);
                                }
                            }
                        }
                    }
                }
            }
        }
        sizes
    }
}
