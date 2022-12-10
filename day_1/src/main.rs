use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    bag: Vec<usize>,
}

impl Elf {
    fn cal_count(&self) -> usize {
        self.bag.iter().sum()
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let mut iter = input.lines().peekable();

    let mut elves: Vec<Elf> = Vec::new();

    while iter.peek().is_some() {
        let calories: Vec<usize> = iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.parse::<usize>())
            .collect::<Result<_, _>>()?;

        elves.push(Elf { bag: calories })
    }

    elves.sort_by_key(|elf1| elf1.cal_count());

    let count: usize = elves.iter().rev().take(3).map(|elf| elf.cal_count()).sum();
    println!("Total calories by top three elves: {count}");

    Ok(())
}
