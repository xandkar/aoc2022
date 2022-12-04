use std::io::BufRead; // lines()

use anyhow::{anyhow, Result};

pub struct Data {
    range_pairs: Vec<((u64, u64), (u64, u64))>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut range_pairs = Vec::new();
        for line_result in std::io::BufReader::new(file).lines() {
            let line = line_result?;
            let mut rs = line.split(',');
            let r1 = rs.next().ok_or_else(|| {
                anyhow!("missing 1st range in line: {line:?}")
            })?;
            let r2 = rs.next().ok_or_else(|| {
                anyhow!("missing 2nd range in line: {line:?}")
            })?;
            let mut r1_bounds = r1.split('-');
            let mut r2_bounds = r2.split('-');
            let r1_lo = r1_bounds.next().ok_or_else(|| {
                anyhow!("missing lower bound in 1st range of line: {line:?}")
            })?;
            let r1_hi = r1_bounds.next().ok_or_else(|| {
                anyhow!("missing upper bound in 1st range of line: {line:?}")
            })?;
            let r2_lo = r2_bounds.next().ok_or_else(|| {
                anyhow!("missing lower bound in 2nd range of line: {line:?}")
            })?;
            let r2_hi = r2_bounds.next().ok_or_else(|| {
                anyhow!("missing upper bound in 2nd range of line: {line:?}")
            })?;
            let r1_lo: u64 = r1_lo.parse()?;
            let r1_hi: u64 = r1_hi.parse()?;
            let r2_lo: u64 = r2_lo.parse()?;
            let r2_hi: u64 = r2_hi.parse()?;
            if r1_lo > r1_hi {
                return Err(anyhow!(
                    "invalid range {:?} in line: {line:?}",
                    (r1_lo, r1_hi)
                ));
            }
            if r2_lo > r2_hi {
                return Err(anyhow!(
                    "invalid range {:?} in line: {line:?}",
                    (r2_lo, r2_hi)
                ));
            }
            let p1 = (r1_lo, r1_hi);
            let p2 = (r2_lo, r2_hi);
            let ps = (p1, p2);
            range_pairs.push(ps);
        }
        Ok(Self { range_pairs })
    }

    pub fn part1(&self) -> Result<usize> {
        let count = self
            .range_pairs
            .iter()
            .filter(|(r1, r2)| {
                is_2nd_range_contained_in_1st(r1, r2)
                    || is_2nd_range_contained_in_1st(r2, r1)
            })
            .count();
        Ok(count)
    }

    pub fn part2(&self) -> Result<usize> {
        let count = self
            .range_pairs
            .iter()
            .filter(|(r1, r2)| is_ranges_overlap(r1, r2))
            .count();
        Ok(count)
    }
}

fn is_2nd_range_contained_in_1st(
    (r1_lo, r1_hi): &(u64, u64),
    (r2_lo, r2_hi): &(u64, u64),
) -> bool {
    r2_lo >= r1_lo && r2_hi <= r1_hi
}

fn is_ranges_overlap(
    (r1_lo, r1_hi): &(u64, u64),
    (r2_lo, r2_hi): &(u64, u64),
) -> bool {
    r1_hi >= r2_lo && r1_lo <= r2_hi
}

#[test]
fn range_containment() {
    assert!(is_2nd_range_contained_in_1st(&(1, 5), &(2, 4)));
    assert!(!is_2nd_range_contained_in_1st(&(1, 5), &(2, 6)));
}

#[test]
fn range_overlap() {
    assert!(is_ranges_overlap(&(1, 5), &(2, 4)));
    assert!(is_ranges_overlap(&(1, 5), &(4, 8)));
    assert!(!is_ranges_overlap(&(1, 5), &(6, 8)));
}
