use std::io::BufRead;

use anyhow::{anyhow, Result};

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mult,
}

#[derive(Clone, Copy)]
enum Val {
    Old,
    Num(u64),
}

type Exp = (Op, Val);

fn eval(exp: Exp, old: u64) -> u64 {
    match exp {
        (Op::Add, Val::Num(n)) => old + n,
        (Op::Add, Val::Old) => old + old,
        (Op::Mult, Val::Num(n)) => old * n,
        (Op::Mult, Val::Old) => old * old,
    }
}

#[derive(Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>, // Worry levels.
    operation: Exp, // How a worry level changes as this monkey inspects an item.
    test: u64,
    dst_if_true: usize,
    dst_if_false: usize,
}

impl Monkey {
    fn new(tmp: &MonkeyTmp) -> Result<Self> {
        Ok(Self {
            id: tmp.id.ok_or_else(|| anyhow!("monkey missing id"))?,
            items: (tmp
                .items
                .as_ref()
                .ok_or_else(|| anyhow!("monkey missing items"))?)
            .to_vec(),
            operation: tmp
                .operation
                .ok_or_else(|| anyhow!("monkey missing operation"))?,
            test: tmp.test.ok_or_else(|| anyhow!("monkey missing test"))?,
            dst_if_true: tmp
                .dst_if_true
                .ok_or_else(|| anyhow!("monkey missing dst_if_true"))?,
            dst_if_false: tmp
                .dst_if_false
                .ok_or_else(|| anyhow!("monkey missing dst_if_false"))?,
        })
    }
}

pub struct Data {
    monkeys: Vec<Monkey>,
}

#[derive(Clone)]
struct MonkeyTmp {
    id: Option<usize>,
    items: Option<Vec<u64>>,
    operation: Option<Exp>,
    test: Option<u64>,
    dst_if_true: Option<usize>,
    dst_if_false: Option<usize>,
}

impl MonkeyTmp {
    fn new() -> Self {
        Self {
            id: None,
            items: None,
            operation: None,
            test: None,
            dst_if_true: None,
            dst_if_false: None,
        }
    }
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut monkeys = Vec::new();
        let mut tmp = MonkeyTmp::new();
        for (ln, line_result) in
            std::io::BufReader::new(file).lines().enumerate()
        {
            let line = line_result?;
            let fields = line.split_whitespace().collect::<Vec<&str>>();
            match &fields[..] {
                ["Monkey", id] => {
                    let id = id.strip_suffix(':').ok_or_else(|| {
                        anyhow!("invalid monkey id: {:?} on line {}", id, ln)
                    })?;
                    let id: usize = id.parse()?;
                    tmp = MonkeyTmp {
                        id: Some(id),
                        ..tmp
                    };
                }
                ["Starting", "items:", items_strs @ ..] => {
                    let mut items = Vec::new();
                    for item in items_strs {
                        let item = item.strip_suffix(',').unwrap_or(item);
                        let item = item.parse::<u64>()?;
                        items.push(item);
                    }
                    tmp = MonkeyTmp {
                        items: Some(items),
                        ..tmp
                    };
                }
                ["Operation:", "new", "=", "old", op, val] => {
                    // FIXME: Handle non-numeric: "new = old * old"
                    let val = match *val {
                        "old" => Val::Old,
                        val => Val::Num(val.parse()?),
                    };
                    let op = match *op {
                        "+" => Op::Add,
                        "*" => Op::Mult,
                        _ => {
                            return Err(anyhow!(
                                "unsupported operator: {:?}",
                                op
                            ))
                        }
                    };
                    tmp = MonkeyTmp {
                        operation: Some((op, val)),
                        ..tmp
                    };
                }
                ["Test:", "divisible", "by", num] => {
                    let num: u64 = num.parse()?;
                    tmp = MonkeyTmp {
                        test: Some(num),
                        ..tmp
                    };
                }
                ["If", "true:", "throw", "to", "monkey", id] => {
                    let id: usize = id.parse()?;
                    tmp = MonkeyTmp {
                        dst_if_true: Some(id),
                        ..tmp
                    };
                }
                ["If", "false:", "throw", "to", "monkey", id] => {
                    let id: usize = id.parse()?;
                    tmp = MonkeyTmp {
                        dst_if_false: Some(id),
                        ..tmp
                    };
                }
                [] => {
                    // TODO How to have a re-usable closure that does this construct & push?
                    monkeys.push(Monkey::new(&tmp)?);
                    tmp = MonkeyTmp::new();
                }
                _ => return Err(anyhow!("invalid line {}: {:?}", ln, line)),
            }
        }
        // TODO How to have a re-usable closure that does this construct & push?
        monkeys.push(Monkey::new(&tmp)?);
        Ok(Self { monkeys })
    }

    pub fn solve1(&self) -> Result<u64> {
        let rounds = 20;
        let reduce = &mut |w| w / 3;
        Ok(monkey_biz_level(&self.monkeys, rounds, reduce))
    }

    pub fn solve2(&self) -> Result<u64> {
        let modulus: u64 = self
            .monkeys
            .iter()
            .map(|Monkey { test, .. }| test)
            .product();
        let rounds = 10_000;
        let reduce = &mut |w| w % modulus;
        Ok(monkey_biz_level(&self.monkeys, rounds, reduce))
    }
}

fn monkey_biz_level(
    monkeys: &[Monkey],
    rounds: usize,

    // Can we not somehow have all these muts for such a simple closure?
    // Which doesn't _actually_ mutate anything.
    reduce: &mut dyn FnMut(u64) -> u64,
) -> u64 {
    let mut monkeys = monkeys.to_vec();
    let mut count = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            assert_eq!(m, monkeys[m].id);
            let items: Vec<u64> = monkeys[m].items.clone();
            monkeys[m].items = Vec::new();
            for w0 in items {
                count[m] += 1;
                let w1 = eval(monkeys[m].operation, w0);
                let w2 = reduce(w1);
                let dst = if w2 % monkeys[m].test == 0 {
                    monkeys[m].dst_if_true
                } else {
                    monkeys[m].dst_if_false
                };
                monkeys[dst].items.push(w2);
            }
        }
    }
    count.sort();
    count.reverse();
    count[..2].iter().product()
}
