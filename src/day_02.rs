use std::io::BufRead; // lines()

use anyhow::{anyhow, Result};

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn parse(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(anyhow!("invalid strategy: {s:?}")),
        }
    }
}

#[derive(Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn parse(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(anyhow!("invalid choice: {s:?}")),
        }
    }

    fn points(&self) -> u64 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn for_outcome(a: &Self, b: &Outcome) -> Self {
        match (b, a) {
            (Outcome::Win, Self::Rock) => Self::Paper,
            (Outcome::Win, Self::Paper) => Self::Scissors,
            (Outcome::Win, Self::Scissors) => Self::Rock,

            (Outcome::Lose, Self::Rock) => Self::Scissors,
            (Outcome::Lose, Self::Paper) => Self::Rock,
            (Outcome::Lose, Self::Scissors) => Self::Paper,

            (Outcome::Draw, a) => a.clone(),
        }
    }

    fn from_misunderstanding_of_outcome(o: &Outcome) -> Self {
        match o {
            Outcome::Lose => Self::Rock,
            Outcome::Draw => Self::Paper,
            Outcome::Win => Self::Scissors,
        }
    }
}

struct Score {
    a: u64,
    b: u64,
}

impl Score {
    fn add(s1: &Self, s2: &Self) -> Self {
        Score {
            a: s1.a + s2.a,
            b: s1.b + s2.b,
        }
    }

    fn from_choices(a: &Choice, b: &Choice) -> Self {
        let base = Score {
            a: Choice::points(a),
            b: Choice::points(b),
        };
        let bonus = match (a, b) {
            (Choice::Rock, Choice::Rock) => Score { a: 3, b: 3 },
            (Choice::Rock, Choice::Paper) => Score { a: 0, b: 6 },
            (Choice::Rock, Choice::Scissors) => Score { a: 6, b: 0 },

            (Choice::Paper, Choice::Rock) => Score { a: 6, b: 0 },
            (Choice::Paper, Choice::Paper) => Score { a: 3, b: 3 },
            (Choice::Paper, Choice::Scissors) => Score { a: 0, b: 6 },

            (Choice::Scissors, Choice::Rock) => Score { a: 0, b: 6 },
            (Choice::Scissors, Choice::Paper) => Score { a: 6, b: 0 },
            (Choice::Scissors, Choice::Scissors) => Score { a: 3, b: 3 },
        };
        Score::add(&base, &bonus)
    }
}

struct Game {
    a: Choice,
    b: Outcome,
}

impl Game {
    fn parse(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<&str>>()[..] {
            [a, b] => {
                let a = Choice::parse(a)?;
                let b = Outcome::parse(b)?;
                Ok(Self { a, b })
            }
            _ => Err(anyhow!("invalid game: {s:?}")),
        }
    }
}

pub struct Data {
    games: Vec<Game>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let mut games: Vec<Game> = Vec::new();
        for line_result in std::io::BufReader::new(file).lines() {
            let line = line_result?;
            games.push(Game::parse(&line)?);
        }
        Ok(Self { games })
    }

    pub fn solve1(&self) -> Result<u64> {
        Ok(self
            .games
            .iter()
            .map(|Game { a, b }| {
                Score::from_choices(
                    a,
                    &Choice::from_misunderstanding_of_outcome(b),
                )
            })
            .map(|Score { b, .. }| b)
            .sum())
    }

    pub fn solve2(&self) -> Result<u64> {
        Ok(self
            .games
            .iter()
            .map(|Game { a, b }| {
                Score::from_choices(a, &Choice::for_outcome(a, b))
            })
            .map(|Score { b, .. }| b)
            .sum())
    }
}
