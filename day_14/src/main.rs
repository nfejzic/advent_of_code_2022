use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    ops::RangeInclusive,
    str::FromStr,
    time::Duration,
};

use anyhow::Result;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (column, row) = s
            .trim()
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Invalid input: {s}"))?;

        let p = Point {
            x: column.parse()?,
            y: row.parse()?,
        };

        Ok(p)
    }
}

#[derive(Debug, Clone)]
struct Cave {
    columns: BTreeMap<usize, Vec<RangeInclusive<usize>>>,
    rows: BTreeMap<usize, Vec<RangeInclusive<usize>>>,
    sand_cells: BTreeSet<Point>,
    sand: Sand,
    bounds: Bounds,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Bounds {
    start_row: usize,
    end_row: usize,
    start_col: usize,
    end_col: usize,
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave = Cave::new();

        for line in s.lines() {
            let points: Vec<Point> = line
                .split("->")
                .map(|point_str| point_str.parse())
                .collect::<Result<_>>()?;

            for points in points.windows(2) {
                if let &[point1, point2, ..] = points {
                    cave.set_rock(point1, point2);
                }
            }
        }

        cave.calc_bounds();

        Ok(cave)
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Bounds {
            start_row,
            end_row,
            start_col,
            end_col,
        } = self.bounds;

        for y in start_row..=(end_row / 3) {
            for x in start_col..=end_col {
                let p = Point { x, y };

                if x == 500 && y == 0 {
                    f.write_str("|")?;
                } else if self.is_rock(&p) {
                    f.write_str("#")?;
                } else if self.is_sand(&p) {
                    f.write_str("o")?;
                } else if p == self.sand.pos {
                    f.write_str("+")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Cave {
    fn new() -> Self {
        Self {
            columns: Default::default(),
            rows: Default::default(),
            sand_cells: Default::default(),
            sand: Default::default(),
            bounds: Default::default(),
        }
    }

    fn calc_bounds(&mut self) {
        let used_cols = self
            .rows
            .values()
            .flat_map(|ranges| ranges.iter().flat_map(|range| range.clone()))
            .chain(self.columns.keys().cloned());

        let start_col = used_cols
            .clone()
            .min()
            .expect("No rows occupied, can't render cave...");

        let end_col = used_cols
            .max()
            .expect("No rows occupied, can't render cave...");

        let end_row = self
            .columns
            .values()
            .flat_map(|ranges| ranges.iter().flat_map(|range| range.clone()))
            .chain(self.rows.keys().cloned())
            .max()
            .expect("No rows occupied, can't render cave...");

        let start_row = 0;

        self.bounds = Bounds {
            start_row,
            end_row,
            start_col,
            end_col,
        }
    }

    fn set_rock(&mut self, point1: Point, point2: Point) {
        if point1.x == point2.x {
            // same column
            let y_start = point1.y.min(point2.y);
            let y_end = point1.y.max(point2.y);
            let range = y_start..=y_end;

            self.columns.entry(point1.x).or_default().push(range);
        } else {
            // same row
            let x_start = point1.x.min(point2.x);
            let x_end = point1.x.max(point2.x);
            let range = x_start..=x_end;

            self.rows.entry(point1.y).or_default().push(range);
        }
    }

    fn is_rock(&self, point: &Point) -> bool {
        let row = point.y;
        let column = point.x;

        // Part 2
        if row == self.bounds.end_row + 2 {
            return true;
        }

        let mut contained = false;
        if let Some(ranges) = self.rows.get(&row) {
            contained |= ranges.iter().any(|range| range.contains(&point.x));
        }

        if let Some(ranges) = self.columns.get(&column) {
            contained |= ranges.iter().any(|range| range.contains(&point.y));
        }

        contained
    }

    fn set_sand(&mut self, point: Point) {
        self.sand_cells.insert(point);
    }

    fn is_sand(&self, point: &Point) -> bool {
        self.sand_cells.contains(point)
    }

    fn is_air(&self, point: &Point) -> bool {
        !self.is_sand(point) && !self.is_rock(point)
    }

    fn out_of_bounds(&self, point: Point) -> bool {
        let Bounds {
            start_col,
            end_col,
            end_row,
            ..
        } = self.bounds;

        let _part1 = point.x < start_col || point.x > end_col || point.y > end_row;

        // Part 2
        point.y > end_row + 2
    }

    fn simulate(&mut self) {
        self.sand.reset();

        loop {
            // print!("{esc}c", esc = 27 as char);
            if let Some(point) = self.sand.can_fall(self) {
                self.sand.fall_to(point);
                // println!("{:?}", self.sand.pos);
                println!("{:?}", self.sand.pos);
                // print!("{self}");

                if self.out_of_bounds(self.sand.pos) {
                    // escaped bounds, will keep going forever
                    break;
                }
            } else {
                self.set_sand(self.sand.pos);
                if self.sand.pos.y == 0 {
                    // part 2
                    break;
                }
                self.sand.reset();
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Sand {
    pos: Point,
}

impl Sand {
    fn can_fall(&self, cave: &Cave) -> Option<Point> {
        let Point { x, mut y } = self.pos;
        y += 1;

        if cave.is_air(&Point { x, y }) {
            Some(Point { x, y })
        } else if cave.is_air(&Point { x: x - 1, y }) {
            Some(Point { x: x - 1, y })
        } else if cave.is_air(&Point { x: x + 1, y }) {
            Some(Point { x: x + 1, y })
        } else {
            None
        }
    }

    fn fall_to(&mut self, point: Point) {
        self.pos = point
    }

    fn reset(&mut self) {
        self.pos = Point { x: 500, y: 0 };
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let mut cave: Cave = input.parse()?;
    println!("{:?}", cave.bounds);
    println!("{cave}");
    cave.simulate();

    let res = cave.sand_cells.len();

    println!("{res} units of sand came to rest! (Part 1)");

    Ok(())
}
