use std::cmp::Ordering;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
    Term(u8),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;

        match (self, other) {
            (Term(lhs), Term(rhs)) => lhs.cmp(rhs),
            (List(lhs), List(rhs)) => lhs.cmp(rhs),
            (Term(lhs), List(rhs)) => [Term(*lhs)][..].cmp(rhs),
            (List(lhs), Term(rhs)) => lhs.as_slice().cmp(&[Term(*rhs)][..]),
        }
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let mut items = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect::<Vec<Packet>>();

    let res1: usize = items
        .chunks(2)
        .enumerate()
        .filter(|(_, window)| window[0] < window[1])
        .map(|(idx, _)| idx + 1)
        .sum();

    let divider_2: Packet = serde_json::from_str("[[2]]")?;
    let divider_6: Packet = serde_json::from_str("[[6]]")?;

    items.extend_from_slice(&[divider_2.clone(), divider_6.clone()]);

    items.sort();

    let res2: usize = items
        .iter()
        .enumerate()
        .filter(|(_, item)| *item == &divider_2 || *item == &divider_6)
        .map(|(idx, _)| idx + 1)
        .product();

    println!("Result of part 1 = {res1}");
    println!("Result of part 2 = {res2}");

    Ok(())
}
