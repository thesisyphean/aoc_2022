use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Strategy {
    Rock,
    Paper,
    Scissors,
}

impl Strategy {
    fn score(&self) -> u32 {
        match self {
            Strategy::Rock => 1,
            Strategy::Paper => 2,
            Strategy::Scissors => 3,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Strategy::Rock => Strategy::Scissors,
            Strategy::Paper => Strategy::Rock,
            Strategy::Scissors => Strategy::Paper,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Strategy::Rock => Strategy::Paper,
            Strategy::Paper => Strategy::Scissors,
            Strategy::Scissors => Strategy::Rock,
        }
    }
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Strategy::Rock,
            "B" | "Y" => Strategy::Paper,
            "C" | "Z" => Strategy::Scissors,
            _ => panic!("{} is an invalid strategy!", s),
        })
    }
}

struct Round(Strategy, Strategy);

impl Round {
    fn score(&self) -> u32 {
        self.1.score() +

        if self.1.wins_against() == self.0 {
            6
        } else if self.1 == self.0 {
            3
        } else {
            0
        }
    }

    fn decide_play(strat: Strategy, result: &str) -> Self {
        Round(strat, match result {
            "X" => strat.wins_against(),
            "Y" => strat,
            "Z" => strat.loses_against(),
            _ => panic!("{} is not a valid result!", result),
        })
    }
}

fn parse_rounds(input: &[&str], decide: bool) -> Vec<Round> {
    let mut rounds = vec![];

    for line in input {
        let parts: Vec<_> = line.split_whitespace()
            .collect();

        if !decide {
            rounds.push(Round(parts[0].parse().unwrap(),
                parts[1].parse().unwrap()));
        } else {
            rounds.push(Round::decide_play(parts[0].parse().unwrap(),
                parts[1]));
        }
    }

    rounds
}

pub fn solve_1(input: &[&str]) -> u32 {
    parse_rounds(input, false).iter()
        .map(|r| r.score())
        .sum()
}

pub fn solve_2(input: &[&str]) -> u32 {
    parse_rounds(input, true).iter()
        .map(|r| r.score())
        .sum()
}