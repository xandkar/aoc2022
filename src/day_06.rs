use std::collections::HashSet;

use anyhow::{anyhow, Result};

pub struct Data {
    signal: String,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let signal = std::fs::read_to_string(input)?.trim().to_string();
        Ok(Self { signal })
    }

    pub fn part1(&self) -> Result<usize> {
        marker(&self.signal, 4).ok_or_else(|| anyhow!("no solution found"))
    }

    pub fn part2(&self) -> Result<usize> {
        marker(&self.signal, 14).ok_or_else(|| anyhow!("no solution found"))
    }
}

fn marker(s: &str, l: usize) -> Option<usize> {
    for (i, _) in s.chars().enumerate() {
        let n = i + l;
        if s[i..n].chars().collect::<HashSet<char>>().len() == l {
            return Some(n);
        }
    }
    None
}

#[test]
fn marker_test_1() {
    assert_eq!(Some(7), marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
    assert_eq!(Some(19), marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
}

#[test]
fn marker_test_2() {
    assert_eq!(Some(5), marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(Some(23), marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
}

#[test]
fn marker_test_3() {
    assert_eq!(Some(6), marker("nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(Some(23), marker("nppdvjthqldpwncqszvftbrmjlhg", 14));
}

#[test]
fn marker_test_4() {
    assert_eq!(Some(10), marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(Some(29), marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
}

#[test]
fn marker_test_5() {
    assert_eq!(Some(11), marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    assert_eq!(Some(26), marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
}
