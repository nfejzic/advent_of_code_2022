use std::{collections::BTreeMap, ops::RangeInclusive, str::FromStr, time::Instant};

use anyhow::{anyhow, Result};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, pos: Point) -> usize {
        self.x.abs_diff(pos.x) + self.y.abs_diff(pos.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("Invalid input for point: {s}"))?;

        Ok(Self {
            x: x_str.replace("x=", "").trim().parse()?,
            y: y_str.replace("y=", "").trim().parse()?,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Beacon {
    pos: Point,
}

impl FromStr for Beacon {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!("Invalid input for beacon: {s}");
        let start = s.find("beacon").ok_or_else(err)?;
        let s = &s[start..];

        let start = s.find("x=").ok_or_else(err)?;
        let s = &s[start..];

        let pos = s.parse()?;

        Ok(Self { pos })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    pos: Point,
    beacon: Beacon,
    distance: usize,
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // input looks like:
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let err = || anyhow!("Invalid input for sensor: {s}");

        let beacon: Beacon = s.parse()?;

        let start = s.find("Sensor at").ok_or_else(err)?;
        let s = &s[start..];

        let start = s.find("x=").ok_or_else(err)?;
        let s = &s[start..];

        let end = s.find(':').ok_or_else(err)?;
        let s = &s[0..end];

        let pos: Point = s.parse()?;

        let distance = pos.manhattan_distance(beacon.pos);

        Ok(Self {
            pos,
            beacon,
            distance,
        })
    }
}

impl Sensor {
    fn reaches_line(&self, line: isize) -> bool {
        let distance = self.pos.manhattan_distance(self.beacon.pos);

        self.pos.y.abs_diff(line) <= distance
    }

    fn distance(&self) -> usize {
        self.pos.manhattan_distance(self.beacon.pos)
    }

    pub(crate) fn reaches_point(&self, p: Point) -> bool {
        self.distance() >= self.pos.manhattan_distance(p)
    }

    pub(crate) fn contains_unreachable(&self, min: Point, max: Point) -> bool {
        let corners = [
            (min.x, min.y),
            (max.x, min.y),
            (max.x, max.y),
            (min.x, max.y),
        ];

        let largest_distance = corners
            .into_iter()
            .map(Point::from)
            .map(|corner| corner.manhattan_distance(self.pos))
            .max()
            .unwrap();

        largest_distance > self.distance
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Bounds {
    start: Point,
    end: Point,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Grid {
    sensors: Vec<Sensor>,
    bounds: Bounds,
    beacons: BTreeMap<isize, Vec<isize>>,
    empty: BTreeMap<isize, Vec<RangeInclusive<isize>>>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sensors: Vec<Sensor> = s.lines().map(|line| line.parse()).collect::<Result<_>>()?;

        let mut beacons: BTreeMap<_, Vec<_>> = BTreeMap::default();

        for sensor in &sensors {
            let point = sensor.beacon.pos;

            beacons.entry(point.y).or_default().push(point.x);
        }

        let mut grid = Self {
            sensors,
            beacons,
            ..Default::default()
        };

        grid.update_bounds();

        Ok(grid)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Bounds { start, end } = self.bounds;

        f.write_str(&format!("{:3}", " "))?;
        for x in start.x..=end.x {
            f.write_str(&format!("{x:3}"))?;
        }
        f.write_str("\n")?;

        for y in start.y..=end.y {
            f.write_str(&format!("{y:3}  "))?;
            for x in start.x..=end.x {
                let p = Point { x, y };
                if self.is_sensor(p) {
                    f.write_str(&format!("{:3}", "S"))?;
                } else if self.is_beacon(p) {
                    f.write_str(&format!("{:3}", "B"))?;
                } else if self.is_empty(p) {
                    f.write_str(&format!("{:3}", "#"))?;
                } else {
                    f.write_str(&format!("{:3}", "."))?;
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Grid {
    fn is_sensor(&self, point: Point) -> bool {
        self.sensors.iter().any(|sensor| sensor.pos == point)
    }

    fn is_beacon(&self, point: Point) -> bool {
        self.sensors.iter().any(|sensor| sensor.beacon.pos == point)
    }

    fn is_empty(&self, point: Point) -> bool {
        match self.empty.get(&point.y) {
            Some(ranges) => ranges.iter().any(|range| range.contains(&point.x)),
            None => false,
        }
    }

    fn calculate(&self, line: isize) -> Vec<RangeInclusive<isize>> {
        let mut ranges: Vec<RangeInclusive<isize>> = Vec::default();

        for sensor in self
            .sensors
            .iter()
            .filter(|sensor| sensor.reaches_line(line))
        {
            // get manhattan distance
            let distance: usize = sensor.pos.manhattan_distance(sensor.beacon.pos);

            let x_distance = distance - sensor.pos.y.abs_diff(line);
            let start_x = sensor.pos.x - x_distance as isize;
            let end_x = sensor.pos.x + x_distance as isize;

            if let Some(positions) = self.beacons.get(&line) {
                if positions.is_empty() {
                    ranges.push(start_x..=end_x);
                } else {
                    let mut start = start_x;
                    for pos in positions {
                        ranges.push(start..=pos - 1);
                        start = pos + 1;
                    }

                    if start <= end_x {
                        ranges.push(start..=end_x);
                    }
                }
            } else {
                ranges.push(start_x..=end_x);
            }
        }

        ranges
    }

    fn calculate_part2(&self, min: isize, max: isize) -> Point {
        // find beacon not reachable by ANY sensor

        let min = Point::new(min, min);
        let max = Point::new(max, max);

        let mut quadrants_to_check = vec![(min, max)];

        while let Some((min, max)) = quadrants_to_check.pop() {
            if min == max {
                if self.sensors.iter().all(|sensor| !sensor.reaches_point(min)) {
                    return min;
                }
            } else {
                let mid = Point::new((min.x + max.x) / 2, (min.y + max.y) / 2);

                let quadrants = [
                    (min, mid),
                    ((mid.x + 1, min.y).into(), (max.x, mid.y).into()),
                    ((min.x, mid.y + 1).into(), (mid.x, max.y).into()),
                    ((mid.x + 1, mid.y + 1).into(), max),
                ];

                for (min, max) in quadrants {
                    if min.x > max.x || min.y > max.y {
                        continue;
                    }

                    if self
                        .sensors
                        .iter()
                        .all(|sensor| sensor.contains_unreachable(min, max))
                    {
                        quadrants_to_check.push((min, max));
                    }
                }
            }
        }

        panic!("This should not be reached");
    }

    fn update_bounds(&mut self) {
        let all_points: Vec<Point> = self
            .sensors
            .iter()
            .flat_map(|sensor| vec![sensor.pos, sensor.beacon.pos])
            .collect();

        let x_points = all_points.iter().map(|point| point.x).chain(
            self.empty
                .values()
                .flatten()
                .flat_map(|range| range.clone()),
        );

        let min_x = x_points.clone().min().unwrap();
        let max_x = x_points.max().unwrap();

        let y_points = all_points
            .iter()
            .map(|point| point.y)
            .chain(self.empty.keys().copied());

        let min_y = y_points.clone().min().unwrap();
        let max_y = y_points.max().unwrap();

        self.bounds = Bounds {
            start: Point { x: min_x, y: min_y },
            end: Point { x: max_x, y: max_y },
        };
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let grid: Grid = input.parse()?;

    println!("Grid bounds = {:#?}", grid.bounds);
    let line = 2000000;

    let time = Instant::now();
    let mut res1 = grid
        .calculate(line)
        .iter()
        .flat_map(|range| range.clone())
        .collect::<Vec<_>>();

    res1.sort();

    res1.dedup();
    let res1 = res1.len();
    let time = time.elapsed();

    println!("Result of part 1 = {res1}");
    println!("Calculated in {}ms", time.as_millis());

    let time = Instant::now();
    let res2 = grid.calculate_part2(0, 4000000);
    let res2 = res2.x * 4000000 + res2.y;
    let time = time.elapsed();

    println!("Result of part 2 = {res2:#?}");
    println!("Calculated in {}ns", time.as_nanos());

    Ok(())
}
