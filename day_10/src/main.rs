use std::{collections::VecDeque, str::FromStr};

use utils::StringError;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    AddX(isize),
    Noop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let "noop" = s {
            return Ok(Self::Noop);
        }

        let (instruction, value) = s
            .split_once(' ')
            .unwrap_or_else(|| panic!("Bad input: {}", s));

        let value: Option<isize> = match value {
            "" => None,
            num => Some(num.parse()?),
        };

        match instruction {
            "addx" => Ok(Self::AddX(value.unwrap_or_else(|| {
                panic!(
                    "addx instruction must be provided with a value... input: '{}'",
                    s
                )
            }))),

            "noop" => Ok(Self::Noop),
            _ => Err(StringError::from(format!("Bad input: {}", s)).into()),
        }
    }
}

trait CycleLen {
    fn cycle_len(&self) -> usize;
}

impl CycleLen for Instruction {
    fn cycle_len(&self) -> usize {
        use Instruction::*;
        match self {
            AddX(_) => 2,
            Noop => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ExecutingInstruction {
    instruction: Instruction,
    cycles_left: usize,
}

impl From<Instruction> for ExecutingInstruction {
    fn from(instruction: Instruction) -> Self {
        Self {
            instruction,
            cycles_left: instruction.cycle_len(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Registers {
    x: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cpu {
    registers: Registers,
    cycle: usize,
    executing: Option<ExecutingInstruction>,
    instructions: VecDeque<Instruction>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            registers: Registers { x: 1 },
            cycle: 1,
            executing: None,
            instructions: VecDeque::default(),
        }
    }

    fn load(&mut self, instructions: &[Instruction]) {
        self.instructions.extend(instructions.iter());
    }

    fn value_at(&mut self, cycle: usize) -> Result<isize> {
        while self.cycle < cycle {
            self.execute()?;
        }

        Ok(self.registers.x)
    }

    fn execute(&mut self) -> Result<()> {
        self.cycle += 1;

        if self.executing.is_none() {
            self.executing = match self.instructions.pop_front() {
                Some(instr) => Some(ExecutingInstruction::from(instr)),
                None => return Err(StringError::from("No instructions left to execute.").into()),
            }
        }

        if let Some(ref mut exec) = self.executing {
            exec.cycles_left -= 1;

            if exec.cycles_left == 0 {
                let instr = exec.instruction;
                self.apply(instr);
                self.executing = None;
            }
        }

        Ok(())
    }

    fn apply(&mut self, instr: Instruction) {
        match instr {
            Instruction::AddX(value) => self.registers.x += value,
            Instruction::Noop => {}
        }
    }

    fn is_empty(&self) -> bool {
        self.executing.is_none() && self.instructions.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Crt {
    cpu: Cpu,
    line: usize,
}

impl Crt {
    fn new(cpu: Cpu) -> Self {
        Self { cpu, line: 0 }
    }

    fn load(&mut self, instructions: &[Instruction]) {
        self.cpu.load(instructions)
    }

    fn draw(&mut self) -> Result<()> {
        while !self.cpu.is_empty() {
            let res = self.draw_line();
            println!();

            if res.is_err() {
                break;
            }
        }

        Ok(())
    }

    fn draw_line(&mut self) -> Result<()> {
        for pixel in 0..40 {
            let cycle = self.line * 40 + pixel + 1;
            let x = self.cpu.value_at(cycle)?;
            let range = (x - 1)..=(x + 1);

            let pixel = match range.contains(&(pixel as isize)) {
                true => "#",
                false => ".",
            };

            print!("{pixel}");
        }

        self.line += 1;
        Ok(())
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let instructions: Vec<_> = input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<_>>()?;

    let mut cpu = Cpu::new();
    cpu.load(&instructions);

    let cycles = [20, 60, 100, 140, 180, 220];

    let mut res = 0;
    for cycle in cycles {
        let val = cpu.value_at(cycle)?;
        res += val * cycle as isize;
    }

    println!("Res = {res}");

    let mut crt = Crt::new(Cpu::new());
    crt.load(&instructions);
    crt.draw()?;

    Ok(())
}
