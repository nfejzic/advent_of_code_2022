use std::{
    collections::{BTreeMap, VecDeque},
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Stack {
    letters: VecDeque<char>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Stacks {
    stacks: BTreeMap<usize, Stack>,
}

impl Stacks {
    fn apply_movement(&mut self, Movement { from, to, how_many }: Movement) {
        for _ in 0..how_many {
            let package = self
                .stacks
                .get_mut(&from)
                .unwrap()
                .letters
                .pop_back()
                .unwrap();

            self.stacks.get_mut(&to).unwrap().letters.push_back(package);
        }
    }

    fn apply_movement_9001(&mut self, Movement { from, to, how_many }: Movement) {
        let start_index = self.stacks.get(&from).unwrap().letters.len() - how_many;

        for package in self
            .stacks
            .get_mut(&from)
            .unwrap()
            .letters
            .drain(start_index..)
            .collect::<Vec<_>>()
        {
            self.stacks.get_mut(&to).unwrap().letters.push_back(package);
        }
    }

    fn print_crates(&self) {
        for stack in self.stacks.values() {
            if let Some(letter) = stack.letters.back() {
                print!("{}", letter);
            }
        }

        println!();
    }
}

impl FromStr for Stacks {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = input.lines().take_while(|line| !line.is_empty()).collect();

        let err = "Malformed input.";
        let _stacks = lines
            .last()
            .ok_or(err)?
            .split(char::is_whitespace)
            .filter(|input| !input.is_empty())
            .count();

        let mut stacks: BTreeMap<usize, Stack> = BTreeMap::default();

        for line in lines.iter().rev().skip(1) {
            for (n, symbol) in line.chars().enumerate() {
                // characters are found on position 4 * n - 3
                if symbol.is_alphabetic() {
                    let index = (n + 3) / 4;
                    stacks
                        .entry(index)
                        .or_insert_with(Stack::default)
                        .letters
                        .push_back(symbol);
                }
            }
        }

        Ok(Stacks { stacks })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Movement {
    from: usize,
    to: usize,
    how_many: usize,
}

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input
            .split(' ')
            .filter_map(|word| word.parse::<usize>().ok());

        let err = "Malformed input";
        let how_many = input.next().ok_or(err)?;
        let from = input.next().ok_or(err)?;
        let to = input.next().ok_or(err)?;

        Ok(Movement { from, to, how_many })
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Movements(Vec<Movement>);

impl Deref for Movements {
    type Target = Vec<Movement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Movements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Movement>> for Movements {
    fn from(input: Vec<Movement>) -> Self {
        Self(input)
    }
}

impl FromStr for Movements {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let movements = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(Movement::from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>()
            .into();

        Ok(movements)
    }
}

fn read_file() -> Result<String, String> {
    let mut args = std::env::args();

    let file_name = args
        .nth(1)
        .ok_or_else(|| String::from("Please provide file path."))?;

    std::fs::read_to_string(&file_name).map_err(|_| format!("Could not open file {file_name}"))
}

fn main() -> Result<(), String> {
    let input = read_file()?;

    let mut stacks: Stacks = input.parse()?;
    let movements: Movements = input.parse()?;

    let mut stacks_first = stacks.clone();
    for movement in movements.iter() {
        stacks_first.apply_movement(*movement);
    }

    print!("Crate mover 9000: ");
    stacks_first.print_crates();

    for movement in movements.iter() {
        stacks.apply_movement_9001(*movement);
    }

    print!("Crate mover 9001: ");
    stacks.print_crates();
    Ok(())
}
