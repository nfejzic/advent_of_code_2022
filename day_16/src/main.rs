use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Valve {
    label: String,
    rate: usize,
    priority: usize,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!("Bad input: {s}");

        let label = s[6..8].to_string();

        let end = s.find(';').ok_or_else(err)?;
        let rate = s[23..end].parse()?;

        let tunnels_start = s[end..].find(char::is_uppercase).ok_or_else(err)?;
        let tunnels = s[end + tunnels_start..]
            .split(',')
            .map(|valve| valve.trim().to_string())
            .collect();

        Ok(Valve {
            label,
            rate,
            priority: 0,
            tunnels,
        })
    }
}

fn calc_prio(valves: &mut BTreeMap<String, Valve>) {
    let mut scores = BTreeMap::default();

    for valve in valves.values() {
        let mut score = 0;
        for tunnel in &valve.tunnels {
            let rate = valves.get(tunnel).map(|valve| valve.rate).unwrap_or(0);
            score += rate;
        }

        scores.insert(valve.label.clone(), score);
    }

    for valve in valves.values_mut() {
        valve.priority = valve.rate + *scores.get(&valve.label).unwrap_or(&0);
    }
}

fn real_val(rate: usize, tick: usize) -> usize {
    rate * tick.saturating_sub(1)
}

fn calc_part1(valves: &BTreeMap<String, Valve>, limit: usize) -> usize {
    let mut curr_valve = valves.get("AA");
    let mut res = 0;

    let mut opened: BTreeSet<String> = BTreeSet::default();

    for tick in (0..limit).rev() {
        if let Some(curr) = curr_valve {
            // if current valve is opened, find best next one
            if opened.contains(&curr.label) {
                let (_, _, label) = curr
                    .tunnels
                    .iter()
                    .filter_map(|tunnel| {
                        valves
                            .get(tunnel)
                            .map(|valve| (valve.rate, valve.priority, tunnel))
                    })
                    .filter(|(_, _, valve)| !opened.contains(*valve))
                    .max()
                    .unwrap();

                println!("move 1: {} -> {}", curr.label, label);
                curr_valve = valves.get(label);
                continue;
            }

            // check if connected valve has higher priority
            if let Some((rate, label)) = curr
                .tunnels
                .iter()
                .filter_map(|tunnel| valves.get(tunnel).map(|valve| (valve.rate, tunnel)))
                .filter(|(_, valve)| !opened.contains(*valve))
                .max()
            {
                if real_val(rate, tick) > real_val(curr.rate, tick) && !opened.contains(label) {
                    println!("move: {} -> {}", curr.label, label);
                    curr_valve = valves.get(label);
                    continue;
                }
            }

            println!("Adding valve: {}", curr.label);
            res += tick.saturating_sub(1) * curr.rate;
            opened.insert(curr.label.clone());
        }
    }

    res
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let mut valves: BTreeMap<String, Valve> = input
        .lines()
        .map(|line| {
            let valve: Valve = line.parse()?;
            Ok((valve.label.clone(), valve))
        })
        .collect::<Result<_>>()?;

    calc_prio(&mut valves);
    let res1 = calc_part1(&valves, 30);
    println!("{res1}");

    Ok(())
}
