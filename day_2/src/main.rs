use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;
use utils::StringError;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Choice::*;
        let ord = match (*self, *other) {
            (Rock, Scissors) => Ordering::Greater,
            (Paper, Rock) => Ordering::Greater,
            (Scissors, Paper) => Ordering::Greater,
            (first, second) if first == second => Ordering::Equal,
            (_, _) => Ordering::Less,
        };

        Some(ord)
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Choice::*;
        match (*self, *other) {
            (Rock, Scissors) => Ordering::Greater,
            (Paper, Rock) => Ordering::Greater,
            (Scissors, Paper) => Ordering::Greater,
            (first, second) if first == second => Ordering::Equal,
            (_, _) => Ordering::Less,
        }
    }
}

impl FromStr for Choice {
    type Err = StringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Choice::*;
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(StringError::from("Invalid input!")),
        }
    }
}

impl Choice {
    fn parse_line(input: &str) -> Result<(Self, Self)> {
        input
            .split(' ')
            .collect::<Vec<_>>()
            .as_slice()
            .chunks(2)
            .next()
            .map(|chunk| {
                // first strategy
                // let choice1 = chunk[0].parse()?;
                // let choice2 = chunk[1].parse()?;
                //
                // Ok((choice1, choice2))

                // second strategy
                let choice1: Choice = chunk[0].parse()?;
                let choice2 = Choice::choose_by_second_strat(&choice1, chunk[1])?;

                Ok((choice1, choice2))
            })
            .ok_or_else(|| StringError::from(format!("Invalid choice input: {input}")))?
    }

    fn choose_by_second_strat(their: &Choice, input: &str) -> Result<Self> {
        let choice = match input {
            "X" => Choice::choose_to_lose(their),
            "Y" => *their,
            "Z" => Choice::choose_to_defeat(their),
            _ => return Err(StringError::from("Invalid input provided").into()),
        };

        Ok(choice)
    }

    fn choose_to_defeat(their: &Choice) -> Choice {
        use Choice::*;
        match their {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn choose_to_lose(their: &Choice) -> Choice {
        use Choice::*;
        match their {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn calculate_score(&self, their: &Choice) -> usize {
        let outcome = match self.cmp(&their) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        outcome + self.value()
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let choices = input
        .lines()
        .map(Choice::parse_line)
        .collect::<Result<Vec<_>, _>>()?;

    let res: usize = choices
        .iter()
        .map(|(their, mine)| mine.calculate_score(their))
        .sum();

    println!("My score = {res}");

    Ok(())
}
