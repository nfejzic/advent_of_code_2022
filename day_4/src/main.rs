use std::{ops::RangeInclusive, str::FromStr};

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct Task {
    range: RangeInclusive<usize>,
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds: Vec<usize> = s
            .split('-')
            .map(|s| s.parse::<usize>())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            range: bounds[0]..=bounds[1],
        })
    }
}

impl Task {
    fn contains_fully(&self, other: &Task) -> bool {
        self.range.contains(other.range.start()) && self.range.contains(other.range.end())
    }

    fn contains_partially(&self, other: &Task) -> bool {
        self.range.contains(other.range.start()) || self.range.contains(other.range.end())
    }

    fn full_overlap_with(&self, other: &Task) -> bool {
        self.contains_fully(other) || other.contains_fully(self)
    }

    fn partial_overlap_with(&self, other: &Task) -> bool {
        self.contains_partially(other) || other.contains_partially(self)
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let tasks = input
        .lines()
        .flat_map(|line| line.split(','))
        .map(Task::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let result1 = tasks
        .chunks(2)
        .filter(|&tasks| tasks[0].full_overlap_with(&tasks[1]))
        .count();

    let result2 = tasks
        .chunks(2)
        .filter(|tasks| tasks[0].partial_overlap_with(&tasks[1]))
        .count();

    println!("{result1}");
    println!("{result2}");

    Ok(())
}
