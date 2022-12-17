use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use anyhow::Result;
use utils::StringError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Num {
    Identity,
    Value(usize),
}

impl FromStr for Num {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "old" => Self::Identity,
            num => Self::Value(num.parse()?),
        };

        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Operation {
    kind: OperationKind,
    first: Num,
    second: Num,
}

impl Operation {
    fn apply(&self, input: usize) -> usize {
        let first = match self.first {
            Num::Identity => input,
            Num::Value(val) => val,
        };

        let second = match self.second {
            Num::Identity => input,
            Num::Value(val) => val,
        };

        self.kind.apply(first, second)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Test {
    divisor: usize,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn get_monkey_idx(&self, input: usize) -> usize {
        match input % self.divisor == 0 {
            true => self.if_true,
            false => self.if_false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

struct MonkeyBuilder {
    items: VecDeque<usize>,
    operation: Option<Operation>,
    test: Option<Test>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum OperationKind {
    Add,
    Mul,
}

impl FromStr for OperationKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OperationKind::*;
        let op = match s {
            "+" => Add,
            "*" => Mul,
            _ => return Err(StringError::from(format!("Invalid operation: {s}")).into()),
        };

        Ok(op)
    }
}

impl OperationKind {
    fn apply<T>(&self, a: T, b: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Debug,
    {
        match self {
            OperationKind::Add => a + b,
            OperationKind::Mul => a * b,
        }
    }
}

impl MonkeyBuilder {
    fn new() -> Self {
        Self {
            items: VecDeque::default(),
            operation: None,
            test: None,
        }
    }

    fn items(&mut self, items: &[usize]) {
        self.items.clear();
        self.items.extend(items.iter());
    }

    fn operation(&mut self, op: Operation) {
        self.operation = Some(op);
    }

    fn test(&mut self, test: Test) {
        self.test = Some(test);
    }

    fn build(self) -> Result<Monkey> {
        let items = self.items.clone();

        let operation = self
            .operation
            .ok_or_else(|| StringError::from("Missing operation!"))?;

        let test = self
            .test
            .ok_or_else(|| StringError::from("Missing test!"))?;

        Ok(Monkey {
            items,
            operation,
            test,
            inspections: 0,
        })
    }
}

impl Default for MonkeyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut builder = MonkeyBuilder::new();
        let mut lines = s.lines();

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.trim().starts_with("Starting items: ") {
                let items = line
                    .replace("Starting items: ", "")
                    .split(',')
                    .map(|s| s.trim().parse())
                    .collect::<Result<Vec<usize>, _>>()?;

                builder.items(&items);
            }

            if line.contains("Operation: ") {
                let line = line.trim().replace("Operation: new = ", "");
                let parts: Vec<&str> = line.split(' ').collect();

                let op: OperationKind = parts[1].parse()?;

                let first = parts[0].parse()?;
                let second = parts[2].parse()?;

                let operation = Operation {
                    kind: op,
                    first,
                    second,
                };

                builder.operation(operation);
            }

            if line.trim().starts_with("Test:") {
                let line = line.replace("Test: divisible by ", "");
                let divisor = line.trim().parse::<usize>()?;

                let if_true = lines
                    .next()
                    .expect("Missing outcomes for monkey.")
                    .rsplit(' ')
                    .next()
                    .expect("Missing outcome for monkey")
                    .parse::<usize>()?;

                let if_false = lines
                    .next()
                    .expect("Missing outcomes for monkey.")
                    .rsplit(' ')
                    .next()
                    .expect("Missing outcome for monkey")
                    .parse::<usize>()?;

                let test = Test {
                    divisor,
                    if_true,
                    if_false,
                };

                builder.test(test);
            }
        }

        builder.build()
    }
}

#[derive(Debug)]
struct Game {
    monkeys: Vec<Monkey>,
    divisor: usize,
}

impl Game {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let divisor = monkeys.iter().map(|monkey| monkey.test.divisor).product();

        Self { monkeys, divisor }
    }

    fn simulate(&mut self, count: usize) {
        for _ in 0..count {
            for monkey in 0..self.monkeys.len() {
                self.simulate_monkey(monkey);
            }
        }
    }

    fn simulate_monkey(&mut self, monkey_idx: usize) {
        let monkey = &mut self.monkeys[monkey_idx];
        let mut send = Vec::default();

        while let Some(item) = monkey.items.pop_front() {
            monkey.inspections += 1;
            let worry = monkey.operation.apply(item);
            let to_monkey = monkey.test.get_monkey_idx(worry);

            let worry = worry % self.divisor;

            send.push((to_monkey, worry));
        }

        for (to_monkey, item) in send {
            self.monkeys[to_monkey].items.push_back(item);
        }
    }

    fn most_active(&self, count: usize) -> Vec<usize> {
        let mut inspections: Vec<usize> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .collect();

        inspections.sort();
        inspections.iter().rev().take(count).copied().collect()
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;
    let mut lines = input.lines().peekable();

    let mut monkeys = Vec::default();
    while lines.peek().is_some() {
        let input: String = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| String::from(line) + "\n")
            .collect();

        monkeys.push(Monkey::from_str(&input)?);
    }

    let mut game = Game::new(monkeys);
    game.simulate(10_000); // part 2: 10 000 rounds
    let is: Vec<usize> = game
        .monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect();

    println!("{is:?}");

    let level: usize = game.most_active(2).iter().product();
    // let level = game.most_active(2);

    println!("{level:#?}");

    Ok(())
}
