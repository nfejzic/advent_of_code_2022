use std::collections::{HashMap, HashSet};

use anyhow::Result;

fn find_score(input: &str) -> usize {
    let first_half = input.chars().take(input.len() / 2);
    let mut second_half = input.chars().skip(input.len() / 2);

    let mut found: HashSet<char> = HashSet::with_capacity(input.len());

    for c in first_half {
        found.insert(c);
    }

    second_half
        .find(|c| found.contains(c))
        .map(|c| c.score())
        .unwrap_or(0)
}

fn find_common(input: &[&str]) -> usize {
    let mut common: HashMap<char, usize> =
        HashMap::with_capacity(input.iter().map(|line| line.len()).max().unwrap_or(0));

    for (line_num, line) in input.iter().enumerate() {
        let line_num = line_num + 1;
        for c in line.chars() {
            let entry = common.entry(c).or_insert(0);

            if *entry == line_num - 1 {
                *entry = line_num;
            }
        }
    }

    let common = common.iter().find(|(_, val)| **val == 3);
    let common = common.iter().find(|(_, val)| **val == 3);

    common.map(|(c, _)| c.score()).unwrap_or(0)
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let result: usize = input.lines().map(find_score).sum();
    println!("{result}");

    let result: usize = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(find_common)
        .sum();
    println!("{result}");

    Ok(())
}

trait CharScore {
    fn score(&self) -> usize;
}

impl CharScore for char {
    fn score(&self) -> usize {
        if self.is_uppercase() {
            (*self as usize) - 64 + 26
        } else {
            (*self as usize) - 96
        }
    }
}
