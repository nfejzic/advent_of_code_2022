use std::{collections::BTreeSet, str::FromStr};

use anyhow::Result;
use utils::StringError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => return Err(StringError::from(format!("Bad input for direction: {}", s)).into()),
        };

        Ok(dir)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Command {
    direction: Direction,
    count: usize,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((direction, count)) = s.split_once(' ') {
            let direction: Direction = direction.parse()?;
            let count = count.parse()?;

            Ok(Self { direction, count })
        } else {
            Err(StringError::from(format!("Bad input for Command: {s}")).into())
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn move_to(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rope {
    segments: Vec<Position>,
    tail_visits: BTreeSet<Position>,
}

impl Default for Rope {
    fn default() -> Self {
        let head = Default::default();
        let tail = Default::default();
        let tail_visits = [tail].into();

        Self {
            segments: vec![head, tail],
            tail_visits,
        }
    }
}

impl Rope {
    fn new(segments_count: usize) -> Self {
        let segments = vec![Position::default(); segments_count];
        let tail_visits = [segments[segments_count - 1]].into();

        Self {
            segments,
            tail_visits,
        }
    }

    fn apply(&mut self, command: Command) {
        for _ in 0..command.count {
            self.segments[0].move_to(command.direction);

            let num_of_segments = self.segments.windows(2).count();

            for index in 0..num_of_segments {
                if self.segment_len(index) > 1 {
                    let store_position = index == num_of_segments - 1;
                    self.move_segment(index, store_position);
                }
            }
        }
    }

    fn move_segment(&mut self, index: usize, store_position: bool) {
        let head = self.segments[index];
        let tail = &mut self.segments[index + 1];

        let x_move = match tail.x.cmp(&head.x) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let y_move = match tail.y.cmp(&head.y) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        tail.x += x_move;
        tail.y += y_move;

        if store_position {
            self.tail_visits.insert(*tail);
        }
    }

    fn segment_len(&self, index: usize) -> usize {
        let head = self.segments[index];
        let tail = self.segments[index + 1];

        let x_diff = isize::abs(head.x - tail.x);
        let y_diff = isize::abs(head.y - tail.y);

        x_diff.max(y_diff) as usize // guaranteed to fit into usize
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let commands: Vec<_> = input
        .lines()
        .map(Command::from_str)
        .collect::<Result<_>>()?;

    let mut rope = Rope::new(2);

    for command in commands.iter() {
        rope.apply(*command);
    }

    let res = rope.tail_visits.len();
    println!("Tail visited {res} positions.");

    let mut rope = Rope::new(10);

    for command in commands.iter() {
        rope.apply(*command);
    }

    let res = rope.tail_visits.len();
    println!("Tail of rope with 9 segments visited {res} positions.");
    Ok(())
}
